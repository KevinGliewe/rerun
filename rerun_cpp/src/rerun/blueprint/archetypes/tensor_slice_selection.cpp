// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/tensor_slice_selection.fbs".

#include "tensor_slice_selection.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {
    TensorSliceSelection TensorSliceSelection::clear_fields() {
        auto archetype = TensorSliceSelection();
        archetype.width =
            ComponentBatch::empty<rerun::components::TensorWidthDimension>(Descriptor_width)
                .value_or_throw();
        archetype.height =
            ComponentBatch::empty<rerun::components::TensorHeightDimension>(Descriptor_height)
                .value_or_throw();
        archetype.indices = ComponentBatch::empty<rerun::components::TensorDimensionIndexSelection>(
                                Descriptor_indices
        )
                                .value_or_throw();
        archetype.slider =
            ComponentBatch::empty<rerun::blueprint::components::TensorDimensionIndexSlider>(
                Descriptor_slider
            )
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> TensorSliceSelection::columns(const Collection<uint32_t>& lengths_
    ) {
        std::vector<ComponentColumn> columns;
        columns.reserve(4);
        if (width.has_value()) {
            columns.push_back(width.value().partitioned(lengths_).value_or_throw());
        }
        if (height.has_value()) {
            columns.push_back(height.value().partitioned(lengths_).value_or_throw());
        }
        if (indices.has_value()) {
            columns.push_back(indices.value().partitioned(lengths_).value_or_throw());
        }
        if (slider.has_value()) {
            columns.push_back(slider.value().partitioned(lengths_).value_or_throw());
        }
        return columns;
    }

    Collection<ComponentColumn> TensorSliceSelection::columns() {
        if (width.has_value()) {
            return columns(std::vector<uint32_t>(width.value().length(), 1));
        }
        if (height.has_value()) {
            return columns(std::vector<uint32_t>(height.value().length(), 1));
        }
        if (indices.has_value()) {
            return columns(std::vector<uint32_t>(indices.value().length(), 1));
        }
        if (slider.has_value()) {
            return columns(std::vector<uint32_t>(slider.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::blueprint::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>>
        AsComponents<blueprint::archetypes::TensorSliceSelection>::as_batches(
            const blueprint::archetypes::TensorSliceSelection& archetype
        ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(4);

        if (archetype.width.has_value()) {
            cells.push_back(archetype.width.value());
        }
        if (archetype.height.has_value()) {
            cells.push_back(archetype.height.value());
        }
        if (archetype.indices.has_value()) {
            cells.push_back(archetype.indices.value());
        }
        if (archetype.slider.has_value()) {
            cells.push_back(archetype.slider.value());
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
