use pong_core::Game;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    bind: wgpu::BindGroup,
}
impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .unwrap();
        // Ask the surface capability API; never assume presentation textures can be read.
        if surface
            .get_capabilities(&adapter)
            .usages
            .contains(wgpu::TextureUsages::COPY_SRC)
        {
            config.usage |= wgpu::TextureUsages::COPY_SRC;
        }
        surface.configure(&device, &config);
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("court state"),
            contents: bytemuck::cast_slice(&[0.0f32; 8]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("analytic court"),
            source: wgpu::ShaderSource::Wgsl(include_str!("court.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pl),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                targets: &[Some(config.format.into())],
                compilation_options: Default::default(),
            }),
            multiview_mask: None,
            cache: None,
        });
        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            uniform,
            bind,
        }
    }
    pub fn resize(&mut self, w: u32, h: u32) -> Result<(), String> {
        if w > 0 && h > 0 {
            self.config.width = w;
            self.config.height = h;
            self.surface.configure(&self.device, &self.config);
        }
        Ok(())
    }
    pub fn render(&mut self, g: &Game, capture: Option<&std::path::Path>) -> Result<(), String> {
        if capture.is_some() && !self.config.usage.contains(wgpu::TextureUsages::COPY_SRC) {
            return Err("Surface does not support app-owned COPY_SRC capture".into());
        }
        let data = [
            g.player,
            g.cpu,
            g.ball.x,
            g.ball.y,
            self.config.width as f32,
            self.config.height as f32,
            g.scores[0] as f32,
            g.scores[1] as f32,
        ];
        self.queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&data));
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f)
            | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return if capture.is_some() {
                    Err("Capture target temporarily unavailable".into())
                } else {
                    Ok(())
                };
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return if capture.is_some() {
                    Err("Capture target temporarily unavailable".into())
                } else {
                    Ok(())
                }
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("Surface validation failed".into())
            }
        };
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.draw(0..3, 0..1);
        }
        let readback = if capture.is_some() {
            match self.config.format {
                wgpu::TextureFormat::Rgba8Unorm
                | wgpu::TextureFormat::Rgba8UnormSrgb
                | wgpu::TextureFormat::Bgra8Unorm
                | wgpu::TextureFormat::Bgra8UnormSrgb => {}
                other => return Err(format!("Capture pixel format unsupported: {other:?}")),
            }
            let stride = (self.config.width * 4).div_ceil(256) * 256;
            let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("app capture readback"),
                size: u64::from(stride) * u64::from(self.config.height),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &frame.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(stride),
                        rows_per_image: Some(self.config.height),
                    },
                },
                wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
            );
            Some((buffer, stride))
        } else {
            None
        };
        self.queue.submit([encoder.finish()]);
        if let (Some(path), Some((buffer, stride))) = (capture, readback) {
            let (tx, rx) = std::sync::mpsc::channel();
            buffer.map_async(wgpu::MapMode::Read, .., move |result| {
                let _ = tx.send(result);
            });
            self.device
                .poll(wgpu::PollType::Wait {
                    submission_index: None,
                    timeout: Some(std::time::Duration::from_secs(5)),
                })
                .map_err(|e| e.to_string())?;
            rx.recv_timeout(std::time::Duration::from_secs(1))
                .map_err(|e| e.to_string())?
                .map_err(|e| e.to_string())?;
            let bytes = buffer.get_mapped_range(..);
            let bgra = matches!(
                self.config.format,
                wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
            );
            let mut rgb =
                Vec::with_capacity(self.config.width as usize * self.config.height as usize);
            for row in bytes
                .chunks(stride as usize)
                .take(self.config.height as usize)
            {
                for c in row[..self.config.width as usize * 4].as_chunks::<4>().0 {
                    let (r, b) = if bgra { (c[2], c[0]) } else { (c[0], c[2]) };
                    rgb.push((u32::from(r) << 16) | (u32::from(c[1]) << 8) | u32::from(b));
                }
            }
            pong_scenario::Demo::save_rgb(path, &rgb, self.config.width, self.config.height);
            drop(bytes);
            buffer.unmap();
        }
        frame.present();
        Ok(())
    }
}
