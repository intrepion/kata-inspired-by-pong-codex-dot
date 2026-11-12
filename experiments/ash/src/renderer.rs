//! Vulkan-only renderer. Rectangles use vkCmdClearAttachments in a render pass:
//! intentionally no shader compiler, no OpenGL, and no wgpu abstraction.
use ash::{vk, Entry};
use pong_low_level_common::{viewport, App, BACKGROUND};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    error::Error,
    ffi::{CStr, CString},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub struct Renderer {
    _entry: Entry,
    instance: ash::Instance,
    surface_api: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    swap_api: ash::khr::swapchain::Device,
    swap: vk::SwapchainKHR,
    extent: vk::Extent2D,
    views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    format: vk::Format,
    capture_enabled: bool,
    readback: vk::Buffer,
    readback_memory: vk::DeviceMemory,
    pass: vk::RenderPass,
    frames: Vec<vk::Framebuffer>,
    pool: vk::CommandPool,
    cmd: vk::CommandBuffer,
    acquired: vk::Semaphore,
    finished: vk::Semaphore,
}
impl Renderer {
    pub unsafe fn new(window: &winit::window::Window, capture_enabled: bool) -> Result<Self> {
        let entry=Entry::load().map_err(|e|format!("Vulkan loader unavailable ({e}). On macOS install/configure the official Vulkan SDK with MoltenVK; on Windows/Linux install a Vulkan-capable driver."))?;
        let name = CString::new("Pong ash")?;
        let app = vk::ApplicationInfo::default()
            .application_name(&name)
            .api_version(vk::API_VERSION_1_0);
        let display = window.display_handle()?.as_raw();
        let mut extensions = ash_window::enumerate_required_extensions(display)?.to_vec();
        let supported = entry.enumerate_instance_extension_properties(None)?;
        let portability = supported.iter().any(|e| {
            CStr::from_ptr(e.extension_name.as_ptr()) == ash::khr::portability_enumeration::NAME
        });
        if portability {
            extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
        }
        // MoltenVK portability subset may require physical-device-properties2 on Vulkan 1.0.
        if supported.iter().any(|e| {
            CStr::from_ptr(e.extension_name.as_ptr())
                == ash::khr::get_physical_device_properties2::NAME
        }) {
            extensions.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
        }
        let flags = if portability {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };
        let instance = entry.create_instance(
            &vk::InstanceCreateInfo::default()
                .application_info(&app)
                .enabled_extension_names(&extensions)
                .flags(flags),
            None,
        )?;
        let surface = ash_window::create_surface(
            &entry,
            &instance,
            display,
            window.window_handle()?.as_raw(),
            None,
        )?;
        let surface_api = ash::khr::surface::Instance::new(&entry, &instance);
        let mut selected = None;
        for p in instance.enumerate_physical_devices()? {
            for (index, q) in instance
                .get_physical_device_queue_family_properties(p)
                .iter()
                .enumerate()
            {
                if q.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && surface_api.get_physical_device_surface_support(p, index as u32, surface)?
                {
                    let ext = instance.enumerate_device_extension_properties(p)?;
                    if ext.iter().any(|e| {
                        CStr::from_ptr(e.extension_name.as_ptr()) == ash::khr::swapchain::NAME
                    }) {
                        selected = Some((p, index as u32));
                        break;
                    }
                }
            }
            if selected.is_some() {
                break;
            }
        }
        let (physical, family) =
            selected.ok_or("No Vulkan graphics/present queue with swapchain support")?;
        let props = instance.get_physical_device_properties(physical);
        eprintln!(
            "Vulkan device: {}",
            CStr::from_ptr(props.device_name.as_ptr()).to_string_lossy()
        );
        let priorities = [1.];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(family)
            .queue_priorities(&priorities)];
        let mut device_ext = vec![ash::khr::swapchain::NAME.as_ptr()];
        if instance
            .enumerate_device_extension_properties(physical)?
            .iter()
            .any(|e| {
                CStr::from_ptr(e.extension_name.as_ptr()) == ash::khr::portability_subset::NAME
            })
        {
            device_ext.push(ash::khr::portability_subset::NAME.as_ptr());
        }
        let device = instance.create_device(
            physical,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&device_ext),
            None,
        )?;
        let queue = device.get_device_queue(family, 0);
        let swap_api = ash::khr::swapchain::Device::new(&instance, &device);
        let pool = device.create_command_pool(
            &vk::CommandPoolCreateInfo::default()
                .queue_family_index(family)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
            None,
        )?;
        let cmd = device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )?[0];
        let acquired = device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?;
        let finished = device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?;
        let mut r = Self {
            _entry: entry,
            instance,
            surface_api,
            surface,
            physical,
            device,
            queue,
            swap_api,
            swap: vk::SwapchainKHR::null(),
            extent: vk::Extent2D::default(),
            views: vec![],
            images: vec![],
            format: vk::Format::UNDEFINED,
            capture_enabled,
            readback: vk::Buffer::null(),
            readback_memory: vk::DeviceMemory::null(),
            pass: vk::RenderPass::null(),
            frames: vec![],
            pool,
            cmd,
            acquired,
            finished,
        };
        let size = window.inner_size();
        r.resize(size.width, size.height)?;
        Ok(r)
    }
    unsafe fn clear_swap(&mut self) {
        if self.readback != vk::Buffer::null() {
            self.device.destroy_buffer(self.readback, None);
            self.readback = vk::Buffer::null();
        }
        if self.readback_memory != vk::DeviceMemory::null() {
            self.device.free_memory(self.readback_memory, None);
            self.readback_memory = vk::DeviceMemory::null();
        }
        self.images.clear();
        for f in self.frames.drain(..) {
            self.device.destroy_framebuffer(f, None);
        }
        if self.pass != vk::RenderPass::null() {
            self.device.destroy_render_pass(self.pass, None);
            self.pass = vk::RenderPass::null();
        }
        for v in self.views.drain(..) {
            self.device.destroy_image_view(v, None);
        }
        if self.swap != vk::SwapchainKHR::null() {
            self.swap_api.destroy_swapchain(self.swap, None);
            self.swap = vk::SwapchainKHR::null();
        }
    }
    pub unsafe fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            return Ok(());
        }
        self.device.device_wait_idle()?;
        self.clear_swap();
        let caps = self
            .surface_api
            .get_physical_device_surface_capabilities(self.physical, self.surface)?;
        let formats = self
            .surface_api
            .get_physical_device_surface_formats(self.physical, self.surface)?;
        let mut format = *formats
            .iter()
            .find(|f| f.format == vk::Format::B8G8R8A8_UNORM)
            .or(formats.first())
            .ok_or("No Vulkan surface formats")?;
        if format.format == vk::Format::UNDEFINED {
            format.format = vk::Format::B8G8R8A8_UNORM;
        }
        self.extent = if caps.current_extent.width != u32::MAX {
            caps.current_extent
        } else {
            vk::Extent2D {
                width: width.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: height.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        };
        let count = (caps.min_image_count + 1).min(if caps.max_image_count == 0 {
            u32::MAX
        } else {
            caps.max_image_count
        });
        let alpha = [
            vk::CompositeAlphaFlagsKHR::OPAQUE,
            vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::INHERIT,
        ]
        .into_iter()
        .find(|a| caps.supported_composite_alpha.contains(*a))
        .ok_or("No composite alpha mode")?;
        let mut usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
        if self.capture_enabled {
            if !caps
                .supported_usage_flags
                .contains(vk::ImageUsageFlags::TRANSFER_SRC)
            {
                return Err(
                    "Surface cannot copy app-owned swapchain images; scripted capture unsupported"
                        .into(),
                );
            }
            if !supported_capture_format(format.format) {
                return Err(format!("Unsupported capture format: {:?}", format.format).into());
            }
            usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        }
        self.format = format.format;
        self.swap = self.swap_api.create_swapchain(
            &vk::SwapchainCreateInfoKHR::default()
                .surface(self.surface)
                .min_image_count(count)
                .image_format(format.format)
                .image_color_space(format.color_space)
                .image_extent(self.extent)
                .image_array_layers(1)
                .image_usage(usage)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(caps.current_transform)
                .composite_alpha(alpha)
                .present_mode(vk::PresentModeKHR::FIFO)
                .clipped(true),
            None,
        )?;
        let attachments = [vk::AttachmentDescription::default()
            .format(format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let refs = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let sub = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&refs)];
        let deps = [vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)];
        self.pass = self.device.create_render_pass(
            &vk::RenderPassCreateInfo::default()
                .attachments(&attachments)
                .subpasses(&sub)
                .dependencies(&deps),
            None,
        )?;
        self.images = self.swap_api.get_swapchain_images(self.swap)?;
        for &image in &self.images {
            let range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            let view = self.device.create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format.format)
                    .subresource_range(range),
                None,
            )?;
            self.views.push(view);
            self.frames.push(
                self.device.create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(self.pass)
                        .attachments(&[view])
                        .width(self.extent.width)
                        .height(self.extent.height)
                        .layers(1),
                    None,
                )?,
            );
        }
        if self.capture_enabled {
            let size = u64::from(self.extent.width) * u64::from(self.extent.height) * 4;
            self.readback = self.device.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )?;
            let requirements = self.device.get_buffer_memory_requirements(self.readback);
            let properties = self
                .instance
                .get_physical_device_memory_properties(self.physical);
            let memory_type = (0..properties.memory_type_count)
                .find(|&i| {
                    requirements.memory_type_bits & (1 << i) != 0
                        && properties.memory_types[i as usize].property_flags.contains(
                            vk::MemoryPropertyFlags::HOST_VISIBLE
                                | vk::MemoryPropertyFlags::HOST_COHERENT,
                        )
                })
                .ok_or("No host-visible coherent capture memory")?;
            self.readback_memory = self.device.allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type),
                None,
            )?;
            self.device
                .bind_buffer_memory(self.readback, self.readback_memory, 0)?;
        }
        Ok(())
    }
    pub unsafe fn draw(
        &mut self,
        app: &App,
        capture: &mut Option<std::path::PathBuf>,
    ) -> Result<bool> {
        if self.swap == vk::SwapchainKHR::null() {
            return Ok(false);
        }
        // Intentionally serialize this learning baseline; queue idle also makes presentation
        // semaphore reuse safe. Production engines use per-frame/per-image sync instead.
        self.device.queue_wait_idle(self.queue)?;
        let (index, suboptimal) = match self.swap_api.acquire_next_image(
            self.swap,
            u64::MAX,
            self.acquired,
            vk::Fence::null(),
        ) {
            Ok(v) => v,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return Ok(true),
            Err(e) => return Err(e.into()),
        };
        self.device
            .reset_command_buffer(self.cmd, vk::CommandBufferResetFlags::empty())?;
        self.device
            .begin_command_buffer(self.cmd, &vk::CommandBufferBeginInfo::default())?;
        let color = |c: [u8; 3]| vk::ClearColorValue {
            float32: [
                c[0] as f32 / 255.,
                c[1] as f32 / 255.,
                c[2] as f32 / 255.,
                1.,
            ],
        };
        let clear = [vk::ClearValue {
            color: color(BACKGROUND),
        }];
        self.device.cmd_begin_render_pass(
            self.cmd,
            &vk::RenderPassBeginInfo::default()
                .render_pass(self.pass)
                .framebuffer(self.frames[index as usize])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.extent,
                })
                .clear_values(&clear),
            vk::SubpassContents::INLINE,
        );
        let (s, ox, oy) = viewport(self.extent.width, self.extent.height);
        for r in app.rectangles() {
            let x = (ox + r.x * s).max(0.) as i32;
            let y = (oy + r.y * s).max(0.) as i32;
            let right = ((ox + (r.x + r.w) * s).ceil() as u32).min(self.extent.width);
            let bottom = ((oy + (r.y + r.h) * s).ceil() as u32).min(self.extent.height);
            if right <= x as u32 || bottom <= y as u32 {
                continue;
            }
            let a = [vk::ClearAttachment::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .color_attachment(0)
                .clear_value(vk::ClearValue {
                    color: color(r.color),
                })];
            let rect = [vk::ClearRect::default()
                .rect(vk::Rect2D {
                    offset: vk::Offset2D { x, y },
                    extent: vk::Extent2D {
                        width: right - x as u32,
                        height: bottom - y as u32,
                    },
                })
                .base_array_layer(0)
                .layer_count(1)];
            self.device.cmd_clear_attachments(self.cmd, &a, &rect);
        }
        self.device.cmd_end_render_pass(self.cmd);
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let barrier = vk::ImageMemoryBarrier::default()
            .image(self.images[index as usize])
            .subresource_range(range)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        if capture.is_some() {
            self.device.cmd_pipeline_barrier(
                self.cmd,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier
                    .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                    .dst_access_mask(vk::AccessFlags::TRANSFER_READ)],
            );
            // buffer_row_length=0 means tightly packed rows; unlike linear images,
            // this copy has no implementation-specific mapped image row pitch.
            self.device.cmd_copy_image_to_buffer(
                self.cmd,
                self.images[index as usize],
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                self.readback,
                &[vk::BufferImageCopy::default()
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_extent(vk::Extent3D {
                        width: self.extent.width,
                        height: self.extent.height,
                        depth: 1,
                    })],
            );
            self.device.cmd_pipeline_barrier(
                self.cmd,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST | vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(self.readback)
                    .size(vk::WHOLE_SIZE)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::HOST_READ)],
                &[barrier
                    .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                    .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                    .src_access_mask(vk::AccessFlags::TRANSFER_READ)],
            );
        } else {
            self.device.cmd_pipeline_barrier(
                self.cmd,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier.new_layout(vk::ImageLayout::PRESENT_SRC_KHR)],
            );
        }
        self.device.end_command_buffer(self.cmd)?;
        let waits = [self.acquired];
        let stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let cmds = [self.cmd];
        let signals = [self.finished];
        let submit = [vk::SubmitInfo::default()
            .wait_semaphores(&waits)
            .wait_dst_stage_mask(&stages)
            .command_buffers(&cmds)
            .signal_semaphores(&signals)];
        self.device
            .queue_submit(self.queue, &submit, vk::Fence::null())?;
        if let Some(path) = capture.as_ref() {
            // Only map after this frame's transfer completes; never read a previous frame.
            self.device.queue_wait_idle(self.queue)?;
            let size = self.extent.width as usize * self.extent.height as usize * 4;
            let mapped = self.device.map_memory(
                self.readback_memory,
                0,
                size as u64,
                vk::MemoryMapFlags::empty(),
            )?;
            let pixels = packed_rgb(
                std::slice::from_raw_parts(mapped.cast::<u8>(), size),
                self.format,
            );
            self.device.unmap_memory(self.readback_memory);
            pong_scenario::Demo::save_rgb(path, &pixels, self.extent.width, self.extent.height);
            *capture = None;
        }
        match self.swap_api.queue_present(
            self.queue,
            &vk::PresentInfoKHR::default()
                .wait_semaphores(&signals)
                .swapchains(&[self.swap])
                .image_indices(&[index]),
        ) {
            Ok(v) => Ok(v || suboptimal),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }
}
impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.clear_swap();
            self.device.destroy_semaphore(self.acquired, None);
            self.device.destroy_semaphore(self.finished, None);
            self.device.destroy_command_pool(self.pool, None);
            self.device.destroy_device(None);
            self.surface_api.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

