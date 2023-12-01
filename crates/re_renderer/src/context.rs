use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use type_map::concurrent::{self, TypeMap};

use crate::{
    allocator::{CpuWriteGpuReadBelt, GpuReadbackBelt},
    config::{DeviceTier, RenderContextConfig},
    global_bindings::GlobalBindings,
    renderer::Renderer,
    resource_managers::{MeshManager, TextureManager2D},
    wgpu_resources::WgpuResourcePools,
    FileResolver, FileServer, FileSystem, RecommendedFileResolver,
};

/// Any resource involving wgpu rendering which can be re-used across different scenes.
/// I.e. render pipelines, resource pools, etc.
pub struct RenderContext {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    pub(crate) shared_renderer_data: SharedRendererData,
    pub(crate) renderers: RwLock<Renderers>,
    pub(crate) resolver: RecommendedFileResolver,
    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
    pub(crate) err_tracker: std::sync::Arc<crate::error_tracker::ErrorTracker>,

    pub mesh_manager: RwLock<MeshManager>,
    pub texture_manager_2d: TextureManager2D,
    pub(crate) cpu_write_gpu_read_belt: Mutex<CpuWriteGpuReadBelt>,
    pub(crate) gpu_readback_belt: Mutex<GpuReadbackBelt>,

    /// List of unfinished queue submission via this context.
    ///
    /// This is currently only about submissions we do via the global encoder in [`ActiveFrameContext`]
    /// TODO(andreas): We rely on egui for the "primary" submissions in re_viewer. It would be nice to take full control over all submissions.
    inflight_queue_submissions: Vec<wgpu::SubmissionIndex>,

    pub active_frame: ActiveFrameContext,

    pub gpu_resources: WgpuResourcePools, // Last due to drop order.
}

/// Immutable data that is shared between all [`Renderer`]
pub struct SharedRendererData {
    pub(crate) config: RenderContextConfig,

    /// Global bindings, always bound to 0 bind group slot zero.
    /// [`Renderer`] are not allowed to use bind group 0 themselves!
    pub(crate) global_bindings: GlobalBindings,
}

/// Struct owning *all* [`Renderer`].
/// [`Renderer`] are created lazily and stay around indefinitely.
pub(crate) struct Renderers {
    renderers: concurrent::TypeMap,
}

impl Renderers {
    pub fn get_or_create<Fs: FileSystem, R: 'static + Renderer + Send + Sync>(
        &mut self,
        shared_data: &SharedRendererData,
        resource_pools: &mut WgpuResourcePools,
        device: &wgpu::Device,
        resolver: &mut FileResolver<Fs>,
    ) -> &R {
        self.renderers.entry().or_insert_with(|| {
            re_tracing::profile_scope!("create_renderer", std::any::type_name::<R>());
            R::create_renderer(shared_data, resource_pools, device, resolver)
        })
    }

    pub fn get<R: 'static + Renderer>(&self) -> Option<&R> {
        self.renderers.get::<R>()
    }
}

impl RenderContext {
    /// Chunk size for our cpu->gpu buffer manager.
    ///
    /// 32MiB chunk size (as big as a for instance a 2048x1024 float4 texture)
    /// (it's tempting to use something smaller on Web, but this may just cause more
    /// buffers to be allocated the moment we want to upload a bigger chunk)
    const CPU_WRITE_GPU_READ_BELT_DEFAULT_CHUNK_SIZE: Option<wgpu::BufferSize> =
        wgpu::BufferSize::new(1024 * 1024 * 32);

    /// Chunk size for our gpu->cpu buffer manager.
    ///
    /// We expect large screenshots to be rare occurrences, so we go with fairly small chunks of just 64 kiB.
    /// (this is as much memory as a 128x128 rgba8 texture, or a little bit less than a 64x64 picking target with depth)
    /// I.e. screenshots will end up in dedicated chunks.
    const GPU_READBACK_BELT_DEFAULT_CHUNK_SIZE: Option<wgpu::BufferSize> =
        wgpu::BufferSize::new(1024 * 64);

    /// Limit maximum number of in flight submissions to this number.
    ///
    /// By limiting the number of submissions we have on the queue we ensure that GPU stalls do not
    /// cause us to request more and more memory to prepare more and more submissions.
    ///
    /// Note that this is only indirectly related to number of buffered frames,
    /// since buffered frames/blit strategy are all about the display<->gpu interface,
    /// whereas this is about a *particular aspect* of the cpu<->gpu interface.
    ///
    /// Should be somewhere between 1-4, too high and we use up more memory and introduce latency,
    /// too low and we may starve the GPU.
    const MAX_NUM_INFLIGHT_QUEUE_SUBMISSIONS: usize = 4;

