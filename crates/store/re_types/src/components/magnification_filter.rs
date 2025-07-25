// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/magnification_filter.fbs".

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
#![allow(non_camel_case_types)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentType};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Filter used when magnifying an image/texture such that a single pixel/texel is displayed as multiple pixels on screen.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum MagnificationFilter {
    /// Show the nearest pixel value.
    ///
    /// This will give a blocky appearance when zooming in.
    /// Used as default when rendering 2D images.
    #[default]
    Nearest = 1,

    /// Linearly interpolate the nearest neighbors, creating a smoother look when zooming in.
    ///
    /// Used as default for mesh rendering.
    Linear = 2,
}

impl ::re_types_core::Component for MagnificationFilter {
    #[inline]
    fn name() -> ComponentType {
        "rerun.components.MagnificationFilter".into()
    }
}

::re_types_core::macros::impl_into_cow!(MagnificationFilter);

impl ::re_types_core::Loggable for MagnificationFilter {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::UInt8
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| *datum as u8);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(PrimitiveArray::<UInt8Type>::new(
                ScalarBuffer::from(
                    data0
                        .into_iter()
                        .map(|v| v.unwrap_or_default())
                        .collect::<Vec<_>>(),
                ),
                data0_validity,
            ))
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<UInt8Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.components.MagnificationFilter#enum")?
            .into_iter()
            .map(|typ| match typ {
                Some(1) => Ok(Some(Self::Nearest)),
                Some(2) => Ok(Some(Self::Linear)),
                None => Ok(None),
                Some(invalid) => Err(DeserializationError::missing_union_arm(
                    Self::arrow_datatype(),
                    "<invalid>",
                    invalid as _,
                )),
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.MagnificationFilter")?)
    }
}

impl std::fmt::Display for MagnificationFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nearest => write!(f, "Nearest"),
            Self::Linear => write!(f, "Linear"),
        }
    }
}

impl ::re_types_core::reflection::Enum for MagnificationFilter {
    #[inline]
    fn variants() -> &'static [Self] {
        &[Self::Nearest, Self::Linear]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::Nearest => {
                "Show the nearest pixel value.\n\nThis will give a blocky appearance when zooming in.\nUsed as default when rendering 2D images."
            }
            Self::Linear => {
                "Linearly interpolate the nearest neighbors, creating a smoother look when zooming in.\n\nUsed as default for mesh rendering."
            }
        }
    }
}

impl ::re_byte_size::SizeBytes for MagnificationFilter {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}
