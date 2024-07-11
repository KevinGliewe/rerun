// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/annotation_context.fbs".

#pragma once

#include "../collection.hpp"
#include "../datatypes/class_description_map_elem.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <type_traits>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class ListBuilder;
} // namespace arrow

namespace rerun::components {
    /// **Component**: The annotation context provides additional information on how to display entities.
    ///
    /// Entities can use `datatypes::ClassId`s and `datatypes::KeypointId`s to provide annotations, and
    /// the labels and colors will be looked up in the appropriate
    /// annotation context. We use the *first* annotation context we find in the
    /// path-hierarchy when searching up through the ancestors of a given entity
    /// path.
    struct AnnotationContext {
        /// List of class descriptions, mapping class indices to class names, colors etc.
        rerun::Collection<rerun::datatypes::ClassDescriptionMapElem> class_map;

      public:
        // Extensions to generated type defined in 'annotation_context_ext.cpp'

        /// Construct from an initializer list of elements from which `rerun::datatypes::ClassDescriptionMapElem`s can be constructed.
        ///
        /// This will then create a new collection of `rerun::datatypes::ClassDescriptionMapElem`.
        ///
        /// _Implementation note_:
        /// We handle this type of conversion in a generic `rerun::ContainerAdapter`.
        /// However, it is *still* necessary since initializer list overload resolution is handled
        /// in a special way by the compiler, making this case not being covered by the general container case.
        template <
            typename TElement, //
            typename = std::enable_if_t<
                std::is_constructible_v<datatypes::ClassDescriptionMapElem, TElement>> //
            >
        AnnotationContext(std::initializer_list<TElement> class_descriptions) {
            std::vector<datatypes::ClassDescriptionMapElem> class_map_new;
            class_map_new.reserve(class_descriptions.size());
            for (const auto& class_description : class_descriptions) {
                class_map_new.emplace_back(std::move(class_description));
            }
            class_map = Collection<datatypes::ClassDescriptionMapElem>::take_ownership(
                std::move(class_map_new)
            );
        }

      public:
        AnnotationContext() = default;

        AnnotationContext(rerun::Collection<rerun::datatypes::ClassDescriptionMapElem> class_map_)
            : class_map(std::move(class_map_)) {}

        AnnotationContext& operator=(
            rerun::Collection<rerun::datatypes::ClassDescriptionMapElem> class_map_
        ) {
            class_map = std::move(class_map_);
            return *this;
        }
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::AnnotationContext> {
        static constexpr const char Name[] = "rerun.components.AnnotationContext";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::components::AnnotationContext` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::AnnotationContext* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::ListBuilder* builder, const components::AnnotationContext* elements,
            size_t num_elements
        );
    };
} // namespace rerun