    pub fn new(
        adapter: &wgpu::Adapter,
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        config: RenderContextConfig,
    ) -> Self {
        re_tracing::profile_function!();

        log_adapter_info(&adapter.get_info());

        let mut gpu_resources = WgpuResourcePools::default();
        let global_bindings = GlobalBindings::new(&mut gpu_resources, &device);

        // Validate capabilities of the device.
        assert!(
            config.device_caps.limits().check_limits(&device.limits()),
            "The given device doesn't support the required limits for the given hardware caps {:?}.
            Required:
            {:?}
            Actual:
            {:?}",
            config.device_caps,
            config.device_caps.limits(),
            device.limits(),
        );
        assert!(
            device.features().contains(config.device_caps.features()),
            "The given device doesn't support the required features for the given hardware caps {:?}.
            Required:
            {:?}
            Actual:
            {:?}",
            config.device_caps,
            config.device_caps.features(),
            device.features(),
        );
        assert!(adapter.get_downlevel_capabilities().flags.contains(config.device_caps.required_downlevel_capabilities().flags),
            "The given device doesn't support the required downlevel capabilities for the given hardware caps {:?}.
            Required:
            {:?}
            Actual:
            {:?}",
            config.device_caps,
            config.device_caps.required_downlevel_capabilities(),
            adapter.get_downlevel_capabilities(),
        );

        // In debug builds, make sure to catch all errors, never crash, and try to
        // always let the user find a way to return a poisoned pipeline back into a
        // sane state.
        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
        let err_tracker = {
            let err_tracker = std::sync::Arc::new(crate::error_tracker::ErrorTracker::default());
            device.on_uncaptured_error({
                let err_tracker = std::sync::Arc::clone(&err_tracker);
                Box::new(move |err| err_tracker.handle_error(err))
            });
            err_tracker
        };

        let shared_renderer_data = SharedRendererData {
            config,
            global_bindings,
        };

        let mut resolver = crate::new_recommended_file_resolver();
        let mut renderers = RwLock::new(Renderers {
            renderers: TypeMap::new(),
        });

        let mesh_manager = RwLock::new(MeshManager::new(renderers.get_mut().get_or_create(
            &shared_renderer_data,
            &mut gpu_resources,
            &device,
            &mut resolver,
        )));
        let texture_manager_2d =
            TextureManager2D::new(device.clone(), queue.clone(), &gpu_resources.textures);

        let active_frame = ActiveFrameContext {
            before_view_builder_encoder: Mutex::new(FrameGlobalCommandEncoder::new(&device)),
            per_frame_data_helper: TypeMap::new(),
            frame_index: 0,
        };

        // Register shader workarounds for the current device.
        if adapter.get_info().backend == wgpu::Backend::BrowserWebGpu {
            // Chrome/Tint does not support `@invariant` when targeting Metal.
            // https://bugs.chromium.org/p/chromium/issues/detail?id=1439273
            // (bug is fixed as of writing, but hasn't hit any public released version yet)
            // Ignoring it is fine in the cases we use it, it's mostly there to avoid a (correct!) warning in wgpu.
            gpu_resources
                .shader_modules
                .shader_text_workaround_replacements
                .push((
                    "@invariant @builtin(position)".to_owned(),
                    "@builtin(position)".to_owned(),
                ));
        }

        RenderContext {
            device,
            queue,

            shared_renderer_data,

            renderers,

            gpu_resources,

            mesh_manager,
            texture_manager_2d,
            cpu_write_gpu_read_belt: Mutex::new(CpuWriteGpuReadBelt::new(Self::CPU_WRITE_GPU_READ_BELT_DEFAULT_CHUNK_SIZE.unwrap())),
            gpu_readback_belt: Mutex::new(GpuReadbackBelt::new(Self::GPU_READBACK_BELT_DEFAULT_CHUNK_SIZE.unwrap())),

            resolver,

            #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
            err_tracker,

            inflight_queue_submissions: Vec::new(),

            active_frame,
        }
    }

