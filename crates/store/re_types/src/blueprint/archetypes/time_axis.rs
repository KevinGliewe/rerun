// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/time_axis.fbs".

#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentType};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Configuration for the time (Y) axis of a plot.
///
/// ⚠️ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
#[derive(Clone, Debug, Default)]
pub struct TimeAxis {
    /// How should the horizontal/X/time axis be linked across multiple plots?
    pub link: Option<SerializedComponentBatch>,
}

impl TimeAxis {
    /// Returns the [`ComponentDescriptor`] for [`Self::link`].
    ///
    /// The corresponding component is [`crate::blueprint::components::LinkAxis`].
    #[inline]
    pub fn descriptor_link() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype: Some("rerun.blueprint.archetypes.TimeAxis".into()),
            component: "TimeAxis:link".into(),
            component_type: Some("rerun.blueprint.components.LinkAxis".into()),
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [TimeAxis::descriptor_link()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [TimeAxis::descriptor_link()]);

impl TimeAxis {
    /// The total number of components in the archetype: 0 required, 0 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 1usize;
}

impl ::re_types_core::Archetype for TimeAxis {
    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.TimeAxis".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Time axis"
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let link = arrays_by_descr
            .get(&Self::descriptor_link())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_link()));
        Ok(Self { link })
    }
}

impl ::re_types_core::AsComponents for TimeAxis {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        std::iter::once(self.link.clone()).flatten().collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for TimeAxis {}

impl TimeAxis {
    /// Create a new `TimeAxis`.
    #[inline]
    pub fn new() -> Self {
        Self { link: None }
    }

    /// Update only some specific fields of a `TimeAxis`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `TimeAxis`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            link: Some(SerializedComponentBatch::new(
                crate::blueprint::components::LinkAxis::arrow_empty(),
                Self::descriptor_link(),
            )),
        }
    }

    /// How should the horizontal/X/time axis be linked across multiple plots?
    #[inline]
    pub fn with_link(mut self, link: impl Into<crate::blueprint::components::LinkAxis>) -> Self {
        self.link = try_serialize_field(Self::descriptor_link(), [link]);
        self
    }
}

impl ::re_byte_size::SizeBytes for TimeAxis {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.link.heap_size_bytes()
    }
}
