// Local modification: port rendering and surface-acquisition handling to wgpu 29. See PATCH-PROVENANCE.txt.
use crate::{
    CameraContainer, ObjectStorage, PipelineData, WindowSize,
    prelude::{ShaderSettings, TextureData},
    utils::default_resources::{DEFAULT_COLOR, DEFAULT_SHADER, DEFAULT_TEXTURE},
};

/// Main renderer class. this will contain all methods and data related to the renderer
#[derive(Debug)]
pub struct Renderer {
    /// A [`wgpu::Surface`] represents a platform-specific surface
    /// (e.g. a window) onto which rendered images may be presented.
    pub surface: Option<wgpu::Surface<'static>>,
    /// Context for all of the gpu objects
    pub instance: wgpu::Instance,
    /// Handle to a physical graphics and/or compute device.
    #[allow(unused)]
    pub adapter: wgpu::Adapter,
    /// Open connection to a graphics and/or compute device.
    pub device: wgpu::Device,
    device_lost: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// Handle to a command queue on a device.
    pub queue: wgpu::Queue,
    /// Describes a [`wgpu::Surface`]
    pub config: wgpu::SurfaceConfiguration,
    /// The size of the window
    pub size: WindowSize,
    /// The texture bind group layout
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    /// The uniform bind group layout
    pub default_uniform_bind_group_layout: wgpu::BindGroupLayout,
    /// The depth buffer, used to render object depth
    pub depth_buffer: (wgpu::Texture, wgpu::TextureView, wgpu::Sampler),
    /// The default data used within the renderer
    pub default_data: Option<(crate::Textures, crate::Shaders, crate::UniformBuffers)>,
    /// The camera used in the engine
    pub camera: Option<crate::UniformBuffers>,
    /// Background clear color
    pub clear_color: wgpu::Color,
    /// Scissor cut section of the screen to render to
    /// (x, y, width, height)
    pub scissor_rect: Option<(u32, u32, u32, u32)>,
    /// The texture data that holds data for the headless mode
    #[cfg(feature = "headless")]
    pub headless_texture_data: Vec<u8>,
}
unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    /// Creates a new renderer.
    pub(crate) async fn new(
        size: WindowSize,
        settings: crate::EngineSettings,
        #[cfg(all(feature = "window", not(feature = "headless"), not(target_os = "android")))]
        display: winit::event_loop::OwnedDisplayHandle,
    ) -> Result<Self, crate::error::Error> {
        if size.0 == 0 || size.1 == 0 {
            return Err(crate::error::Error::Custom("initial dimensions must be nonzero".into()));
        }
        // The instance is a handle to our GPU
        #[cfg(all(feature = "window", not(feature = "headless"), not(target_os = "android")))]
        let mut instance_desc = wgpu::InstanceDescriptor::new_with_display_handle(Box::new(display));
        #[cfg(not(all(feature = "window", not(feature = "headless"), not(target_os = "android"))))]
        let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
        instance_desc.backends = settings.backends;
        let instance = wgpu::Instance::new(instance_desc);

        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: settings.power_preference,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
        {
            Ok(adapter) => {
                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor {
                        label: Some("Device"),
                        required_features: settings.features,
                        required_limits: settings.limits,
                        memory_hints: if settings.power_preference
                            == wgpu::PowerPreference::HighPerformance
                        {
                            wgpu::MemoryHints::Performance
                        } else {
                            wgpu::MemoryHints::MemoryUsage
                        },
                        trace: wgpu::Trace::Off,
                        ..Default::default()
                    })
                    .await?;

                let device_lost = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let lost_flag = device_lost.clone();
                device.set_device_lost_callback(move |reason, message| {
                    eprintln!("Blue Engine device lost: {reason:?}: {message}");
                    lost_flag.store(true, std::sync::atomic::Ordering::Release);
                });
                if !valid_surface_dimensions(size, device.limits().max_texture_dimension_2d) {
                    return Err(crate::error::Error::Custom("initial dimensions exceed GPU limits".into()));
                }
                let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;

                #[cfg(target_os = "android")]
                let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: texture_format, //wgpu::TextureFormat::Bgra8UnormSrgb,
                    #[cfg(target_os = "android")]
                    width: 1080,
                    #[cfg(not(feature = "android"))]
                    width: size.0,
                    #[cfg(target_os = "android")]
                    height: 2300,
                    #[cfg(not(target_os = "android"))]
                    height: size.1,
                    #[cfg(target_os = "android")]
                    present_mode: wgpu::PresentMode::Mailbox,
                    #[cfg(not(target_os = "android"))]
                    present_mode: settings.present_mode,
                    alpha_mode: settings.alpha_mode,
                    view_formats: vec![texture_format],
                    desired_maximum_frame_latency: settings.desired_maximum_frame_latency,
                };

                let texture_bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    multisampled: false,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), //comparison: false,
                                // filtering: true,
                                count: None,
                            },
                        ],
                        label: Some("texture_bind_group_layout"),
                    });

                let default_uniform_bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("uniform dynamic bind group layout"),
                        entries: &[wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        }],
                    });

                let depth_buffer = Renderer::build_depth_buffer("Depth Buffer", &device, &config);

                let mut renderer = Self {
                    instance,
                    adapter,
                    surface: None,
                    device,
                    device_lost,
                    queue,
                    config,
                    size,

                    texture_bind_group_layout,
                    default_uniform_bind_group_layout,
                    depth_buffer,

                    default_data: None,
                    camera: None,
                    clear_color: wgpu::Color::BLACK,
                    scissor_rect: None,

                    #[cfg(feature = "headless")]
                    headless_texture_data: Vec::<u8>::with_capacity((size.0 * size.1) as usize * 4),
                };

                renderer.build_default_data();

                Ok(renderer)
            }
            Err(e) => Err(crate::error::Error::AdapterNotFound(e)),
        }
    }

    pub(crate) fn build_default_data(&mut self) {
        if let Ok(default_texture) = self.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::prelude::TextureMode::Clamp,
            //crate::prelude::TextureFormat::PNG
        ) {
            let default_uniform = self.build_uniform_buffer(&vec![
                self.build_uniform_buffer_part("Transformation Matrix", crate::Matrix4::IDENTITY),
                self.build_uniform_buffer_part("Color", DEFAULT_COLOR),
            ]);

            let default_shader = self.build_shader(
                "Default Shader",
                DEFAULT_SHADER.to_string(),
                Some(&default_uniform.1),
                ShaderSettings::default(),
            );

            self.default_data = Some((default_texture, default_shader, default_uniform.0));
        } else {
            eprintln!("Could not build the default texture, there may be something wrong!");
            self.default_data = None;
        }
    }

    /// Resize the window.
    #[cfg(all(not(feature = "headless"), feature = "window"))]
    pub(crate) fn resize(&mut self, new_size: WindowSize) -> Result<(), crate::error::Error> {
        if new_size.0 != 0 && new_size.1 != 0 {
            if !valid_surface_dimensions(new_size, self.device.limits().max_texture_dimension_2d) {
                return Err(crate::error::Error::Custom("window dimensions exceed GPU limits".into()));
            }
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            #[cfg(not(target_os = "android"))]
            if let Some(surface) = self.surface.as_ref() {
                surface.configure(&self.device, &self.config);
                self.depth_buffer =
                    Self::build_depth_buffer("Depth Buffer", &self.device, &self.config);
            }
        }
        Ok(())
    }

    /// Render the scene. Returns the command encoder, the texture view, and the surface texture.
    pub(crate) fn pre_render(
        &mut self,
        objects: &ObjectStorage,
        window_size: WindowSize,
        camera: &CameraContainer,
    ) -> Result<
        Option<(
            wgpu::CommandEncoder,
            wgpu::TextureView,
            Option<wgpu::SurfaceTexture>,
            Option<(wgpu::Buffer, wgpu::Texture)>,
        )>,
        crate::error::Error,
    > {
        if self.device_lost.load(std::sync::atomic::Ordering::Acquire) {
            return Err(crate::error::Error::Custom("GPU device lost".into()));
        }
        #[cfg(not(feature = "headless"))]
        if !valid_surface_dimensions(window_size, u32::MAX) { return Ok(None); }
        #[cfg(not(feature = "headless"))]
        let surface = if let Some(ref surface) = self.surface {
            surface
        } else {
            return Ok(None);
        };
        #[cfg(not(feature = "headless"))]
        let acquisition = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => Acquisition::Ready(frame),
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => Acquisition::Suboptimal(frame),
            wgpu::CurrentSurfaceTexture::Outdated => Acquisition::Outdated,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => Acquisition::Skip,
            wgpu::CurrentSurfaceTexture::Lost => Acquisition::Terminal("GPU surface lost; recreate required"),
            wgpu::CurrentSurfaceTexture::Validation => Acquisition::Terminal("GPU surface validation error"),
        };
        #[cfg(not(feature = "headless"))]
        let Some(frame) = resolve_acquisition(acquisition, || {
            if surface_size_ready([self.config.width, self.config.height], [window_size.0, window_size.1]) {
                surface.configure(&self.device, &self.config);
            }
        })? else { return Ok(None); };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        #[cfg(feature = "headless")]
        let render_target = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[self.config.format],
        });

        // ? There might be a way to enable both headless and normal rendering,
        // ? However the cost of it can increase a lot
        #[cfg(feature = "headless")]
        let view = render_target.create_view(&wgpu::TextureViewDescriptor::default());
        #[cfg(not(feature = "headless"))]
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        #[cfg(feature = "headless")]
        let headless_output_staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: self.headless_texture_data.capacity() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.1,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            multiview_mask: None,
            occlusion_query_set: None,
        });

        if let Some(scissor_rect) = self.scissor_rect {
            // check if scissor bounds are smaller than the window
            if scissor_rect.0 + scissor_rect.2 < window_size.0
                && scissor_rect.1 + scissor_rect.3 < window_size.1
            {
                render_pass.set_scissor_rect(
                    scissor_rect.0,
                    scissor_rect.1,
                    scissor_rect.2,
                    scissor_rect.3,
                );
            }
        }

        if let Some(default_data) = self.default_data.as_ref() {
            render_pass.set_bind_group(0, &default_data.0, &[]);
            render_pass.set_pipeline(&default_data.1);
        }

        // sort the object list in descending render order
        // ! There needs to be a better way for this, to not iterate twice
        let mut object_list = objects.iter().collect::<Vec<_>>();
        object_list.sort_by(|(_, a), (_, b)| a.render_order.cmp(&b.render_order).reverse());

        for (_, i) in object_list {
            if let Some(camera_data) = i.camera_effect.as_ref() {
                if let Some(camera) = camera.get(camera_data.as_ref()) {
                    render_pass.set_bind_group(1, &camera.uniform_data, &[]);
                }
            } else {
                if let Some(main_camera) = camera.get("main") {
                    render_pass.set_bind_group(1, &main_camera.uniform_data, &[]);
                }
            }

            if i.is_visible {
                let vertex_buffer = get_pipeline_vertex_buffer(&i.pipeline.vertex_buffer, objects);
                let shader = get_pipeline_shader(&i.pipeline.shader, objects);
                let texture = get_pipeline_texture(&i.pipeline.texture, objects);
                let uniform = get_pipeline_uniform_buffer(&i.pipeline.uniform, objects);

                // vertex
                if let Some(vertex_buffer) = vertex_buffer {
                    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, i.instance_buffer.slice(..));
                    render_pass.set_index_buffer(
                        vertex_buffer.index_buffer.slice(..),
                        #[cfg(not(feature = "u32"))]
                        wgpu::IndexFormat::Uint16,
                        #[cfg(feature = "u32")]
                        wgpu::IndexFormat::Uint32,
                    );

                    // shader
                    if let Some(shader) = shader {
                        render_pass.set_pipeline(shader);
                    }
                    // texture
                    if let Some(texture) = texture {
                        render_pass.set_bind_group(0, texture, &[]);
                    }
                    // uniform
                    if let Some(Some(uniform)) = uniform {
                        render_pass.set_bind_group(2, uniform, &[]);
                    }
                    render_pass.draw_indexed(0..vertex_buffer.length, 0, 0..i.instances.len() as _);
                }
            }
        }
        drop(render_pass);

        Ok(Some((
            encoder,
            view,
            #[cfg(feature = "headless")]
            None,
            #[cfg(not(feature = "headless"))]
            Some(frame),
            #[cfg(feature = "headless")]
            Some((headless_output_staging_buffer, render_target)),
            #[cfg(not(feature = "headless"))]
            None,
        )))
    }

    /// Render the scene.
    pub(crate) fn render(
        &mut self,
        encoder: wgpu::CommandEncoder,
        _frame: Option<wgpu::SurfaceTexture>,
        _headless: Option<(wgpu::Buffer, wgpu::Texture)>,
    ) {
        #[cfg(feature = "headless")]
        {
            #[allow(clippy::expect_used)]
            let (output_staging_buffer, render_target) =
                _headless.expect("Error unpacking headless content. This should not error!");

            let mut encoder = encoder;

            #[cfg(feature = "headless")]
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &render_target,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &output_staging_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        // This needs to be a multiple of 256. Normally we would need to pad
                        // it but we here know it will work out anyways.
                        bytes_per_row: Some(self.config.width * 4),
                        rows_per_image: Some(self.config.height),
                    },
                },
                wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
            );

            // submit will accept anything that implements IntoIter
            self.queue.submit(Some(encoder.finish()));

            pollster::block_on(async {
                let buffer_slice = output_staging_buffer.slice(..);
                let (sender, receiver) = std::sync::mpsc::channel();
                buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
                self.device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .unwrap();
                receiver.recv().unwrap().unwrap();
                {
                    let view = buffer_slice.get_mapped_range();
                    self.headless_texture_data.extend_from_slice(&view[..]);
                }
                output_staging_buffer.unmap();
            });
        }

        #[cfg(not(feature = "headless"))]
        if let Some(frame) = _frame {
            // submit will accept anything that implements IntoIter
            self.queue.submit(Some(encoder.finish()));

            frame.present();
        }
    }

    /// Sets the background color
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = wgpu::Color { r, g, b, a }
    }
}