    fn poll_device(&mut self) {
        re_tracing::profile_function!();

        // Browsers don't let us wait for GPU work via `poll`:
        //
        // * WebGPU: `poll` is a no-op as the spec doesn't specify it at all. Calling it doesn't hurt though.
        //
        // * WebGL: Internal timeout can't go above a browser specific value.
        //          Since wgpu ran into issues in the past with some browsers returning errors,
        //          it uses a timeout of zero and ignores errors there.
        //
        //          This causes unused buffers to be freed immediately, which is wrong but also doesn't hurt
        //          since WebGL doesn't care about freeing buffers/textures that are still in use.
        //          Meaning, that from our POV we're actually freeing cpu memory that we wanted to free anyways.
        //          *More importantly this means that we get buffers from the staging belts back earlier!*
        //          Therefore, we just always "block" instead on WebGL to free as early as possible,
        //          knowing that we're not _actually_ blocking.
        //
        //          For more details check https://github.com/gfx-rs/wgpu/issues/3601
        if cfg!(target_arch = "wasm32")
            && self.shared_renderer_data.config.device_caps.tier == DeviceTier::Gles
        {
            self.device.poll(wgpu::Maintain::Wait);
            return;
        }

        // Ensure not too many queue submissions are in flight.
        let num_submissions_to_wait_for = self
            .inflight_queue_submissions
            .len()
            .saturating_sub(Self::MAX_NUM_INFLIGHT_QUEUE_SUBMISSIONS);

        if let Some(newest_submission_to_wait_for) = self
            .inflight_queue_submissions
            .drain(0..num_submissions_to_wait_for)
            .last()
        {
            self.device.poll(wgpu::Maintain::WaitForSubmissionIndex(
                newest_submission_to_wait_for,
            ));
        }
    }

    /// Call this at the beginning of a new frame.
    ///
    /// Updates internal book-keeping, frame allocators and executes delayed events like shader reloading.
    pub fn begin_frame(&mut self) {
        re_tracing::profile_function!();

        // If the currently active frame still has an encoder, we need to finish it and queue it.
        // This should only ever happen for the first frame where we created an encoder for preparatory work. Every other frame we take the encoder at submit!
        if self
            .active_frame
            .before_view_builder_encoder
            .lock()
            .0
            .is_some()
        {
            assert!(self.active_frame.frame_index == 0, "There was still a command encoder from the previous frame at the beginning of the current. Did you forget to call RenderContext::before_submit?");
            self.before_submit();
        }

        // Request write used staging buffer back.
        // TODO(andreas): If we'd control all submissions, we could move this directly after the submission which would be a bit better.
        self.cpu_write_gpu_read_belt.get_mut().after_queue_submit();
        // Map all read staging buffers.
        self.gpu_readback_belt.get_mut().after_queue_submit();

        self.active_frame = ActiveFrameContext {
            before_view_builder_encoder: Mutex::new(FrameGlobalCommandEncoder::new(&self.device)),
            frame_index: self.active_frame.frame_index + 1,
            per_frame_data_helper: TypeMap::new(),
        };
        let frame_index = self.active_frame.frame_index;

        // Tick the error tracker so that it knows when to reset!
        // Note that we're ticking on begin_frame rather than raw frames, which
        // makes a world of difference when we're in a poisoned state.
        #[cfg(all(not(target_arch = "wasm32"), debug_assertions))] // native debug build
        self.err_tracker.tick();

        // The set of files on disk that were modified in any way since last frame,
        // ignoring deletions.
        // Always an empty set in release builds.
        let modified_paths = FileServer::get_mut(|fs| fs.collect(&mut self.resolver));
        if !modified_paths.is_empty() {
            re_log::debug!(?modified_paths, "got some filesystem events");
        }

        self.mesh_manager.get_mut().begin_frame(frame_index);
        self.texture_manager_2d.begin_frame(frame_index);
        self.gpu_readback_belt.get_mut().begin_frame(frame_index);

        {
            let WgpuResourcePools {
                bind_group_layouts,
                bind_groups,
                pipeline_layouts,
                render_pipelines,
                samplers,
                shader_modules,
                textures,
                buffers,
            } = &mut self.gpu_resources; // not all pools require maintenance

            // Shader module maintenance must come before render pipelines because render pipeline
            // recompilation picks up all shaders that have been recompiled this frame.
            shader_modules.begin_frame(
                &self.device,
                &mut self.resolver,
                frame_index,
                &modified_paths,
            );
            render_pipelines.begin_frame(
                &self.device,
                frame_index,
                shader_modules,
                pipeline_layouts,
            );

            bind_groups.begin_frame(frame_index, textures, buffers, samplers);

            textures.begin_frame(frame_index);
            buffers.begin_frame(frame_index);

            pipeline_layouts.begin_frame(frame_index);
            bind_group_layouts.begin_frame(frame_index);
            samplers.begin_frame(frame_index);
        }

        // Poll device *after* resource pool `begin_frame` since resource pools may each decide drop resources.
        // Wgpu internally may then internally decide to let go of these buffers.
        self.poll_device();
    }

