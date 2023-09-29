// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

#[derive(Clone, Debug, PartialEq)]
pub struct AffixFuzzer2 {
    pub fuzz1101: Vec<crate::testing::components::AffixFuzzer1>,
    pub fuzz1102: Vec<crate::testing::components::AffixFuzzer2>,
    pub fuzz1103: Vec<crate::testing::components::AffixFuzzer3>,
    pub fuzz1104: Vec<crate::testing::components::AffixFuzzer4>,
    pub fuzz1105: Vec<crate::testing::components::AffixFuzzer5>,
    pub fuzz1106: Vec<crate::testing::components::AffixFuzzer6>,
    pub fuzz1107: Vec<crate::testing::components::AffixFuzzer7>,
    pub fuzz1108: Vec<crate::testing::components::AffixFuzzer8>,
    pub fuzz1109: Vec<crate::testing::components::AffixFuzzer9>,
    pub fuzz1110: Vec<crate::testing::components::AffixFuzzer10>,
    pub fuzz1111: Vec<crate::testing::components::AffixFuzzer11>,
    pub fuzz1112: Vec<crate::testing::components::AffixFuzzer12>,
    pub fuzz1113: Vec<crate::testing::components::AffixFuzzer13>,
    pub fuzz1114: Vec<crate::testing::components::AffixFuzzer14>,
    pub fuzz1115: Vec<crate::testing::components::AffixFuzzer15>,
    pub fuzz1116: Vec<crate::testing::components::AffixFuzzer16>,
    pub fuzz1117: Vec<crate::testing::components::AffixFuzzer17>,
    pub fuzz1118: Vec<crate::testing::components::AffixFuzzer18>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 18usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.testing.components.AffixFuzzer2Indicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.InstanceKey".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 20usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
            "rerun.testing.components.AffixFuzzer2Indicator".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

impl AffixFuzzer2 {
    pub const NUM_COMPONENTS: usize = 20usize;
}

/// Indicator component for the [`AffixFuzzer2`] [`crate::Archetype`]
pub type AffixFuzzer2Indicator = crate::GenericIndicatorComponent<AffixFuzzer2>;

impl crate::Archetype for AffixFuzzer2 {
    type Indicator = AffixFuzzer2Indicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer2".into()
    }

    #[inline]
    fn indicator() -> crate::MaybeOwnedComponentBatch<'static> {
        static INDICATOR: AffixFuzzer2Indicator = AffixFuzzer2Indicator::DEFAULT;
        crate::MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow(
        arrow_data: impl IntoIterator<
            Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>),
        >,
    ) -> crate::DeserializationResult<Self> {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let fuzz1101 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer1")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1101")?;
            <crate::testing::components::AffixFuzzer1>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1101")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1101")?
        };
        let fuzz1102 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer2")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1102")?;
            <crate::testing::components::AffixFuzzer2>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1102")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1102")?
        };
        let fuzz1103 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer3")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1103")?;
            <crate::testing::components::AffixFuzzer3>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1103")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1103")?
        };
        let fuzz1104 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer4")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1104")?;
            <crate::testing::components::AffixFuzzer4>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1104")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1104")?
        };
        let fuzz1105 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer5")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1105")?;
            <crate::testing::components::AffixFuzzer5>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1105")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1105")?
        };
        let fuzz1106 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer6")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1106")?;
            <crate::testing::components::AffixFuzzer6>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1106")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1106")?
        };
        let fuzz1107 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer7")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1107")?;
            <crate::testing::components::AffixFuzzer7>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1107")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1107")?
        };
        let fuzz1108 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer8")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1108")?;
            <crate::testing::components::AffixFuzzer8>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1108")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1108")?
        };
        let fuzz1109 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer9")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1109")?;
            <crate::testing::components::AffixFuzzer9>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1109")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1109")?
        };
        let fuzz1110 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer10")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1110")?;
            <crate::testing::components::AffixFuzzer10>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1110")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1110")?
        };
        let fuzz1111 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer11")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1111")?;
            <crate::testing::components::AffixFuzzer11>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1111")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1111")?
        };
        let fuzz1112 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer12")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1112")?;
            <crate::testing::components::AffixFuzzer12>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1112")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1112")?
        };
        let fuzz1113 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer13")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1113")?;
            <crate::testing::components::AffixFuzzer13>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1113")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1113")?
        };
        let fuzz1114 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer14")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1114")?;
            <crate::testing::components::AffixFuzzer14>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1114")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1114")?
        };
        let fuzz1115 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer15")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1115")?;
            <crate::testing::components::AffixFuzzer15>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1115")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1115")?
        };
        let fuzz1116 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer16")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1116")?;
            <crate::testing::components::AffixFuzzer16>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1116")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1116")?
        };
        let fuzz1117 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer17")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1117")?;
            <crate::testing::components::AffixFuzzer17>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1117")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1117")?
        };
        let fuzz1118 = {
            let array = arrays_by_name
                .get("rerun.testing.components.AffixFuzzer18")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1118")?;
            <crate::testing::components::AffixFuzzer18>::from_arrow_opt(&**array)
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1118")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.archetypes.AffixFuzzer2#fuzz1118")?
        };
        Ok(Self {
            fuzz1101,
            fuzz1102,
            fuzz1103,
            fuzz1104,
            fuzz1105,
            fuzz1106,
            fuzz1107,
            fuzz1108,
            fuzz1109,
            fuzz1110,
            fuzz1111,
            fuzz1112,
            fuzz1113,
            fuzz1114,
            fuzz1115,
            fuzz1116,
            fuzz1117,
            fuzz1118,
        })
    }
}

