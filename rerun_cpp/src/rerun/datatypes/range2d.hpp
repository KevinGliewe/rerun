// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/range2d.fbs".

#pragma once

#include "../component_descriptor.hpp"
#include "../result.hpp"
#include "range1d.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: An Axis-Aligned Bounding Box in 2D space, implemented as the minimum and maximum corners.
    struct Range2D {
        /// The range of the X-axis (usually left and right bounds).
        rerun::datatypes::Range1D x_range;

        /// The range of the Y-axis (usually top and bottom bounds).
        rerun::datatypes::Range1D y_range;

      public:
        Range2D() = default;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Range2D> {
        static constexpr ComponentDescriptor Descriptor = "rerun.datatypes.Range2D";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Range2D` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Range2D* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const datatypes::Range2D* elements, size_t num_elements
        );
    };
} // namespace rerun
