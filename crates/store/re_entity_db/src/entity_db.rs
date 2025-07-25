use std::sync::Arc;

use nohash_hasher::IntMap;

use re_chunk::{
    Chunk, ChunkBuilder, ChunkId, ChunkResult, LatestAtQuery, RowId, TimeInt, TimePoint, Timeline,
    TimelineName,
};
use re_chunk_store::{
    ChunkStore, ChunkStoreChunkStats, ChunkStoreConfig, ChunkStoreDiffKind, ChunkStoreEvent,
    ChunkStoreHandle, ChunkStoreSubscriber as _, GarbageCollectionOptions, GarbageCollectionTarget,
};
use re_log_types::{
    ApplicationId, EntityPath, EntityPathHash, LogMsg, RecordingId, ResolvedTimeRange,
    ResolvedTimeRangeF, SetStoreInfo, StoreId, StoreInfo, StoreKind, TimeType,
};
use re_query::{
    QueryCache, QueryCacheHandle, StorageEngine, StorageEngineArcReadGuard, StorageEngineReadGuard,
    StorageEngineWriteGuard,
};
use re_smart_channel::SmartChannelSource;

use crate::{Error, TimesPerTimeline, ingestion_statistics::IngestionStatistics};

// ----------------------------------------------------------------------------

/// See [`GarbageCollectionOptions::time_budget`].
pub const DEFAULT_GC_TIME_BUDGET: std::time::Duration = std::time::Duration::from_micros(3500); // empirical

// ----------------------------------------------------------------------------¨

/// What class of [`EntityDb`] is this?
///
/// The class is used to semantically group recordings in the UI (e.g. in the recording panel) and
/// to determine how to source the default blueprint. For example, `DatasetPartition` dbs might have
/// their default blueprint sourced remotely.
pub enum EntityDbClass<'a> {
    /// This is a regular local recording (e.g. loaded from a `.rrd` file or logged to the viewer).
    LocalRecording,

    /// This is an official rerun example recording.
    ExampleRecording,

    /// This is a recording loaded from a remote dataset partition.
    DatasetPartition(&'a re_uri::DatasetDataUri),

    /// This is a blueprint.
    Blueprint,
}

// ---

/// An in-memory database built from a stream of [`LogMsg`]es.
///
/// NOTE: all mutation is to be done via public functions!
#[derive(Clone)] // Useful for tests
pub struct EntityDb {
    /// Store id associated with this [`EntityDb`]. Must be identical to the `storage_engine`'s
    /// store id.
    store_id: StoreId,

    /// Set by whomever created this [`EntityDb`].
    ///
    /// Clones of an [`EntityDb`] gets a `None` source.
    pub data_source: Option<re_smart_channel::SmartChannelSource>,

    /// Comes in a special message, [`LogMsg::SetStoreInfo`].
    set_store_info: Option<SetStoreInfo>,

    /// Keeps track of the last time data was inserted into this store (viewer wall-clock).
    last_modified_at: web_time::Instant,

    /// The highest `RowId` in the store,
    /// which corresponds to the last edit time.
    /// Ignores deletions.
    latest_row_id: Option<RowId>,

    /// In many places we just store the hashes, so we need a way to translate back.
    entity_path_from_hash: IntMap<EntityPathHash, EntityPath>,

    /// The global-scope time tracker.
    ///
    /// For each timeline, keeps track of what times exist, recursively across all
    /// entities/components.
    ///
    /// Used for time control.
    ///
    /// TODO(#7084): Get rid of [`TimesPerTimeline`] and implement time-stepping with [`crate::TimeHistogram`] instead.
    times_per_timeline: TimesPerTimeline,

    /// A time histogram of all entities, for every timeline.
    time_histogram_per_timeline: crate::TimeHistogramPerTimeline,

    /// A tree-view (split on path components) of the entities.
    tree: crate::EntityTree,