// =========================== Extract Pipeline Data ===========================
macro_rules! gen_pipeline {
    ($function_name:ident, $buffer_type:ty, $buffer_field:ident) => {
        fn $function_name<'a>(
            data: &'a PipelineData<$buffer_type>,
            objects: &'a ObjectStorage,
        ) -> Option<&'a $buffer_type> {
            match data {
                PipelineData::Copy(object_id) => {
                    let data = objects.get(object_id);
                    if let Some(data) = data {
                        $function_name(&data.pipeline.$buffer_field, objects)
                    } else {
                        None
                    }
                }
                PipelineData::Data(data) => Some(data),
            }
        }
    };
}

gen_pipeline!(
    get_pipeline_vertex_buffer,
    crate::VertexBuffers,
    vertex_buffer
);
gen_pipeline!(get_pipeline_shader, crate::Shaders, shader);
gen_pipeline!(get_pipeline_texture, crate::Textures, texture);

/// Get the pipeline uniform_buffer.
fn get_pipeline_uniform_buffer<'a>(
    data: &'a PipelineData<Option<crate::UniformBuffers>>,
    objects: &'a ObjectStorage,
) -> Option<&'a Option<crate::UniformBuffers>> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id);
            if let Some(data) = data {
                get_pipeline_uniform_buffer(&data.pipeline.uniform, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

// Kept generic so tests can verify the acquired texture is dropped before
// reconfiguration without requiring a GPU or constructing a SurfaceTexture.
#[cfg(not(feature = "headless"))]
enum Acquisition<T> {
    Ready(T), Suboptimal(T), Outdated, Skip, Terminal(&'static str),
}
fn valid_surface_dimensions(size: WindowSize, maximum: u32) -> bool {
    size.0 > 0 && size.1 > 0 && size.0 <= maximum && size.1 <= maximum
}
#[cfg(not(feature = "headless"))]
fn surface_size_ready(config: [u32; 2], window: [u32; 2]) -> bool {
    config[0] > 0 && config[1] > 0 && config == window
}
#[cfg(not(feature = "headless"))]
fn resolve_acquisition<T>(value: Acquisition<T>, reconfigure: impl FnOnce()) -> Result<Option<T>, crate::error::Error> {
    match value {
        Acquisition::Ready(frame) => Ok(Some(frame)),
        Acquisition::Suboptimal(frame) => { drop(frame); reconfigure(); Ok(None) }
        Acquisition::Outdated => { reconfigure(); Ok(None) }
        Acquisition::Skip => Ok(None),
        Acquisition::Terminal(message) => Err(crate::error::Error::Custom(message.into())),
    }
}
#[cfg(all(test, not(feature = "headless")))]
mod acquisition_tests {
    use super::{Acquisition, resolve_acquisition, surface_size_ready, valid_surface_dimensions};
    use std::cell::Cell;
    struct Frame<'a>(&'a Cell<bool>);
    impl Drop for Frame<'_> { fn drop(&mut self) { self.0.set(true); } }
    #[test]
    fn surface_dimensions_respect_zero_and_device_limit() {
        assert!(valid_surface_dimensions((640, 480), 1024));
        assert!(valid_surface_dimensions((1024, 1024), 1024));
        assert!(!valid_surface_dimensions((0, 480), 1024));
        assert!(!valid_surface_dimensions((640, 1025), 1024));
    }
    #[test]
    fn reconfigure_requires_current_nonzero_window_size() {
        assert!(surface_size_ready([640, 480], [640, 480]));
        assert!(!surface_size_ready([640, 480], [0, 0]));
        assert!(!surface_size_ready([640, 480], [800, 600]));
        assert!(!surface_size_ready([0, 480], [0, 480]));
    }
    #[test]
    fn suboptimal_drops_frame_before_reconfigure() {
        let dropped = Cell::new(false); let called = Cell::new(false);
        let result = resolve_acquisition(Acquisition::Suboptimal(Frame(&dropped)), || {
            assert!(dropped.get()); called.set(true);
        }).unwrap();
        assert!(result.is_none() && called.get());
    }
    #[test]
    fn ready_skip_outdated_and_terminal_remain_distinct() {
        let configured=Cell::new(0);
        assert_eq!(resolve_acquisition(Acquisition::Ready(7), || configured.set(99)).unwrap(), Some(7));
        assert!(resolve_acquisition::<u8>(Acquisition::Skip, || configured.set(99)).unwrap().is_none());
        assert!(resolve_acquisition::<u8>(Acquisition::Outdated, || configured.set(configured.get()+1)).unwrap().is_none());
        assert_eq!(configured.get(),1);
        for message in ["GPU surface lost; recreate required", "GPU surface validation error"] {
            let error=resolve_acquisition::<u8>(Acquisition::Terminal(message), || configured.set(99)).unwrap_err();
            assert!(error.to_string().contains(message));
        }
        assert_eq!(configured.get(),1);
    }
}
