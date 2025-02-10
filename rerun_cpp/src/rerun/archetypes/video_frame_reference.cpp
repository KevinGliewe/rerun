// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/video_frame_reference.fbs".

#include "video_frame_reference.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    VideoFrameReference VideoFrameReference::clear_fields() {
        auto archetype = VideoFrameReference();
        archetype.timestamp =
            ComponentBatch::empty<rerun::components::VideoTimestamp>(Descriptor_timestamp)
                .value_or_throw();
        archetype.video_reference =
            ComponentBatch::empty<rerun::components::EntityPath>(Descriptor_video_reference)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> VideoFrameReference::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(3);
        if (timestamp.has_value()) {
            columns.push_back(timestamp.value().partitioned(lengths_).value_or_throw());
        }
        if (video_reference.has_value()) {
            columns.push_back(video_reference.value().partitioned(lengths_).value_or_throw());
        }
        columns.push_back(ComponentColumn::from_indicators<VideoFrameReference>(
                              static_cast<uint32_t>(lengths_.size())
        )
                              .value_or_throw());
        return columns;
    }

    Collection<ComponentColumn> VideoFrameReference::columns() {
        if (timestamp.has_value()) {
            return columns(std::vector<uint32_t>(timestamp.value().length(), 1));
        }
        if (video_reference.has_value()) {
            return columns(std::vector<uint32_t>(video_reference.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>> AsComponents<archetypes::VideoFrameReference>::as_batches(
        const archetypes::VideoFrameReference& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.timestamp.has_value()) {
            cells.push_back(archetype.timestamp.value());
        }
        if (archetype.video_reference.has_value()) {
            cells.push_back(archetype.video_reference.value());
        }
        {
            auto result = ComponentBatch::from_indicator<VideoFrameReference>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