    /// The [`StorageEngine`] that backs this [`EntityDb`].
    ///
    /// This object and all its internal fields are **never** allowed to be publicly exposed,
    /// whether that is directly or through methods, _even if that's just shared references_.
    ///
    /// The only way to get access to the [`StorageEngine`] from the outside is to use
    /// [`EntityDb::storage_engine`], which returns a read-only guard.
    /// The design statically guarantees the absence of deadlocks and race conditions that normally
    /// results from letting store and cache handles arbitrarily loose all across the codebase.
    storage_engine: StorageEngine,

    stats: IngestionStatistics,
}

impl EntityDb {
    pub fn new(store_id: StoreId) -> Self {
        Self::with_store_config(store_id, ChunkStoreConfig::from_env().unwrap_or_default())
    }

    pub fn with_store_config(store_id: StoreId, store_config: ChunkStoreConfig) -> Self {
        let store = ChunkStoreHandle::new(ChunkStore::new(store_id.clone(), store_config));
        let cache = QueryCacheHandle::new(QueryCache::new(store.clone()));

        // Safety: these handles are never going to be leaked outside of the `EntityDb`.
        #[allow(unsafe_code)]
        let storage_engine = unsafe { StorageEngine::new(store, cache) };

        Self {
            store_id,
            data_source: None,
            set_store_info: None,
            last_modified_at: web_time::Instant::now(),
            latest_row_id: None,
            entity_path_from_hash: Default::default(),
            times_per_timeline: Default::default(),
            tree: crate::EntityTree::root(),
            time_histogram_per_timeline: Default::default(),
            storage_engine,
            stats: IngestionStatistics::default(),
        }
    }

    #[inline]
    pub fn tree(&self) -> &crate::EntityTree {
        &self.tree
    }

