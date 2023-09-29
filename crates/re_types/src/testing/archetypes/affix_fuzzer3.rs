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
pub struct AffixFuzzer3 {
    pub fuzz2001: Option<crate::testing::components::AffixFuzzer1>,
    pub fuzz2002: Option<crate::testing::components::AffixFuzzer2>,
    pub fuzz2003: Option<crate::testing::components::AffixFuzzer3>,
    pub fuzz2004: Option<crate::testing::components::AffixFuzzer4>,
    pub fuzz2005: Option<crate::testing::components::AffixFuzzer5>,
    pub fuzz2006: Option<crate::testing::components::AffixFuzzer6>,
    pub fuzz2007: Option<crate::testing::components::AffixFuzzer7>,
    pub fuzz2008: Option<crate::testing::components::AffixFuzzer8>,
    pub fuzz2009: Option<crate::testing::components::AffixFuzzer9>,
    pub fuzz2010: Option<crate::testing::components::AffixFuzzer10>,
    pub fuzz2011: Option<crate::testing::components::AffixFuzzer11>,
    pub fuzz2012: Option<crate::testing::components::AffixFuzzer12>,
    pub fuzz2013: Option<crate::testing::components::AffixFuzzer13>,
    pub fuzz2014: Option<crate::testing::components::AffixFuzzer14>,
    pub fuzz2015: Option<crate::testing::components::AffixFuzzer15>,
    pub fuzz2016: Option<crate::testing::components::AffixFuzzer16>,
    pub fuzz2017: Option<crate::testing::components::AffixFuzzer17>,
    pub fuzz2018: Option<crate::testing::components::AffixFuzzer18>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.testing.components.AffixFuzzer3Indicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.InstanceKey".into(),
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

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 20usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer3Indicator".into(),
            "rerun.components.InstanceKey".into(),
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

impl AffixFuzzer3 {
    pub const NUM_COMPONENTS: usize = 20usize;
}

/// Indicator component for the [`AffixFuzzer3`] [`crate::Archetype`]
pub type AffixFuzzer3Indicator = crate::GenericIndicatorComponent<AffixFuzzer3>;

impl crate::Archetype for AffixFuzzer3 {
    type Indicator = AffixFuzzer3Indicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer3".into()
    }

    #[inline]
    fn indicator() -> crate::MaybeOwnedComponentBatch<'static> {
        static INDICATOR: AffixFuzzer3Indicator = AffixFuzzer3Indicator::DEFAULT;
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
        let fuzz2001 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer1") {
                Some({
                    <crate::testing::components::AffixFuzzer1>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2001")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2001")?
                })
            } else {
                None
            };
        let fuzz2002 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer2") {
                Some({
                    <crate::testing::components::AffixFuzzer2>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2002")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2002")?
                })
            } else {
                None
            };
        let fuzz2003 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer3") {
                Some({
                    <crate::testing::components::AffixFuzzer3>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2003")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2003")?
                })
            } else {
                None
            };
        let fuzz2004 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer4") {
                Some({
                    <crate::testing::components::AffixFuzzer4>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2004")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2004")?
                })
            } else {
                None
            };
        let fuzz2005 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer5") {
                Some({
                    <crate::testing::components::AffixFuzzer5>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2005")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2005")?
                })
            } else {
                None
            };
        let fuzz2006 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer6") {
                Some({
                    <crate::testing::components::AffixFuzzer6>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2006")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2006")?
                })
            } else {
                None
            };
        let fuzz2007 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer7") {
                Some({
                    <crate::testing::components::AffixFuzzer7>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2007")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2007")?
                })
            } else {
                None
            };
        let fuzz2008 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer8") {
                Some({
                    <crate::testing::components::AffixFuzzer8>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2008")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2008")?
                })
            } else {
                None
            };
        let fuzz2009 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer9") {
                Some({
                    <crate::testing::components::AffixFuzzer9>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2009")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2009")?
                })
            } else {
                None
            };
        let fuzz2010 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer10") {
                Some({
                    <crate::testing::components::AffixFuzzer10>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2010")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2010")?
                })
            } else {
                None
            };
        let fuzz2011 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer11") {
                Some({
                    <crate::testing::components::AffixFuzzer11>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2011")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2011")?
                })
            } else {
                None
            };
        let fuzz2012 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer12") {
                Some({
                    <crate::testing::components::AffixFuzzer12>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2012")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2012")?
                })
            } else {
                None
            };
        let fuzz2013 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer13") {
                Some({
                    <crate::testing::components::AffixFuzzer13>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2013")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2013")?
                })
            } else {
                None
            };
        let fuzz2014 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer14") {
                Some({
                    <crate::testing::components::AffixFuzzer14>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2014")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2014")?
                })
            } else {
                None
            };
        let fuzz2015 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer15") {
                Some({
                    <crate::testing::components::AffixFuzzer15>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2015")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2015")?
                })
            } else {
                None
            };
        let fuzz2016 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer16") {
                Some({
                    <crate::testing::components::AffixFuzzer16>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2016")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2016")?
                })
            } else {
                None
            };
        let fuzz2017 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer17") {
                Some({
                    <crate::testing::components::AffixFuzzer17>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2017")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2017")?
                })
            } else {
                None
            };
        let fuzz2018 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer18") {
                Some({
                    <crate::testing::components::AffixFuzzer18>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2018")?
                        .into_iter()
                        .next()
                        .flatten()
                        .ok_or_else(crate::DeserializationError::missing_data)
                        .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2018")?
                })
            } else {
                None
            };
        Ok(Self {
            fuzz2001,
            fuzz2002,
            fuzz2003,
            fuzz2004,
            fuzz2005,
            fuzz2006,
            fuzz2007,
            fuzz2008,
            fuzz2009,
            fuzz2010,
            fuzz2011,
            fuzz2012,
            fuzz2013,
            fuzz2014,
            fuzz2015,
            fuzz2016,
            fuzz2017,
            fuzz2018,
        })
    }
}