impl crate::AsComponents for AffixFuzzer2 {
    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.fuzz1101 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1102 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1103 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1104 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1105 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1106 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1107 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1108 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1109 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1110 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1111 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1112 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1113 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1114 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1115 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1116 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1117 as &dyn crate::ComponentBatch).into()),
            Some((&self.fuzz1118 as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.fuzz1101.len()
    }
}

impl AffixFuzzer2 {
    pub fn new(
        fuzz1101: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer1>>,
        fuzz1102: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer2>>,
        fuzz1103: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer3>>,
        fuzz1104: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer4>>,
        fuzz1105: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer5>>,
        fuzz1106: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer6>>,
        fuzz1107: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer7>>,
        fuzz1108: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer8>>,
        fuzz1109: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer9>>,
        fuzz1110: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer10>>,
        fuzz1111: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer11>>,
        fuzz1112: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer12>>,
        fuzz1113: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer13>>,
        fuzz1114: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer14>>,
        fuzz1115: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer15>>,
        fuzz1116: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer16>>,
        fuzz1117: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer17>>,
        fuzz1118: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer18>>,
    ) -> Self {
        Self {
            fuzz1101: fuzz1101.into_iter().map(Into::into).collect(),
            fuzz1102: fuzz1102.into_iter().map(Into::into).collect(),
            fuzz1103: fuzz1103.into_iter().map(Into::into).collect(),
            fuzz1104: fuzz1104.into_iter().map(Into::into).collect(),
            fuzz1105: fuzz1105.into_iter().map(Into::into).collect(),
            fuzz1106: fuzz1106.into_iter().map(Into::into).collect(),
            fuzz1107: fuzz1107.into_iter().map(Into::into).collect(),
            fuzz1108: fuzz1108.into_iter().map(Into::into).collect(),
            fuzz1109: fuzz1109.into_iter().map(Into::into).collect(),
            fuzz1110: fuzz1110.into_iter().map(Into::into).collect(),
            fuzz1111: fuzz1111.into_iter().map(Into::into).collect(),
            fuzz1112: fuzz1112.into_iter().map(Into::into).collect(),
            fuzz1113: fuzz1113.into_iter().map(Into::into).collect(),
            fuzz1114: fuzz1114.into_iter().map(Into::into).collect(),
            fuzz1115: fuzz1115.into_iter().map(Into::into).collect(),
            fuzz1116: fuzz1116.into_iter().map(Into::into).collect(),
            fuzz1117: fuzz1117.into_iter().map(Into::into).collect(),
            fuzz1118: fuzz1118.into_iter().map(Into::into).collect(),
        }
    }
}
