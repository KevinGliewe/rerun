# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/scalar.fbs".

# You can extend this class by creating a "ScalarExt" class in "scalar_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["Scalar", "ScalarBatch"]


class Scalar(datatypes.Float64, ComponentMixin):
    """
    **Component**: A scalar value, encoded as a 64-bit floating point.

    Used for time series plots.
    """

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of ScalarExt in scalar_ext.py

    # Note: there are no fields here because Scalar delegates to datatypes.Float64


class ScalarBatch(datatypes.Float64Batch, ComponentBatchMixin):
    _COMPONENT_TYPE: str = "rerun.components.Scalar"


# This is patched in late to avoid circular dependencies.
Scalar._BATCH_TYPE = ScalarBatch  # type: ignore[assignment]
