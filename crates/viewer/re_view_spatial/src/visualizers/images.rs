use re_types::{
    archetypes::Image,
    components::{DrawOrder, ImageBuffer, ImageFormat, Opacity},
    image::ImageKind,
    Component as _,
};
use re_view::HybridResults;
use re_viewer_context::{
    IdentifiedViewSystem, ImageInfo, MaybeVisualizableEntities, QueryContext,
    TypedComponentFallbackProvider, ViewContext, ViewContextCollection, ViewQuery,
    ViewSystemExecutionError, VisualizableEntities, VisualizableFilterContext, VisualizerQueryInfo,
    VisualizerSystem,
};

use crate::{
    contexts::SpatialSceneEntityContext,
    view_kind::SpatialViewKind,
    visualizers::{filter_visualizable_2d_entities, textured_rect_from_image},
    PickableRectSourceData, PickableTexturedRect,
};

use super::{entity_iterator::process_archetype, SpatialViewVisualizerData};

pub struct ImageVisualizer {
    pub data: SpatialViewVisualizerData,
}

impl Default for ImageVisualizer {
    fn default() -> Self {
        Self {
            data: SpatialViewVisualizerData::new(Some(SpatialViewKind::TwoD)),
        }
    }
}

struct ImageComponentData {
    image: ImageInfo,
    opacity: Option<Opacity>,
}

impl IdentifiedViewSystem for ImageVisualizer {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "Image".into()
    }
}

impl VisualizerSystem for ImageVisualizer {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<Image>()
    }

    fn filter_visualizable_entities(
        &self,
        entities: MaybeVisualizableEntities,
        context: &dyn VisualizableFilterContext,
    ) -> VisualizableEntities {
        re_tracing::profile_function!();
        filter_visualizable_2d_entities(entities, context)
    }

    fn execute(
        &mut self,
        ctx: &ViewContext<'_>,
        view_query: &ViewQuery<'_>,
        context_systems: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, ViewSystemExecutionError> {
        process_archetype::<Self, Image, _>(
            ctx,
            view_query,
            context_systems,
            |ctx, spatial_ctx, results| {
                self.process_image(ctx, results, spatial_ctx);
                Ok(())
            },
        )?;

        // TODO(#702): draw order is translated to depth offset, which works fine for opaque images,
        // but for everything with transparency, actual drawing order is still important.
        // We mitigate this a bit by at least sorting the images within each other.
        // Sorting of Images vs DepthImage vs SegmentationImage uses the fact that
        // visualizers are executed in the order of their identifiers.
        // -> The draw order is always DepthImage then Image then SegmentationImage,
        //    which happens to be exactly what we want 🙈
        self.data.pickable_rects.sort_by_key(|image| {
            (
                image.textured_rect.options.depth_offset,
                egui::emath::OrderedFloat(image.textured_rect.options.multiplicative_tint.a()),
            )
        });

        Ok(vec![PickableTexturedRect::to_draw_data(
            ctx.viewer_ctx.render_ctx,
            &self.data.pickable_rects,
        )?])
    }

    fn data(&self) -> Option<&dyn std::any::Any> {
        Some(self.data.as_any())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn fallback_provider(&self) -> &dyn re_viewer_context::ComponentFallbackProvider {
        self
    }
}

impl ImageVisualizer {
    fn process_image(
        &mut self,
        ctx: &QueryContext<'_>,
        results: &HybridResults<'_>,
        spatial_ctx: &SpatialSceneEntityContext<'_>,
    ) {
        use super::entity_iterator::{iter_component, iter_slices};
        use re_view::RangeResultsExt as _;

        let entity_path = ctx.target_entity_path;

        let Some(all_buffer_chunks) = results.get_required_chunks(&ImageBuffer::name()) else {
            return;
        };
        let Some(all_formats_chunks) = results.get_required_chunks(&ImageFormat::name()) else {
            return;
        };

        let timeline = ctx.query.timeline();
        let all_buffers_indexed =
            iter_slices::<&[u8]>(&all_buffer_chunks, timeline, ImageBuffer::name());
        let all_formats_indexed =
            iter_component::<ImageFormat>(&all_formats_chunks, timeline, ImageFormat::name());
        let all_opacities = results.iter_as(timeline, Opacity::name());

        let data = re_query::range_zip_1x2(
            all_buffers_indexed,
            all_formats_indexed,
            all_opacities.slice::<f32>(),
        )
        .filter_map(|(index, buffers, formats, opacities)| {
            let buffer = buffers.first()?;

            Some(ImageComponentData {
                image: ImageInfo {
                    buffer_row_id: index.1,
                    buffer: buffer.clone().into(),
                    format: first_copied(formats.as_deref())?.0,
                    kind: ImageKind::Color,
                },
                opacity: first_copied(opacities).map(Into::into),
            })
        });

        for ImageComponentData { image, opacity } in data {
            let opacity = opacity.unwrap_or_else(|| self.fallback_for(ctx));
            let multiplicative_tint =
                re_renderer::Rgba::from_white_alpha(opacity.0.clamp(0.0, 1.0));
            let colormap = None;

            if let Some(textured_rect) = textured_rect_from_image(
                ctx.viewer_ctx,
                entity_path,
                spatial_ctx,
                &image,
                colormap,
                multiplicative_tint,
                "Image",
                &mut self.data,
            ) {
                self.data.pickable_rects.push(PickableTexturedRect {
                    ent_path: entity_path.clone(),
                    textured_rect,
                    source_data: PickableRectSourceData::Image {
                        image,
                        depth_meter: None,
                    },
                });
            }
        }
    }
}

impl TypedComponentFallbackProvider<Opacity> for ImageVisualizer {
    fn fallback_for(&self, _ctx: &re_viewer_context::QueryContext<'_>) -> Opacity {
        1.0.into()
    }
}

impl TypedComponentFallbackProvider<DrawOrder> for ImageVisualizer {
    fn fallback_for(&self, _ctx: &QueryContext<'_>) -> DrawOrder {
        DrawOrder::DEFAULT_IMAGE
    }
}

re_viewer_context::impl_component_fallback_provider!(ImageVisualizer => [DrawOrder, Opacity]);

fn first_copied<T: Copy>(slice: Option<&[T]>) -> Option<T> {
    slice.and_then(|element| element.first()).copied()
}