    /// Returns a read-only guard to the backing [`StorageEngine`].
    #[inline]
    pub fn storage_engine(&self) -> StorageEngineReadGuard<'_> {
        self.storage_engine.read()
    }

    /// Returns a reference to the backing [`StorageEngine`].
    ///
    /// This can be used to obtain a clone of the [`StorageEngine`].
    ///
    /// # Safety
    ///
    /// Trying to lock the [`StorageEngine`] (whether read or write) while the computation of a viewer's
    /// frame is already in progress will lead to data inconsistencies, livelocks and deadlocks.
    /// The viewer runs a synchronous work-stealing scheduler (`rayon`) as well as an asynchronous
    /// one (`tokio`): when and where locks are taken is entirely non-deterministic (even unwanted reentrancy
    /// is a possibility).
    ///
    /// Don't use this unless you know what you're doing. Use [`Self::storage_engine`] instead.
    #[expect(unsafe_code)]
    pub unsafe fn storage_engine_raw(&self) -> &StorageEngine {
        &self.storage_engine
    }

    /// Returns a read-only guard to the backing [`StorageEngine`].
    ///
    /// That guard can be cloned at will and has a static lifetime.
    ///
    /// It is not possible to insert any more data in this [`EntityDb`] until the returned guard,
    /// and any clones, have been dropped.
    #[inline]
    pub fn storage_engine_arc(&self) -> StorageEngineArcReadGuard {
        self.storage_engine.read_arc()
    }

    #[inline]
    pub fn store_info_msg(&self) -> Option<&SetStoreInfo> {
        self.set_store_info.as_ref()
    }

    #[inline]
    pub fn store_info(&self) -> Option<&StoreInfo> {
        self.store_info_msg().map(|msg| &msg.info)
    }

    #[inline]
    pub fn application_id(&self) -> &ApplicationId {
        self.store_id().application_id()
    }

    #[inline]
    pub fn recording_id(&self) -> &RecordingId {
        self.store_id().recording_id()
    }

    #[inline]
    pub fn store_kind(&self) -> StoreKind {
        self.store_id().kind()
    }

    #[inline]
    pub fn store_id(&self) -> &StoreId {
        &self.store_id
    }

    /// Returns the [`EntityDbClass`] of this entity db.
    pub fn store_class(&self) -> EntityDbClass<'_> {
        match self.store_kind() {
            StoreKind::Blueprint => EntityDbClass::Blueprint,

            StoreKind::Recording => match &self.data_source {
                Some(SmartChannelSource::RrdHttpStream { url, .. })
                    if url.starts_with("https://app.rerun.io") =>
                {
                    EntityDbClass::ExampleRecording
                }

                Some(SmartChannelSource::RedapGrpcStream { uri, .. }) => {
                    EntityDbClass::DatasetPartition(uri)
                }

                _ => EntityDbClass::LocalRecording,
            },
        }
    }

    /// Read one of the built-in `RecordingInfo` properties.
    pub fn recording_info_property<C: re_types_core::Component>(
        &self,
        component_descr: &re_types_core::ComponentDescriptor,
    ) -> Option<C> {
        debug_assert_eq!(component_descr.component_type, Some(C::name()));
        debug_assert!(
            component_descr.archetype == Some("rerun.archetypes.RecordingInfo".into()),
            "This function should only be used for built-in RecordingInfo components, which are the only recording properties at {}",
            EntityPath::properties()
        );

        self.latest_at_component::<C>(
            &EntityPath::properties(),
            &LatestAtQuery::latest(TimelineName::log_tick()),
            component_descr,
        )
        .map(|(_, value)| value)
    }

    /// Use can use this both for setting the built-in `RecordingInfo` components,
    /// and for setting custom properties on the recording.
    pub fn set_recording_property<Component: re_types_core::Component>(
        &mut self,
        entity_path: EntityPath,
        component_descr: re_types_core::ComponentDescriptor,
        value: &Component,
    ) -> Result<(), Error> {
        debug_assert_eq!(component_descr.component_type, Some(Component::name()));
        debug_assert!(entity_path.starts_with(&EntityPath::properties()));
        debug_assert!(
            (entity_path == EntityPath::properties())
                == (component_descr.archetype == Some("rerun.archetypes.RecordingInfo".into())),
            "RecordingInfo should be logged at {}. Custom properties should be under a child entity",
            EntityPath::properties()
        );

        let chunk = ChunkBuilder::new(ChunkId::new(), entity_path)
            .with_component(RowId::new(), TimePoint::STATIC, component_descr, value)
            .map_err(|err| Error::Chunk(err.into()))?
            .build()?;

        self.add_chunk(&Arc::new(chunk))?;

        Ok(())
    }

    pub fn timeline_type(&self, timeline_name: &TimelineName) -> TimeType {
        self.storage_engine()
            .store()
            .time_column_type(timeline_name)
            .unwrap_or_else(|| {
                if timeline_name == &TimelineName::log_time() {
                    Timeline::log_time().typ()
                } else if timeline_name == &TimelineName::log_tick() {
                    Timeline::log_tick().typ()
                } else {
                    re_log::warn_once!("Timeline {timeline_name:?} not found");
                    TimeType::Sequence
                }
            })
    }

    /// Queries for the given components using latest-at semantics.
    ///
    /// See [`re_query::LatestAtResults`] for more information about how to handle the results.
    ///
    /// This is a cached API -- data will be lazily cached upon access.
    #[inline]
    pub fn latest_at<'a>(
        &self,
        query: &re_chunk_store::LatestAtQuery,
        entity_path: &EntityPath,
        component_descr: impl IntoIterator<Item = &'a re_types_core::ComponentDescriptor>,
    ) -> re_query::LatestAtResults {
        self.storage_engine
            .read()
            .cache()
            .latest_at(query, entity_path, component_descr)
    }

    /// Get the latest index and value for a given dense [`re_types_core::Component`].
    ///
    /// This assumes that the row we get from the store contains at most one instance for this
    /// component; it will log a warning otherwise.
    ///
    /// This should only be used for "mono-components" such as `Transform` and `Tensor`.
    ///
    /// This is a best-effort helper, it will merely log errors on failure.
    #[inline]
    pub fn latest_at_component<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
        component_descr: &re_types_core::ComponentDescriptor,
    ) -> Option<((TimeInt, RowId), C)> {
        debug_assert_eq!(component_descr.component_type, Some(C::name()));

        let results =
            self.storage_engine
                .read()
                .cache()
                .latest_at(query, entity_path, [component_descr]);
        results
            .component_mono(component_descr)
            .map(|value| (results.index(), value))
    }

    /// Get the latest index and value for a given dense [`re_types_core::Component`].
    ///
    /// This assumes that the row we get from the store contains at most one instance for this
    /// component; it will log a warning otherwise.
    ///
    /// This should only be used for "mono-components" such as `Transform` and `Tensor`.
    ///
    /// This is a best-effort helper, and will quietly swallow any errors.
    #[inline]
    pub fn latest_at_component_quiet<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
        component_descr: &re_types_core::ComponentDescriptor,
    ) -> Option<((TimeInt, RowId), C)> {
        debug_assert_eq!(component_descr.component_type, Some(C::name()));

        let results =
            self.storage_engine
                .read()
                .cache()
                .latest_at(query, entity_path, [component_descr]);
        results
            .component_mono_quiet(component_descr)
            .map(|value| (results.index(), value))
    }

    #[inline]
    pub fn latest_at_component_at_closest_ancestor<C: re_types_core::Component>(
        &self,
        entity_path: &EntityPath,
        query: &re_chunk_store::LatestAtQuery,
        component_descr: &re_types_core::ComponentDescriptor,
    ) -> Option<(EntityPath, (TimeInt, RowId), C)> {
        re_tracing::profile_function!();

        let mut cur_entity_path = Some(entity_path.clone());
        while let Some(entity_path) = cur_entity_path {
            if let Some((index, value)) =
                self.latest_at_component(&entity_path, query, component_descr)
            {
                return Some((entity_path, index, value));
            }
            cur_entity_path = entity_path.parent();
        }

        None
    }

    /// If this entity db is the result of a clone, which store was it cloned from?
    ///
    /// A cloned store always gets a new unique ID.
    ///
    /// We currently only use entity db cloning for blueprints:
    /// when we activate a _default_ blueprint that was received on the wire (e.g. from a recording),
    /// we clone it and make the clone the _active_ blueprint.
    /// This means all active blueprints are clones.
    #[inline]
    pub fn cloned_from(&self) -> Option<&StoreId> {
        self.store_info().and_then(|info| info.cloned_from.as_ref())
    }

    pub fn timelines(&self) -> std::collections::BTreeMap<TimelineName, Timeline> {
        self.storage_engine().store().timelines()
    }

    pub fn times_per_timeline(&self) -> &TimesPerTimeline {
        &self.times_per_timeline
    }

    pub fn has_any_data_on_timeline(&self, timeline: &TimelineName) -> bool {
        self.time_histogram_per_timeline
            .get(timeline)
            .is_some_and(|hist| !hist.is_empty())
    }

    /// Returns the time range of data on the given timeline, ignoring any static times.
    pub fn time_range_for(&self, timeline: &TimelineName) -> Option<ResolvedTimeRange> {
        let hist = self.time_histogram_per_timeline.get(timeline)?;
        let min = hist.min_key()?;
        let max = hist.max_key()?;
        Some(ResolvedTimeRange::new(min, max))
    }

    /// Histogram of all events on the timeeline, of all entities.
    pub fn time_histogram(&self, timeline: &TimelineName) -> Option<&crate::TimeHistogram> {
        self.time_histogram_per_timeline.get(timeline)
    }

    #[inline]
    pub fn num_rows(&self) -> u64 {
        self.storage_engine.read().store().stats().total().num_rows
    }

    /// Return the current `ChunkStoreGeneration`. This can be used to determine whether the
    /// database has been modified since the last time it was queried.
    #[inline]
    pub fn generation(&self) -> re_chunk_store::ChunkStoreGeneration {
        self.storage_engine.read().store().generation()
    }

    #[inline]
    pub fn last_modified_at(&self) -> web_time::Instant {
        self.last_modified_at
    }

    /// The highest `RowId` in the store,
    /// which corresponds to the last edit time.
    /// Ignores deletions.
    #[inline]
    pub fn latest_row_id(&self) -> Option<RowId> {
        self.latest_row_id
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set_store_info.is_none() && self.num_rows() == 0
    }

    /// A sorted list of all the entity paths in this database.
    pub fn entity_paths(&self) -> Vec<&EntityPath> {
        use itertools::Itertools as _;
        self.entity_path_from_hash.values().sorted().collect()
    }

    #[inline]
    pub fn ingestion_stats(&self) -> &IngestionStatistics {
        &self.stats
    }

    #[inline]
    pub fn entity_path_from_hash(&self, entity_path_hash: &EntityPathHash) -> Option<&EntityPath> {
        self.entity_path_from_hash.get(entity_path_hash)
    }

    /// Returns `true` also for entities higher up in the hierarchy.
    #[inline]
    pub fn is_known_entity(&self, entity_path: &EntityPath) -> bool {
        self.tree.subtree(entity_path).is_some()
    }

    /// If you log `world/points`, then that is a logged entity, but `world` is not,
    /// unless you log something to `world` too.
    #[inline]
    pub fn is_logged_entity(&self, entity_path: &EntityPath) -> bool {
        self.entity_path_from_hash.contains_key(&entity_path.hash())
    }

    pub fn add(&mut self, msg: &LogMsg) -> Result<Vec<ChunkStoreEvent>, Error> {
        re_tracing::profile_function!();

        debug_assert_eq!(msg.store_id(), self.store_id());

        let store_events = match &msg {
            LogMsg::SetStoreInfo(msg) => {
                self.set_store_info(msg.clone());
                vec![]
            }

            LogMsg::ArrowMsg(_, arrow_msg) => {
                self.last_modified_at = web_time::Instant::now();

                let chunk_batch = re_sorbet::ChunkBatch::try_from(&arrow_msg.batch)
                    .map_err(re_chunk::ChunkError::from)?;
                let mut chunk = re_chunk::Chunk::from_chunk_batch(&chunk_batch)?;
                chunk.sort_if_unsorted();
                self.add_chunk_with_timestamp_metadata(
                    &Arc::new(chunk),
                    &chunk_batch.sorbet_schema().timestamps,
                )?
            }

            LogMsg::BlueprintActivationCommand(_) => {
                // Not for us to handle
                vec![]
            }
        };

        Ok(store_events)
    }

    pub fn add_chunk(&mut self, chunk: &Arc<Chunk>) -> Result<Vec<ChunkStoreEvent>, Error> {
        self.add_chunk_with_timestamp_metadata(chunk, &Default::default())
    }

    fn add_chunk_with_timestamp_metadata(
        &mut self,
        chunk: &Arc<Chunk>,
        timestamps: &re_sorbet::TimestampMetadata,
    ) -> Result<Vec<ChunkStoreEvent>, Error> {
        let mut engine = self.storage_engine.write();
        let store_events = engine.store().insert_chunk(chunk)?;
        engine.cache().on_events(&store_events);

        self.entity_path_from_hash
            .entry(chunk.entity_path().hash())
            .or_insert_with(|| chunk.entity_path().clone());

        let engine = engine.downgrade();

        if self.latest_row_id < chunk.row_id_range().map(|(_, row_id_max)| row_id_max) {
            self.latest_row_id = chunk.row_id_range().map(|(_, row_id_max)| row_id_max);
        }

        {
            // Update our internal views by notifying them of resulting [`ChunkStoreEvent`]s.
            self.times_per_timeline.on_events(&store_events);
            self.time_histogram_per_timeline.on_events(&store_events);
            self.tree.on_store_additions(&store_events);

            // It is possible for writes to trigger deletions: specifically in the case of
            // overwritten static data leading to dangling chunks.
            let entity_paths_with_deletions = store_events
                .iter()
                .filter(|event| event.kind == ChunkStoreDiffKind::Deletion)
                .map(|event| event.chunk.entity_path().clone())
                .collect();

            {
                re_tracing::profile_scope!("on_store_deletions");
                self.tree
                    .on_store_deletions(&engine, &entity_paths_with_deletions, &store_events);
            }

            // We inform the stats last, since it measures e2e latency.
            self.stats.on_events(timestamps, &store_events);
        }

        Ok(store_events)
    }

    pub fn set_store_info(&mut self, store_info: SetStoreInfo) {
        self.set_store_info = Some(store_info);
    }

    /// Free up some RAM by forgetting the older parts of all timelines.
    pub fn purge_fraction_of_ram(&mut self, fraction_to_purge: f32) -> Vec<ChunkStoreEvent> {
        re_tracing::profile_function!();

        assert!((0.0..=1.0).contains(&fraction_to_purge));

        let store_events = self.gc(&GarbageCollectionOptions {
            target: GarbageCollectionTarget::DropAtLeastFraction(fraction_to_purge as _),
            protect_latest: 1,
            time_budget: DEFAULT_GC_TIME_BUDGET,

            // TODO(emilk): we could protect the data that is currently being viewed
            // (e.g. when paused in the live camera example).
            // To be perfect it would need margins (because of latest-at), i.e. we would need to know
            // exactly how far back the latest-at is of each component at the current time…
            // …but maybe it doesn't have to be perfect.
            protected_time_ranges: Default::default(),
        });

        if store_events.is_empty() {
            // If we weren't able to collect any data, then we need to GC the cache itself in order
            // to regain some space.
            // See <https://github.com/rerun-io/rerun/issues/7369#issuecomment-2335164098> for the
            // complete rationale.
            self.storage_engine
                .write()
                .cache()
                .purge_fraction_of_ram(fraction_to_purge);
        }

        store_events
    }

    pub fn gc(&mut self, gc_options: &GarbageCollectionOptions) -> Vec<ChunkStoreEvent> {
        re_tracing::profile_function!();

        let mut engine = self.storage_engine.write();
        let (store_events, stats_diff) = engine.store().gc(gc_options);

        re_log::trace!(
            num_row_ids_dropped = store_events.len(),
            size_bytes_dropped = re_format::format_bytes(stats_diff.total().total_size_bytes as _),
            "purged datastore"
        );

        Self::on_store_deletions(
            &mut self.times_per_timeline,
            &mut self.time_histogram_per_timeline,
            &mut self.tree,
            engine,
            &store_events,
        );

        store_events
    }

    /// Drop all events in the given time range from the given timeline.
    ///
    /// Used to implement undo (erase the last event from the blueprint db).
    pub fn drop_time_range(
        &mut self,
        timeline: &TimelineName,
        drop_range: ResolvedTimeRange,
    ) -> Vec<ChunkStoreEvent> {
        re_tracing::profile_function!();

        let mut engine = self.storage_engine.write();

        let store_events = engine.store().drop_time_range(timeline, drop_range);
        Self::on_store_deletions(
            &mut self.times_per_timeline,
            &mut self.time_histogram_per_timeline,
            &mut self.tree,
            engine,
            &store_events,
        );

        store_events
    }

    /// Unconditionally drops all the data for a given [`EntityPath`] .
    ///
    /// This is _not_ recursive. Children of this entity will not be affected.
    ///
    /// To drop the entire subtree below an entity, see: [`Self::drop_entity_path_recursive`].
    pub fn drop_entity_path(&mut self, entity_path: &EntityPath) {
        re_tracing::profile_function!();

        let mut engine = self.storage_engine.write();

        let store_events = engine.store().drop_entity_path(entity_path);
        Self::on_store_deletions(
            &mut self.times_per_timeline,
            &mut self.time_histogram_per_timeline,
            &mut self.tree,
            engine,
            &store_events,
        );
    }

    /// Unconditionally drops all the data for a given [`EntityPath`] and all its children.
    pub fn drop_entity_path_recursive(&mut self, entity_path: &EntityPath) {
        re_tracing::profile_function!();

        let mut to_drop = vec![entity_path.clone()];

        if let Some(tree) = self.tree().subtree(entity_path) {
            tree.visit_children_recursively(|path| {
                to_drop.push(path.clone());
            });
        }

        for entity_path in to_drop {
            self.drop_entity_path(&entity_path);
        }
    }

    // NOTE: Parameters deconstructed instead of taking `self`, because borrowck cannot understand
    // partial borrows on methods.
    fn on_store_deletions(
        times_per_timeline: &mut TimesPerTimeline,
        time_histogram_per_timeline: &mut crate::TimeHistogramPerTimeline,
        tree: &mut crate::EntityTree,
        mut engine: StorageEngineWriteGuard<'_>,
        store_events: &[ChunkStoreEvent],
    ) {
        engine.cache().on_events(store_events);
        times_per_timeline.on_events(store_events);
        time_histogram_per_timeline.on_events(store_events);

        let engine = engine.downgrade();
        let entity_paths_with_deletions = store_events
            .iter()
            .filter(|event| event.kind == ChunkStoreDiffKind::Deletion)
            .map(|event| event.chunk.entity_path().clone())
            .collect();
        tree.on_store_deletions(&engine, &entity_paths_with_deletions, store_events);
    }

    /// Export the contents of the current database to a sequence of messages.
    ///
    /// If `time_selection` is specified, then only data for that specific timeline over that
    /// specific time range will be accounted for.
    pub fn to_messages(
        &self,
        time_selection: Option<(TimelineName, ResolvedTimeRangeF)>,
    ) -> impl Iterator<Item = ChunkResult<LogMsg>> + '_ {
        re_tracing::profile_function!();

        let engine = self.storage_engine.read();

        let set_store_info_msg = self
            .store_info_msg()
            .map(|msg| Ok(LogMsg::SetStoreInfo(msg.clone())));

        let data_messages = {
            let time_filter = time_selection.map(|(timeline, range)| {
                (
                    timeline,
                    ResolvedTimeRange::new(range.min.floor(), range.max.ceil()),
                )
            });

            let mut chunks: Vec<Arc<Chunk>> = engine
                .store()
                .iter_chunks()
                .filter(move |chunk| {
                    let Some((timeline, time_range)) = time_filter else {
                        return true;
                    };

                    // TODO(cmc): chunk.slice_time_selection(time_selection)
                    chunk.timelines().get(&timeline).is_some_and(|time_column| {
                        time_range.contains(time_column.time_range().min())
                            || time_range.contains(time_column.time_range().max())
                    })
                })
                .cloned() // refcount
                .collect();

            // Try to roughly preserve the order of the chunks
            // from how they were originally logged.
            // See https://github.com/rerun-io/rerun/issues/7175 for why.
            chunks.sort_by_key(|chunk| chunk.row_id_range().map(|(min, _)| min));

            chunks.into_iter().map(|chunk| {
                chunk
                    .to_arrow_msg()
                    .map(|msg| LogMsg::ArrowMsg(self.store_id().clone(), msg))
            })
        };

        // If this is a blueprint, make sure to include the `BlueprintActivationCommand` message.
        // We generally use `to_messages` to export a blueprint via "save". In that
        // case, we want to make the blueprint active and default when it's reloaded.
        // TODO(jleibs): Coupling this with the stored file instead of injecting seems
        // architecturally weird. Would be great if we didn't need this in `.rbl` files
        // at all.
        let blueprint_ready = if self.store_kind() == StoreKind::Blueprint {
            let activate_cmd =
                re_log_types::BlueprintActivationCommand::make_active(self.store_id().clone());

            itertools::Either::Left(std::iter::once(Ok(activate_cmd.into())))
        } else {
            itertools::Either::Right(std::iter::empty())
        };

        set_store_info_msg
            .into_iter()
            .chain(data_messages)
            .chain(blueprint_ready)
    }

    /// Make a clone of this [`EntityDb`], assigning it a new [`StoreId`].
    pub fn clone_with_new_id(&self, new_id: StoreId) -> Result<Self, Error> {
        re_tracing::profile_function!();

        let mut new_db = Self::new(new_id.clone());

        new_db.last_modified_at = self.last_modified_at;
        new_db.latest_row_id = self.latest_row_id;

        // We do NOT clone the `data_source`, because the reason we clone an entity db
        // is so that we can modify it, and then it would be wrong to say its from the same source.
        // Specifically: if we load a blueprint from an `.rdd`, then modify it heavily and save it,
        // it would be wrong to claim that this was the blueprint from that `.rrd`,
        // and it would confuse the user.
        // TODO(emilk): maybe we should use a special `Cloned` data source,
        // wrapping either the original source, the original StoreId, or both.

        if let Some(store_info) = self.store_info() {
            let mut new_info = store_info.clone();
            new_info.store_id = new_id;
            new_info.cloned_from = Some(self.store_id().clone());

            new_db.set_store_info(SetStoreInfo {
                row_id: *RowId::new(),
                info: new_info,
            });
        }

        let engine = self.storage_engine.read();
        for chunk in engine.store().iter_chunks() {
            new_db.add_chunk(&Arc::clone(chunk))?;
        }

        Ok(new_db)
    }
}

/// ## Stats
impl EntityDb {
    /// Returns the stats for the static store of the entity and all its children, recursively.
    ///
    /// This excludes temporal data.
    pub fn subtree_stats_static(
        &self,
        engine: &StorageEngineReadGuard<'_>,
        entity_path: &EntityPath,
    ) -> ChunkStoreChunkStats {
        re_tracing::profile_function!();

        let Some(subtree) = self.tree.subtree(entity_path) else {
            return Default::default();
        };

        let mut stats = ChunkStoreChunkStats::default();
        subtree.visit_children_recursively(|path| {
            stats += engine.store().entity_stats_static(path);
        });

        stats
    }

    /// Returns the stats for the entity and all its children on the given timeline, recursively.
    ///
    /// This excludes static data.
    pub fn subtree_stats_on_timeline(
        &self,
        engine: &StorageEngineReadGuard<'_>,
        entity_path: &EntityPath,
        timeline: &TimelineName,
    ) -> ChunkStoreChunkStats {
        re_tracing::profile_function!();

        let Some(subtree) = self.tree.subtree(entity_path) else {
            return Default::default();
        };

        let mut stats = ChunkStoreChunkStats::default();
        subtree.visit_children_recursively(|path| {
            stats += engine.store().entity_stats_on_timeline(path, timeline);
        });

        stats
    }