impl crate::AsComponents for AffixFuzzer3 {
    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use crate::Archetype as _;
        [
            Some(Self::indicator()),
            self.fuzz2001
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2002
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2003
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2004
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2005
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2006
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2007
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2008
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2009
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2010
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2011
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2012
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2013
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2014
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2015
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2016
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2017
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.fuzz2018
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        0
    }
}

impl AffixFuzzer3 {
    pub fn new() -> Self {
        Self {
            fuzz2001: None,
            fuzz2002: None,
            fuzz2003: None,
            fuzz2004: None,
            fuzz2005: None,
            fuzz2006: None,
            fuzz2007: None,
            fuzz2008: None,
            fuzz2009: None,
            fuzz2010: None,
            fuzz2011: None,
            fuzz2012: None,
            fuzz2013: None,
            fuzz2014: None,
            fuzz2015: None,
            fuzz2016: None,
            fuzz2017: None,
            fuzz2018: None,
        }
    }

    pub fn with_fuzz2001(
        mut self,
        fuzz2001: impl Into<crate::testing::components::AffixFuzzer1>,
    ) -> Self {
        self.fuzz2001 = Some(fuzz2001.into());
        self
    }

    pub fn with_fuzz2002(
        mut self,
        fuzz2002: impl Into<crate::testing::components::AffixFuzzer2>,
    ) -> Self {
        self.fuzz2002 = Some(fuzz2002.into());
        self
    }

    pub fn with_fuzz2003(
        mut self,
        fuzz2003: impl Into<crate::testing::components::AffixFuzzer3>,
    ) -> Self {
        self.fuzz2003 = Some(fuzz2003.into());
        self
    }

    pub fn with_fuzz2004(
        mut self,
        fuzz2004: impl Into<crate::testing::components::AffixFuzzer4>,
    ) -> Self {
        self.fuzz2004 = Some(fuzz2004.into());
        self
    }

    pub fn with_fuzz2005(
        mut self,
        fuzz2005: impl Into<crate::testing::components::AffixFuzzer5>,
    ) -> Self {
        self.fuzz2005 = Some(fuzz2005.into());
        self
    }

    pub fn with_fuzz2006(
        mut self,
        fuzz2006: impl Into<crate::testing::components::AffixFuzzer6>,
    ) -> Self {
        self.fuzz2006 = Some(fuzz2006.into());
        self
    }

    pub fn with_fuzz2007(
        mut self,
        fuzz2007: impl Into<crate::testing::components::AffixFuzzer7>,
    ) -> Self {
        self.fuzz2007 = Some(fuzz2007.into());
        self
    }

    pub fn with_fuzz2008(
        mut self,
        fuzz2008: impl Into<crate::testing::components::AffixFuzzer8>,
    ) -> Self {
        self.fuzz2008 = Some(fuzz2008.into());
        self
    }

    pub fn with_fuzz2009(
        mut self,
        fuzz2009: impl Into<crate::testing::components::AffixFuzzer9>,
    ) -> Self {
        self.fuzz2009 = Some(fuzz2009.into());
        self
    }

    pub fn with_fuzz2010(
        mut self,
        fuzz2010: impl Into<crate::testing::components::AffixFuzzer10>,
    ) -> Self {
        self.fuzz2010 = Some(fuzz2010.into());
        self
    }

    pub fn with_fuzz2011(
        mut self,
        fuzz2011: impl Into<crate::testing::components::AffixFuzzer11>,
    ) -> Self {
        self.fuzz2011 = Some(fuzz2011.into());
        self
    }

    pub fn with_fuzz2012(
        mut self,
        fuzz2012: impl Into<crate::testing::components::AffixFuzzer12>,
    ) -> Self {
        self.fuzz2012 = Some(fuzz2012.into());
        self
    }

    pub fn with_fuzz2013(
        mut self,
        fuzz2013: impl Into<crate::testing::components::AffixFuzzer13>,
    ) -> Self {
        self.fuzz2013 = Some(fuzz2013.into());
        self
    }

    pub fn with_fuzz2014(
        mut self,
        fuzz2014: impl Into<crate::testing::components::AffixFuzzer14>,
    ) -> Self {
        self.fuzz2014 = Some(fuzz2014.into());
        self
    }

    pub fn with_fuzz2015(
        mut self,
        fuzz2015: impl Into<crate::testing::components::AffixFuzzer15>,
    ) -> Self {
        self.fuzz2015 = Some(fuzz2015.into());
        self
    }

    pub fn with_fuzz2016(
        mut self,
        fuzz2016: impl Into<crate::testing::components::AffixFuzzer16>,
    ) -> Self {
        self.fuzz2016 = Some(fuzz2016.into());
        self
    }

    pub fn with_fuzz2017(
        mut self,
        fuzz2017: impl Into<crate::testing::components::AffixFuzzer17>,
    ) -> Self {
        self.fuzz2017 = Some(fuzz2017.into());
        self
    }

    pub fn with_fuzz2018(
        mut self,
        fuzz2018: impl Into<crate::testing::components::AffixFuzzer18>,
    ) -> Self {
        self.fuzz2018 = Some(fuzz2018.into());
        self
    }
}
