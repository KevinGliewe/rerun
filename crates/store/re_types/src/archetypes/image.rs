// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/image.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A monochrome or color image.
///
/// See also [`archetypes::DepthImage`][crate::archetypes::DepthImage] and [`archetypes::SegmentationImage`][crate::archetypes::SegmentationImage].
///
/// Rerun also supports compressed images (JPEG, PNG, …), using [`archetypes::EncodedImage`][crate::archetypes::EncodedImage].
/// For images that refer to video frames see [`archetypes::VideoFrameReference`][crate::archetypes::VideoFrameReference].
/// Compressing images or using video data instead can save a lot of bandwidth and memory.
///
/// The raw image data is stored as a single buffer of bytes in a [`components::Blob`][crate::components::Blob].
/// The meaning of these bytes is determined by the [`components::ImageFormat`][crate::components::ImageFormat] which specifies the resolution
/// and the pixel format (e.g. RGB, RGBA, …).
///
/// The order of dimensions in the underlying [`components::Blob`][crate::components::Blob] follows the typical
/// row-major, interleaved-pixel image format.
///
/// ## Examples
///
/// ### `image_simple`:
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_image").spawn()?;
///
///     let mut image = Array::<u8, _>::zeros((200, 300, 3).f());
///     image.slice_mut(s![.., .., 0]).fill(255);
///     image.slice_mut(s![50..150, 50..150, 0]).fill(0);
///     image.slice_mut(s![50..150, 50..150, 1]).fill(255);
///
///     rec.log(
///         "image",
///         &rerun::Image::from_color_model_and_tensor(rerun::ColorModel::RGB, image)?,
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1200w.png">
///   <img src="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Logging images with various formats
/// ```ignore
/// use rerun::external::ndarray;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_image_formats").spawn()?;
///
///     // Simple gradient image
///     let image = ndarray::Array3::from_shape_fn((256, 256, 3), |(y, x, c)| match c {
///         0 => x as u8,
///         1 => (x + y).min(255) as u8,
///         2 => y as u8,
///         _ => unreachable!(),
///     });
///
///     // RGB image
///     rec.log(
///         "image_rgb",
///         &rerun::Image::from_color_model_and_tensor(rerun::ColorModel::RGB, image.clone())?,
///     )?;
///
///     // Green channel only (Luminance)
///     rec.log(
///         "image_green_only",
///         &rerun::Image::from_color_model_and_tensor(
///             rerun::ColorModel::L,
///             image.slice(ndarray::s![.., .., 1]).to_owned(),
///         )?,
///     )?;
///
///     // BGR image
///     rec.log(
///         "image_bgr",
///         &rerun::Image::from_color_model_and_tensor(
///             rerun::ColorModel::BGR,
///             image.slice(ndarray::s![.., .., ..;-1]).to_owned(),
///         )?,
///     )?;
///
///     // New image with Separate Y/U/V planes with 4:2:2 chroma downsampling
///     let mut yuv_bytes = Vec::with_capacity(256 * 256 + 128 * 256 * 2);
///     yuv_bytes.extend(std::iter::repeat(128).take(256 * 256)); // Fixed value for Y.
///     yuv_bytes.extend((0..256).flat_map(|_y| (0..128).map(|x| x * 2))); // Gradient for U.
///     yuv_bytes.extend((0..256).flat_map(|y| std::iter::repeat(y as u8).take(128))); // Gradient for V.
///     rec.log(
///         "image_yuv422",
///         &rerun::Image::from_pixel_format(
///             [256, 256],
///             rerun::PixelFormat::Y_U_V16_FullRange,
///             yuv_bytes,
///         ),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/image_formats/182a233fb4d0680eb31912a82f328ddaaa66324e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/image_formats/182a233fb4d0680eb31912a82f328ddaaa66324e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/image_formats/182a233fb4d0680eb31912a82f328ddaaa66324e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/image_formats/182a233fb4d0680eb31912a82f328ddaaa66324e/1200w.png">
///   <img src="https://static.rerun.io/image_formats/182a233fb4d0680eb31912a82f328ddaaa66324e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Image {
    /// The raw image data.
    pub buffer: Option<SerializedComponentBatch>,

    /// The format of the image.
    pub format: Option<SerializedComponentBatch>,

    /// Opacity of the image, useful for layering several images.
    ///
    /// Defaults to 1.0 (fully opaque).
    pub opacity: Option<SerializedComponentBatch>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<SerializedComponentBatch>,
}

impl Image {
    /// Returns the [`ComponentDescriptor`] for [`Self::buffer`].
    #[inline]
    pub fn descriptor_buffer() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Image".into()),
            component_name: "rerun.components.ImageBuffer".into(),
            archetype_field_name: Some("buffer".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::format`].
    #[inline]
    pub fn descriptor_format() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Image".into()),
            component_name: "rerun.components.ImageFormat".into(),
            archetype_field_name: Some("format".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::opacity`].
    #[inline]
    pub fn descriptor_opacity() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Image".into()),
            component_name: "rerun.components.Opacity".into(),
            archetype_field_name: Some("opacity".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Image".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Image".into()),
            component_name: "rerun.components.ImageIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| [Image::descriptor_buffer(), Image::descriptor_format()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Image::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| [Image::descriptor_opacity(), Image::descriptor_draw_order()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Image::descriptor_buffer(),
            Image::descriptor_format(),
            Image::descriptor_indicator(),
            Image::descriptor_opacity(),
            Image::descriptor_draw_order(),
        ]
    });