    /// Returns true if an entity or any of its children have any data on the given timeline.
    ///
    /// This includes static data.
    pub fn subtree_has_data_on_timeline(
        &self,
        engine: &StorageEngineReadGuard<'_>,
        timeline: &TimelineName,
        entity_path: &EntityPath,
    ) -> bool {
        re_tracing::profile_function!();

        let Some(subtree) = self.tree.subtree(entity_path) else {
            return false;
        };

        subtree
            .find_first_child_recursive(|path| {
                engine.store().entity_has_data_on_timeline(timeline, path)
            })
            .is_some()
    }

    /// Returns true if an entity or any of its children have any temporal data on the given timeline.
    ///
    /// This ignores static data.
    pub fn subtree_has_temporal_data_on_timeline(
        &self,
        engine: &StorageEngineReadGuard<'_>,
        timeline: &TimelineName,
        entity_path: &EntityPath,
    ) -> bool {
        re_tracing::profile_function!();

        let Some(subtree) = self.tree.subtree(entity_path) else {
            return false;
        };

        subtree
            .find_first_child_recursive(|path| {
                engine
                    .store()
                    .entity_has_temporal_data_on_timeline(timeline, path)
            })
            .is_some()
    }
}

impl re_byte_size::SizeBytes for EntityDb {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        // TODO(emilk): size of entire EntityDb, including secondary indices etc
        self.storage_engine
            .read()
            .store()
            .stats()
            .total()
            .total_size_bytes
    }
}
