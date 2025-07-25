// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/scalars.fbs".

#include "scalars.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Scalars Scalars::clear_fields() {
        auto archetype = Scalars();
        archetype.scalars =
            ComponentBatch::empty<rerun::components::Scalar>(Descriptor_scalars).value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> Scalars::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(1);
        if (scalars.has_value()) {
            columns.push_back(scalars.value().partitioned(lengths_).value_or_throw());
        }
        return columns;
    }

    Collection<ComponentColumn> Scalars::columns() {
        if (scalars.has_value()) {
            return columns(std::vector<uint32_t>(scalars.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>> AsComponents<archetypes::Scalars>::as_batches(
        const archetypes::Scalars& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(1);

        if (archetype.scalars.has_value()) {
            cells.push_back(archetype.scalars.value());
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun
