# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/float64.fbs".

# You can extend this class by creating a "Float64Ext" class in "float64_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)

__all__ = ["Float64", "Float64ArrayLike", "Float64Batch", "Float64Like"]


@define(init=False)
class Float64:
    """**Datatype**: A double-precision 64-bit IEEE 754 floating point number."""

    def __init__(self: Any, value: Float64Like):
        """Create a new instance of the Float64 datatype."""

        # You can define your own __init__ function as a member of Float64Ext in float64_ext.py
        self.__attrs_init__(value=value)

    value: float = field(converter=float)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of Float64Ext in float64_ext.py
        return np.asarray(self.value, dtype=dtype)

    def __float__(self) -> float:
        return float(self.value)

    def __hash__(self) -> int:
        return hash(self.value)


if TYPE_CHECKING:
    Float64Like = Union[Float64, float]
else:
    Float64Like = Any

Float64ArrayLike = Union[
    Float64, Sequence[Float64Like], npt.NDArray[Any], npt.ArrayLike, Sequence[Sequence[float]], Sequence[float]
]


class Float64Batch(BaseBatch[Float64ArrayLike]):
    _ARROW_DATATYPE = pa.float64()

    @staticmethod
    def _native_to_pa_array(data: Float64ArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.float64).flatten()
        return pa.array(array, type=data_type)