fn supported_capture_format(format: vk::Format) -> bool {
    matches!(
        format,
        vk::Format::B8G8R8A8_UNORM
            | vk::Format::B8G8R8A8_SRGB
            | vk::Format::R8G8B8A8_UNORM
            | vk::Format::R8G8B8A8_SRGB
    )
}
fn packed_rgb(bytes: &[u8], format: vk::Format) -> Vec<u32> {
    assert!(supported_capture_format(format));
    assert_eq!(bytes.len() % 4, 0);
    let bgra = matches!(
        format,
        vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB
    );
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|p| {
            let (r, b) = if bgra { (p[2], p[0]) } else { (p[0], p[2]) };
            (u32::from(r) << 16) | (u32::from(p[1]) << 8) | u32::from(b)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_preserves_row_order_and_swizzles_bgra() {
        let bytes = [3, 2, 1, 255, 6, 5, 4, 128];
        assert_eq!(
            packed_rgb(&bytes, vk::Format::B8G8R8A8_UNORM),
            [0x010203, 0x040506]
        );
        assert_eq!(
            packed_rgb(&bytes, vk::Format::R8G8B8A8_SRGB),
            [0x030201, 0x060504]
        );
    }
    #[test]
    fn rejects_non_eight_bit_formats() {
        assert!(!supported_capture_format(
            vk::Format::A2B10G10R10_UNORM_PACK32
        ));
        assert!(!supported_capture_format(vk::Format::UNDEFINED));
    }
}
