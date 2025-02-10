# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/uint16.fbs".

# You can extend this class by creating a "UInt16Ext" class in "uint16_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)

__all__ = ["UInt16", "UInt16ArrayLike", "UInt16Batch", "UInt16Like"]


@define(init=False)
class UInt16:
    """**Datatype**: A 16bit unsigned integer."""

    def __init__(self: Any, value: UInt16Like):
        """Create a new instance of the UInt16 datatype."""

        # You can define your own __init__ function as a member of UInt16Ext in uint16_ext.py
        self.__attrs_init__(value=value)

    value: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of UInt16Ext in uint16_ext.py
        return np.asarray(self.value, dtype=dtype)

    def __int__(self) -> int:
        return int(self.value)

    def __hash__(self) -> int:
        return hash(self.value)


if TYPE_CHECKING:
    UInt16Like = Union[UInt16, int]
else:
    UInt16Like = Any

UInt16ArrayLike = Union[UInt16, Sequence[UInt16Like], int, npt.NDArray[np.uint16]]


class UInt16Batch(BaseBatch[UInt16ArrayLike]):
    _ARROW_DATATYPE = pa.uint16()

    @staticmethod
    def _native_to_pa_array(data: UInt16ArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.uint16).flatten()
        return pa.array(array, type=data_type)
