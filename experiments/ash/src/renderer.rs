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
    pass: vk::RenderPass,
    frames: Vec<vk::Framebuffer>,
    pool: vk::CommandPool,
    cmd: vk::CommandBuffer,
    acquired: vk::Semaphore,
    finished: vk::Semaphore,
}
impl Renderer {
    pub unsafe fn new(window: &winit::window::Window) -> Result<Self> {
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
        self.swap = self.swap_api.create_swapchain(
            &vk::SwapchainCreateInfoKHR::default()
                .surface(self.surface)
                .min_image_count(count)
                .image_format(format.format)
                .image_color_space(format.color_space)
                .image_extent(self.extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
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
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)];
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
        for image in self.swap_api.get_swapchain_images(self.swap)? {
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
        Ok(())
    }
    pub unsafe fn draw(&mut self, app: &App) -> Result<bool> {
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
