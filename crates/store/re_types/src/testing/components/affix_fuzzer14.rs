// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

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
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AffixFuzzer14(pub crate::testing::datatypes::AffixFuzzer3);

impl ::re_types_core::Component for AffixFuzzer14 {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.testing.components.AffixFuzzer14")
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer14);

impl ::re_types_core::Loggable for AffixFuzzer14 {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::testing::datatypes::AffixFuzzer3::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::testing::datatypes::AffixFuzzer3::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::testing::datatypes::AffixFuzzer3::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}

impl<T: Into<crate::testing::datatypes::AffixFuzzer3>> From<T> for AffixFuzzer14 {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::testing::datatypes::AffixFuzzer3> for AffixFuzzer14 {
    #[inline]
    fn borrow(&self) -> &crate::testing::datatypes::AffixFuzzer3 {
        &self.0
    }
}

impl std::ops::Deref for AffixFuzzer14 {
    type Target = crate::testing::datatypes::AffixFuzzer3;

    #[inline]
    fn deref(&self) -> &crate::testing::datatypes::AffixFuzzer3 {
        &self.0
    }
}

impl std::ops::DerefMut for AffixFuzzer14 {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::testing::datatypes::AffixFuzzer3 {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for AffixFuzzer14 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::testing::datatypes::AffixFuzzer3>::is_pod()
    }
}
