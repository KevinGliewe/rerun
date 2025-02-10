# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/mat4x4.fbs".

# You can extend this class by creating a "Mat4x4Ext" class in "mat4x4_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)
from .._converters import (
    to_np_float32,
)
from .mat4x4_ext import Mat4x4Ext

__all__ = ["Mat4x4", "Mat4x4ArrayLike", "Mat4x4Batch", "Mat4x4Like"]


@define(init=False)
class Mat4x4(Mat4x4Ext):
    """
    **Datatype**: A 4x4 Matrix.

    Matrices in Rerun are stored as flat list of coefficients in column-major order:
    ```text
               column 0         column 1         column 2         column 3
           --------------------------------------------------------------------
    row 0 | flat_columns[0]  flat_columns[4]  flat_columns[8]  flat_columns[12]
    row 1 | flat_columns[1]  flat_columns[5]  flat_columns[9]  flat_columns[13]
    row 2 | flat_columns[2]  flat_columns[6]  flat_columns[10] flat_columns[14]
    row 3 | flat_columns[3]  flat_columns[7]  flat_columns[11] flat_columns[15]
    ```

    However, construction is done from a list of rows, which follows NumPy's convention:
    ```python
    np.testing.assert_array_equal(
        rr.datatypes.Mat4x4([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).flat_columns,
        np.array([1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15, 4, 8, 12, 16], dtype=np.float32),
    )
    np.testing.assert_array_equal(
        rr.datatypes.Mat4x4([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]).flat_columns,
        np.array([1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15, 4, 8, 12, 16], dtype=np.float32),
    )
    ```
    If you want to construct a matrix from a list of columns instead, use the named `columns` parameter:
    ```python
    np.testing.assert_array_equal(
        rr.datatypes.Mat4x4(columns=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).flat_columns,
        np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], dtype=np.float32),
    )
    np.testing.assert_array_equal(
        rr.datatypes.Mat4x4(columns=[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]).flat_columns,
        np.array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], dtype=np.float32),
    )
    ```
    """

    # __init__ can be found in mat4x4_ext.py

    flat_columns: npt.NDArray[np.float32] = field(converter=to_np_float32)
    # Flat list of matrix coefficients in column-major order.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of Mat4x4Ext in mat4x4_ext.py
        return np.asarray(self.flat_columns, dtype=dtype)


if TYPE_CHECKING:
    Mat4x4Like = Union[Mat4x4, npt.ArrayLike]
else:
    Mat4x4Like = Any

Mat4x4ArrayLike = Union[
    Mat4x4,
    Sequence[Mat4x4Like],
]


class Mat4x4Batch(BaseBatch[Mat4x4ArrayLike]):
    _ARROW_DATATYPE = pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 16)

    @staticmethod
    def _native_to_pa_array(data: Mat4x4ArrayLike, data_type: pa.DataType) -> pa.Array:
        return Mat4x4Ext.native_to_pa_array_override(data, data_type)