impl Image {
    /// The total number of components in the archetype: 2 required, 1 recommended, 2 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`Image`] [`::re_types_core::Archetype`]
pub type ImageIndicator = ::re_types_core::GenericIndicatorComponent<Image>;

impl ::re_types_core::Archetype for Image {
    type Indicator = ImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Image".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Image"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        ImageIndicator::DEFAULT.serialized().unwrap()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let buffer = arrays_by_descr
            .get(&Self::descriptor_buffer())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_buffer()));
        let format = arrays_by_descr
            .get(&Self::descriptor_format())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_format()));
        let opacity = arrays_by_descr
            .get(&Self::descriptor_opacity())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_opacity()));
        let draw_order = arrays_by_descr
            .get(&Self::descriptor_draw_order())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_draw_order())
            });
        Ok(Self {
            buffer,
            format,
            opacity,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for Image {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.buffer.clone(),
            self.format.clone(),
            self.opacity.clone(),
            self.draw_order.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Image {}

impl Image {
    /// Create a new `Image`.
    #[inline]
    pub fn new(
        buffer: impl Into<crate::components::ImageBuffer>,
        format: impl Into<crate::components::ImageFormat>,
    ) -> Self {
        Self {
            buffer: try_serialize_field(Self::descriptor_buffer(), [buffer]),
            format: try_serialize_field(Self::descriptor_format(), [format]),
            opacity: None,
            draw_order: None,
        }
    }

    /// Update only some specific fields of a `Image`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `Image`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            buffer: Some(SerializedComponentBatch::new(
                crate::components::ImageBuffer::arrow_empty(),
                Self::descriptor_buffer(),
            )),
            format: Some(SerializedComponentBatch::new(
                crate::components::ImageFormat::arrow_empty(),
                Self::descriptor_format(),
            )),
            opacity: Some(SerializedComponentBatch::new(
                crate::components::Opacity::arrow_empty(),
                Self::descriptor_opacity(),
            )),
            draw_order: Some(SerializedComponentBatch::new(
                crate::components::DrawOrder::arrow_empty(),
                Self::descriptor_draw_order(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [
            self.buffer
                .map(|buffer| buffer.partitioned(_lengths.clone()))
                .transpose()?,
            self.format
                .map(|format| format.partitioned(_lengths.clone()))
                .transpose()?,
            self.opacity
                .map(|opacity| opacity.partitioned(_lengths.clone()))
                .transpose()?,
            self.draw_order
                .map(|draw_order| draw_order.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        Ok(columns
            .into_iter()
            .flatten()
            .chain([::re_types_core::indicator_column::<Self>(
                _lengths.into_iter().count(),
            )?]))
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_buffer = self.buffer.as_ref().map(|b| b.array.len());
        let len_format = self.format.as_ref().map(|b| b.array.len());
        let len_opacity = self.opacity.as_ref().map(|b| b.array.len());
        let len_draw_order = self.draw_order.as_ref().map(|b| b.array.len());
        let len = None
            .or(len_buffer)
            .or(len_format)
            .or(len_opacity)
            .or(len_draw_order)
            .unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    /// The raw image data.
    #[inline]
    pub fn with_buffer(mut self, buffer: impl Into<crate::components::ImageBuffer>) -> Self {
        self.buffer = try_serialize_field(Self::descriptor_buffer(), [buffer]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ImageBuffer`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_buffer`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_buffer(
        mut self,
        buffer: impl IntoIterator<Item = impl Into<crate::components::ImageBuffer>>,
    ) -> Self {
        self.buffer = try_serialize_field(Self::descriptor_buffer(), buffer);
        self
    }

    /// The format of the image.
    #[inline]
    pub fn with_format(mut self, format: impl Into<crate::components::ImageFormat>) -> Self {
        self.format = try_serialize_field(Self::descriptor_format(), [format]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ImageFormat`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_format`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_format(
        mut self,
        format: impl IntoIterator<Item = impl Into<crate::components::ImageFormat>>,
    ) -> Self {
        self.format = try_serialize_field(Self::descriptor_format(), format);
        self
    }

    /// Opacity of the image, useful for layering several images.
    ///
    /// Defaults to 1.0 (fully opaque).
    #[inline]
    pub fn with_opacity(mut self, opacity: impl Into<crate::components::Opacity>) -> Self {
        self.opacity = try_serialize_field(Self::descriptor_opacity(), [opacity]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Opacity`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_opacity`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_opacity(
        mut self,
        opacity: impl IntoIterator<Item = impl Into<crate::components::Opacity>>,
    ) -> Self {
        self.opacity = try_serialize_field(Self::descriptor_opacity(), opacity);
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = try_serialize_field(Self::descriptor_draw_order(), [draw_order]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::DrawOrder`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_draw_order`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_draw_order(
        mut self,
        draw_order: impl IntoIterator<Item = impl Into<crate::components::DrawOrder>>,
    ) -> Self {
        self.draw_order = try_serialize_field(Self::descriptor_draw_order(), draw_order);
        self
    }
}

impl ::re_byte_size::SizeBytes for Image {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.buffer.heap_size_bytes()
            + self.format.heap_size_bytes()
            + self.opacity.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }
}