    /// Call this at the end of a frame but before submitting command buffers (e.g. from [`crate::view_builder::ViewBuilder`])
    pub fn before_submit(&mut self) {
        re_tracing::profile_function!();

        // Unmap all write staging buffers.
        self.cpu_write_gpu_read_belt.lock().before_queue_submit();

        if let Some(command_encoder) = self
            .active_frame
            .before_view_builder_encoder
            .lock()
            .0
            .take()
        {
            re_tracing::profile_scope!("finish & submit frame-global encoder");
            let command_buffer = command_encoder.finish();

            // TODO(andreas): For better performance, we should try to bundle this with the single submit call that is currently happening in eframe.
            //                  How do we hook in there and make sure this buffer is submitted first?
            self.inflight_queue_submissions
                .push(self.queue.submit([command_buffer]));
        }
    }
}

pub struct FrameGlobalCommandEncoder(Option<wgpu::CommandEncoder>);

impl FrameGlobalCommandEncoder {
    fn new(device: &wgpu::Device) -> Self {
        Self(Some(device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label:
                    crate::DebugLabel::from("global \"before viewbuilder\" command encoder").get(),
            },
        )))
    }

    /// Gets the global encoder for a frame. Only valid within a frame.
    pub fn get(&mut self) -> &mut wgpu::CommandEncoder {
        self.0
            .as_mut()
            .expect("Frame global encoder can't be accessed outside of a frame!")
    }
}

impl Drop for FrameGlobalCommandEncoder {
    fn drop(&mut self) {
        // Close global command encoder if there is any pending.
        // Not doing so before shutdown causes errors!
        if let Some(encoder) = self.0.take() {
            encoder.finish();
        }
    }
}

pub struct ActiveFrameContext {
    /// Command encoder for all commands that should go in before view builder are submitted.
    ///
    /// This should be used for any gpu copy operation outside of a renderer or view builder.
    /// (i.e. typically in [`crate::renderer::DrawData`] creation!)
    pub before_view_builder_encoder: Mutex<FrameGlobalCommandEncoder>,

    /// Utility type map that will be cleared every frame.
    pub per_frame_data_helper: TypeMap,

    /// Index of this frame. Is incremented for every render frame.
    frame_index: u64,
}

fn log_adapter_info(info: &wgpu::AdapterInfo) {
    re_tracing::profile_function!();

    let is_software_rasterizer_with_known_crashes = {
        // See https://github.com/rerun-io/rerun/issues/3089
        const KNOWN_SOFTWARE_RASTERIZERS: &[&str] = &[
            "lavapipe", // Vulkan software rasterizer
            "llvmpipe", // OpenGL software rasterizer
        ];

        // I'm not sure where the incriminating string will appear, so check all fields at once:
        let info_string = format!("{info:?}").to_lowercase();

        KNOWN_SOFTWARE_RASTERIZERS
            .iter()
            .any(|&software_rasterizer| info_string.contains(software_rasterizer))
    };

    let human_readable_summary = adapter_info_summary(info);

    if is_software_rasterizer_with_known_crashes {
        re_log::warn!("Software rasterizer detected - expect poor performance and crashes. See: https://www.rerun.io/docs/getting-started/troubleshooting#graphics-issues");
        re_log::info!("wgpu adapter {human_readable_summary}");
    } else if info.device_type == wgpu::DeviceType::Cpu {
        re_log::warn!("Software rasterizer detected - expect poor performance. See: https://www.rerun.io/docs/getting-started/troubleshooting#graphics-issues");
        re_log::info!("wgpu adapter {human_readable_summary}");
    } else {
        re_log::debug!("wgpu adapter {human_readable_summary}");
    }
}

/// A human-readable summary about an adapter
fn adapter_info_summary(info: &wgpu::AdapterInfo) -> String {
    let wgpu::AdapterInfo {
        name,
        vendor: _, // skip integer id
        device: _, // skip integer id
        device_type,
        driver,
        driver_info,
        backend,
    } = &info;

    // Example values:
    // > name: "llvmpipe (LLVM 16.0.6, 256 bits)", device_type: Cpu, backend: Vulkan, driver: "llvmpipe", driver_info: "Mesa 23.1.6-arch1.4 (LLVM 16.0.6)"
    // > name: "Apple M1 Pro", device_type: IntegratedGpu, backend: Metal, driver: "", driver_info: ""
    // > name: "ANGLE (Apple, Apple M1 Pro, OpenGL 4.1)", device_type: IntegratedGpu, backend: Gl, driver: "", driver_info: ""

    let mut summary = format!("backend: {backend:?}, device_type: {device_type:?}");

    if !name.is_empty() {
        summary += &format!(", name: {name:?}");
    }
    if !driver.is_empty() {
        summary += &format!(", driver: {driver:?}");
    }
    if !driver_info.is_empty() {
        summary += &format!(", driver_info: {driver_info:?}");
    }

    summary
}
