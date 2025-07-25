use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use itertools::{Itertools as _, izip};

use re_chunk::Chunk;

// ---

#[derive(Debug, Clone, clap::Parser)]
pub struct CompareCommand {
    path_to_rrd1: String,
    path_to_rrd2: String,

    /// If specified, the comparison will focus purely on semantics, ignoring order.
    ///
    /// The Rerun data model is itself unordered, and because many of the internal pipelines are
    /// asynchronous by nature, it is very easy to end up with semantically identical, but
    /// differently ordered data.
    /// In most cases, the distinction is irrelevant, and you'd rather the comparison succeeds.
    #[clap(long, default_value_t = false)]
    unordered: bool,

    /// If specified, dumps both .rrd files as tables.
    #[clap(long, default_value_t = false)]
    full_dump: bool,

    /// If specified, the comparison will ignore chunks without components.
    #[clap(long, default_value_t = false)]
    ignore_chunks_without_components: bool,
}

impl CompareCommand {
    /// Checks whether two .rrd files are _similar_, i.e. not equal on a byte-level but
    /// functionally equivalent.
    ///
    /// Returns `Ok(())` if they match, or an error containing a detailed diff otherwise.
    pub fn run(&self) -> anyhow::Result<()> {
        let Self {
            path_to_rrd1,
            path_to_rrd2,
            unordered,
            full_dump,
            ignore_chunks_without_components,
        } = self;

        re_log::debug!("Comparing {path_to_rrd1:?} to {path_to_rrd2:?}…");

        let path_to_rrd1 = PathBuf::from(path_to_rrd1);
        let path_to_rrd2 = PathBuf::from(path_to_rrd2);

        let (app_id1, chunks1) = load_chunks(&path_to_rrd1, *ignore_chunks_without_components)
            .with_context(|| format!("path: {path_to_rrd1:?}"))?;
        let (app_id2, chunks2) = load_chunks(&path_to_rrd2, *ignore_chunks_without_components)
            .with_context(|| format!("path: {path_to_rrd2:?}"))?;

        if *full_dump {
            println!("{app_id1}");
            for chunk in &chunks1 {
                println!("{chunk}");
            }

            println!("{app_id2}");
            for chunk in &chunks2 {
                println!("{chunk}");
            }
        }

        anyhow::ensure!(
            app_id1 == app_id2,
            "Application IDs do not match: '{app_id1}' vs. '{app_id2}'"
        );

        anyhow::ensure!(
            chunks1.len() == chunks2.len(),
            "Number of Chunks does not match: '{}' vs. '{}'",
            re_format::format_uint(chunks1.len()),
            re_format::format_uint(chunks2.len()),
        );

        let mut unordered_failed = false;
        if *unordered {
            let mut chunks2_opt: Vec<Option<Arc<Chunk>>> =
                chunks2.clone().into_iter().map(Some).collect_vec();
            'outer: for chunk1 in &chunks1 {
                for chunk2 in chunks2_opt.iter_mut().filter(|c| c.is_some()) {
                    #[allow(clippy::unwrap_used)]
                    if re_chunk::Chunk::ensure_similar(chunk1, chunk2.as_ref().unwrap()).is_ok() {
                        *chunk2 = None;
                        continue 'outer;
                    }
                }
                unordered_failed = true;
                break;
            }
        }

        fn format_chunk(chunk: &Chunk) -> String {
            re_format_arrow::format_record_batch_opts(
                &chunk.to_record_batch().expect("Cannot fail in practice"),
                &re_format_arrow::RecordBatchFormatOpts {
                    transposed: false,
                    width: Some(800),
                    include_metadata: true,
                    include_column_metadata: true,
                    trim_field_names: false,
                    trim_metadata_keys: false,
                    trim_metadata_values: false,
                },
            )
            .to_string()
        }

        if !*unordered || unordered_failed {
            for (chunk1, chunk2) in izip!(chunks1, chunks2) {
                re_chunk::Chunk::ensure_similar(&chunk1, &chunk2).with_context(|| {
                    format!(
                        "Chunks diff:\n{}",
                        similar_asserts::SimpleDiff::from_str(
                            &format_chunk(&chunk1),
                            &format_chunk(&chunk2),
                            "got",
                            "expected",
                        ),
                    )
                })?;
            }
        }

        re_log::debug!("{path_to_rrd1:?} and {path_to_rrd2:?} are similar enough.");

        Ok(())
    }
}

/// Given a path to an rrd file, builds up a `ChunkStore` and returns its contents a stream of
/// `Chunk`s.
///
/// Fails if there are more than one data recordings present in the rrd file.
fn load_chunks(
    path_to_rrd: &Path,
    ignore_chunks_without_components: bool,
) -> anyhow::Result<(re_log_types::ApplicationId, Vec<Arc<re_chunk::Chunk>>)> {
    use re_entity_db::EntityDb;
    use re_log_types::StoreId;

    let rrd_file = std::fs::File::open(path_to_rrd).context("couldn't open rrd file contents")?;
    let rrd_file = std::io::BufReader::new(rrd_file);

    // TODO(#10730): if the legacy `StoreId` migration is removed from `Decoder`, this would break
    // the ability of `rrd compare` pre-0.25 rrds. If we want to keep the ability to migrate here,
    // then the pre-#10730 app id caching mechanism must somehow be ported here.
    // TODO(ab): For pre-0.25 legacy data with `StoreId` missing their application id, the migration
    // in `Decoder` requires `SetStoreInfo` to arrive before the corresponding `ArrowMsg`. Ideally
    // this tool would cache orphan `ArrowMsg` until a matching `SetStoreInfo` arrives.
    let mut stores: std::collections::HashMap<StoreId, EntityDb> = Default::default();
    let decoder = re_log_encoding::decoder::Decoder::new(rrd_file)?;
    for msg in decoder {
        let msg = msg.context("decode rrd message")?;
        stores
            .entry(msg.store_id().clone())
            .or_insert_with(|| {
                re_entity_db::EntityDb::with_store_config(
                    msg.store_id().clone(),
                    // We must make sure not to do any store-side compaction during comparisons, or
                    // this will result in flaky roundtrips in some instances.
                    re_chunk_store::ChunkStoreConfig::ALL_DISABLED,
                )
            })
            .add(&msg)
            .context("decode rrd file contents")?;
    }

    let mut stores = stores
        .values()
        .filter(|store| store.store_kind() == re_log_types::StoreKind::Recording)
        .collect_vec();

    anyhow::ensure!(!stores.is_empty(), "no data recording found in rrd file");
    anyhow::ensure!(
        stores.len() == 1,
        "more than one data recording found in rrd file"
    );

    #[allow(clippy::unwrap_used)] // safe, ensured above
    let store = stores.pop().unwrap();
    let engine = store.storage_engine();

    Ok((
        store.application_id().clone(),
        engine
            .store()
            .iter_chunks()
            .filter_map(|c| {
                if ignore_chunks_without_components {
                    (c.num_components() > 0).then_some(c.clone())
                } else {
                    Some(c.clone())
                }
            })
            .collect_vec(),
    ))
}
