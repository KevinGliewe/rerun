// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/map_background.fbs".

#pragma once

#include "../../blueprint/components/map_provider.hpp"
#include "../../collection.hpp"
#include "../../component_batch.hpp"
#include "../../component_column.hpp"
#include "../../indicator_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: Configuration for the background map of the map view.
    struct MapBackground {
        /// Map provider and style to use.
        ///
        /// **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.
        std::optional<ComponentBatch> provider;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.blueprint.components.MapBackgroundIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.blueprint.archetypes.MapBackground";

        /// `ComponentDescriptor` for the `provider` field.
        static constexpr auto Descriptor_provider = ComponentDescriptor(
            ArchetypeName, "provider",
            Loggable<rerun::blueprint::components::MapProvider>::Descriptor.component_name
        );

      public:
        MapBackground() = default;
        MapBackground(MapBackground&& other) = default;
        MapBackground(const MapBackground& other) = default;
        MapBackground& operator=(const MapBackground& other) = default;
        MapBackground& operator=(MapBackground&& other) = default;

        explicit MapBackground(rerun::blueprint::components::MapProvider _provider)
            : provider(ComponentBatch::from_loggable(std::move(_provider), Descriptor_provider)
                           .value_or_throw()) {}

        /// Update only some specific fields of a `MapBackground`.
        static MapBackground update_fields() {
            return MapBackground();
        }

        /// Clear all the fields of a `MapBackground`.
        static MapBackground clear_fields();

        /// Map provider and style to use.
        ///
        /// **Note**: Requires a Mapbox API key in the `RERUN_MAPBOX_ACCESS_TOKEN` environment variable.
        MapBackground with_provider(const rerun::blueprint::components::MapProvider& _provider) && {
            provider =
                ComponentBatch::from_loggable(_provider, Descriptor_provider).value_or_throw();
            return std::move(*this);
        }

        /// Partitions the component data into multiple sub-batches.
        ///
        /// Specifically, this transforms the existing `ComponentBatch` data into `ComponentColumn`s
        /// instead, via `ComponentBatch::partitioned`.
        ///
        /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
        ///
        /// The specified `lengths` must sum to the total length of the component batch.
        Collection<ComponentColumn> columns(const Collection<uint32_t>& lengths_);

        /// Partitions the component data into unit-length sub-batches.
        ///
        /// This is semantically similar to calling `columns` with `std::vector<uint32_t>(n, 1)`,
        /// where `n` is automatically guessed.
        Collection<ComponentColumn> columns();
    };

} // namespace rerun::blueprint::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<blueprint::archetypes::MapBackground> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(
            const blueprint::archetypes::MapBackground& archetype
        );
    };
} // namespace rerun
