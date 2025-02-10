# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer23Ext" class in "affix_fuzzer23_ext.py".

from __future__ import annotations

from rerun._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)

from .. import datatypes

__all__ = ["AffixFuzzer23", "AffixFuzzer23Batch"]


class AffixFuzzer23(datatypes.MultiEnum, ComponentMixin):
    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of AffixFuzzer23Ext in affix_fuzzer23_ext.py

    # Note: there are no fields here because AffixFuzzer23 delegates to datatypes.MultiEnum
    pass


class AffixFuzzer23Batch(datatypes.MultiEnumBatch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.testing.components.AffixFuzzer23")


# This is patched in late to avoid circular dependencies.
AffixFuzzer23._BATCH_TYPE = AffixFuzzer23Batch  # type: ignore[assignment]
