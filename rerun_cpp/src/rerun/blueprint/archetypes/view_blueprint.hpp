// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/view_blueprint.fbs".

#pragma once

#include "../../blueprint/components/view_class.hpp"
#include "../../blueprint/components/view_origin.hpp"
#include "../../collection.hpp"
#include "../../component_batch.hpp"
#include "../../component_column.hpp"
#include "../../components/name.hpp"
#include "../../components/visible.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::blueprint::archetypes {
    /// **Archetype**: The description of a single view.
    ///
    /// ⚠ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
    ///
    struct ViewBlueprint {
        /// The class of the view.
        std::optional<ComponentBatch> class_identifier;

        /// The name of the view.
        std::optional<ComponentBatch> display_name;

        /// The "anchor point" of this view.
        ///
        /// Defaults to the root path '/' if not specified.
        ///
        /// The transform at this path forms the reference point for all scene->world transforms in this view.
        /// I.e. the position of this entity path in space forms the origin of the coordinate system in this view.
        /// Furthermore, this is the primary indicator for heuristics on what entities we show in this view.
        std::optional<ComponentBatch> space_origin;

        /// Whether this view is visible.
        ///
        /// Defaults to true if not specified.
        std::optional<ComponentBatch> visible;

      public:
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.blueprint.archetypes.ViewBlueprint";

        /// `ComponentDescriptor` for the `class_identifier` field.
        static constexpr auto Descriptor_class_identifier = ComponentDescriptor(
            ArchetypeName, "ViewBlueprint:class_identifier",
            Loggable<rerun::blueprint::components::ViewClass>::ComponentType
        );
        /// `ComponentDescriptor` for the `display_name` field.
        static constexpr auto Descriptor_display_name = ComponentDescriptor(
            ArchetypeName, "ViewBlueprint:display_name",
            Loggable<rerun::components::Name>::ComponentType
        );
        /// `ComponentDescriptor` for the `space_origin` field.
        static constexpr auto Descriptor_space_origin = ComponentDescriptor(
            ArchetypeName, "ViewBlueprint:space_origin",
            Loggable<rerun::blueprint::components::ViewOrigin>::ComponentType
        );
        /// `ComponentDescriptor` for the `visible` field.
        static constexpr auto Descriptor_visible = ComponentDescriptor(
            ArchetypeName, "ViewBlueprint:visible",
            Loggable<rerun::components::Visible>::ComponentType
        );

      public:
        ViewBlueprint() = default;
        ViewBlueprint(ViewBlueprint&& other) = default;
        ViewBlueprint(const ViewBlueprint& other) = default;
        ViewBlueprint& operator=(const ViewBlueprint& other) = default;
        ViewBlueprint& operator=(ViewBlueprint&& other) = default;

        explicit ViewBlueprint(rerun::blueprint::components::ViewClass _class_identifier)
            : class_identifier(ComponentBatch::from_loggable(
                                   std::move(_class_identifier), Descriptor_class_identifier
              )
                                   .value_or_throw()) {}

        /// Update only some specific fields of a `ViewBlueprint`.
        static ViewBlueprint update_fields() {
            return ViewBlueprint();
        }

        /// Clear all the fields of a `ViewBlueprint`.
        static ViewBlueprint clear_fields();

        /// The class of the view.
        ViewBlueprint with_class_identifier(
            const rerun::blueprint::components::ViewClass& _class_identifier
        ) && {
            class_identifier =
                ComponentBatch::from_loggable(_class_identifier, Descriptor_class_identifier)
                    .value_or_throw();
            return std::move(*this);
        }

        /// The name of the view.
        ViewBlueprint with_display_name(const rerun::components::Name& _display_name) && {
            display_name = ComponentBatch::from_loggable(_display_name, Descriptor_display_name)
                               .value_or_throw();
            return std::move(*this);
        }

        /// The "anchor point" of this view.
        ///
        /// Defaults to the root path '/' if not specified.
        ///
        /// The transform at this path forms the reference point for all scene->world transforms in this view.
        /// I.e. the position of this entity path in space forms the origin of the coordinate system in this view.
        /// Furthermore, this is the primary indicator for heuristics on what entities we show in this view.
        ViewBlueprint with_space_origin(
            const rerun::blueprint::components::ViewOrigin& _space_origin
        ) && {
            space_origin = ComponentBatch::from_loggable(_space_origin, Descriptor_space_origin)
                               .value_or_throw();
            return std::move(*this);
        }

        /// Whether this view is visible.
        ///
        /// Defaults to true if not specified.
        ViewBlueprint with_visible(const rerun::components::Visible& _visible) && {
            visible = ComponentBatch::from_loggable(_visible, Descriptor_visible).value_or_throw();
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
    struct AsComponents<blueprint::archetypes::ViewBlueprint> {
        /// Serialize all set component batches.
        static Result<Collection<ComponentBatch>> as_batches(
            const blueprint::archetypes::ViewBlueprint& archetype
        );
    };
} // namespace rerun
