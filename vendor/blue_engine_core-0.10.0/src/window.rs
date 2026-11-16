// Local modification: handle desktop surface startup, capabilities, resize and terminal errors. See PATCH-PROVENANCE.txt.
/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::prelude::{Engine, Renderer};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
};

/// A wrapper for winit window to make it easier to use and more ergonomic.
#[derive(Debug)]
pub struct Window {
    /// The winit window itself.
    pub window: Option<std::sync::Arc<crate::winit::window::Window>>,
    /// Default attributes of the window
    pub default_attributes: winit::window::WindowAttributes,
    /// Whether the engine should close.
    pub should_close: bool,
}
crate::macros::impl_deref_field!(
    Window,
    Option<std::sync::Arc<crate::winit::window::Window>>,
    window
);

fn record_first<T>(slot: &mut Option<T>, error: T) {
    if slot.is_none() { *slot = Some(error); }
}

fn surface_format(formats: &[wgpu::TextureFormat]) -> Option<wgpu::TextureFormat> {
    formats.iter().copied().find(|f| f.is_srgb()).or_else(|| formats.first().copied())
}

fn surface_modes_supported(caps: &wgpu::SurfaceCapabilities, present: wgpu::PresentMode, alpha: wgpu::CompositeAlphaMode) -> bool {
    use wgpu::PresentMode::{AutoNoVsync, AutoVsync, Fifo, FifoRelaxed, Immediate, Mailbox};
    use wgpu::CompositeAlphaMode::{Auto, Inherit, Opaque};
    let present_ok = caps.present_modes.contains(&present) || match present {
        AutoVsync => caps.present_modes.iter().any(|m| matches!(m, FifoRelaxed | Fifo)),
        AutoNoVsync => caps.present_modes.iter().any(|m| matches!(m, Immediate | Mailbox | Fifo)),
        _ => false,
    };
    let alpha_ok = caps.alpha_modes.contains(&alpha) || (alpha == Auto && caps.alpha_modes.iter().any(|m| matches!(m, Opaque | Inherit)));
    present_ok && alpha_ok && caps.usages.contains(wgpu::TextureUsages::RENDER_ATTACHMENT)
}

#[cfg(test)]
mod lifecycle_error_tests {
    use super::{record_first, surface_format, surface_modes_supported};
    #[test]
    fn surface_modes_check_explicit_and_automatic_fallbacks() {
        use wgpu::{CompositeAlphaMode as A, PresentMode as P, TextureUsages as U};
        let mut caps = wgpu::SurfaceCapabilities { formats: vec![], present_modes: vec![P::Fifo], alpha_modes: vec![A::Opaque], usages: U::RENDER_ATTACHMENT };
        assert!(surface_modes_supported(&caps, P::AutoVsync, A::Auto));
        assert!(surface_modes_supported(&caps, P::AutoNoVsync, A::Opaque));
        assert!(!surface_modes_supported(&caps, P::Mailbox, A::Opaque));
        assert!(!surface_modes_supported(&caps, P::Fifo, A::PreMultiplied));
        caps.usages = U::empty();
        assert!(!surface_modes_supported(&caps, P::Fifo, A::Opaque));
        caps.usages = U::RENDER_ATTACHMENT; caps.present_modes.clear();
        assert!(!surface_modes_supported(&caps, P::AutoVsync, A::Auto));
    }
    #[test]
    fn retains_first_failure() {
        let mut slot = None; record_first(&mut slot, "startup"); record_first(&mut slot, "later");
        assert_eq!(slot, Some("startup"));
    }
    #[test]
    fn chooses_surface_format_or_reports_empty() {
        use wgpu::TextureFormat::{Rgba8Unorm, Rgba8UnormSrgb};
        assert_eq!(surface_format(&[]), None);
        assert_eq!(surface_format(&[Rgba8Unorm]), Some(Rgba8Unorm));
        assert_eq!(surface_format(&[Rgba8Unorm, Rgba8UnormSrgb]), Some(Rgba8UnormSrgb));
    }
}

impl Engine {
    fn fail(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, error: crate::error::Error) {
        record_first(&mut self.terminal_error, error);
        event_loop.exit();
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            // ? This should move to Window instead of here to clear the clutter
            {
                let new_window = match event_loop.create_window(self.window.default_attributes.clone()) {
                    Ok(window) => window,
                    Err(error) => { self.fail(event_loop, crate::error::Error::Custom(format!("window creation failed: {error}"))); return; }
                };
                let new_window = std::sync::Arc::new(new_window);

                if self.renderer.surface.is_none() {
                    {
                        let surface = match self.renderer.instance.create_surface(new_window.clone()) {
                            Ok(surface) => surface,
                            Err(error) => { self.fail(event_loop, crate::error::Error::Custom(format!("surface creation failed: {error}"))); return; }
                        };
                        let surface_capabilities = surface.get_capabilities(&self.renderer.adapter);
                        let Some(tex_format) = surface_format(&surface_capabilities.formats) else {
                            self.fail(event_loop, crate::error::Error::Custom("surface exposes no formats".into()));
                            return;
                        };

                        if !surface_modes_supported(&surface_capabilities, self.renderer.config.present_mode, self.renderer.config.alpha_mode) {
                            self.fail(event_loop, crate::error::Error::Custom("unsupported surface present mode, alpha mode or usage".into()));
                            return;
                        }
                        let actual = new_window.inner_size();
                        if actual.width == 0 || actual.height == 0 || actual.width > self.renderer.device.limits().max_texture_dimension_2d || actual.height > self.renderer.device.limits().max_texture_dimension_2d {
                            self.fail(event_loop, crate::error::Error::Custom("initial window dimensions are zero or exceed GPU limits".into()));
                            return;
                        }
                        self.renderer.config.width = actual.width;
                        self.renderer.config.height = actual.height;
                        self.renderer.size = (actual.width, actual.height);
                        self.camera.set_resolution((actual.width, actual.height));
                        self.camera.update_view_projection(&mut self.renderer);
                        self.renderer.config.format = tex_format;
                        self.renderer.config.view_formats = vec![tex_format];

                        surface.configure(&self.renderer.device, &self.renderer.config);
                        self.renderer.depth_buffer = Renderer::build_depth_buffer(
                            "Depth Buffer",
                            &self.renderer.device,
                            &self.renderer.config,
                        );
                        self.renderer.surface = Some(surface);

                        self.renderer.build_default_data();
                        self.objects.iter_mut().for_each(|i| {
                            i.1.update(&mut self.renderer);
                        });
                    }
                }

                new_window.set_min_inner_size(self.window.default_attributes.min_inner_size);
                new_window.set_max_inner_size(self.window.default_attributes.max_inner_size);
                if let Some(position) = self.window.default_attributes.position {
                    new_window.set_outer_position(position);
                }
                new_window.set_resizable(self.window.default_attributes.resizable);
                new_window.set_enabled_buttons(self.window.default_attributes.enabled_buttons);
                new_window.set_title(self.window.default_attributes.title.as_str());
                new_window.set_maximized(self.window.default_attributes.maximized);
                new_window.set_visible(self.window.default_attributes.visible);
                new_window.set_transparent(self.window.default_attributes.transparent);
                new_window.set_blur(self.window.default_attributes.blur);
                new_window.set_decorations(self.window.default_attributes.decorations);
                new_window.set_window_icon(self.window.default_attributes.window_icon.clone());
                new_window.set_theme(self.window.default_attributes.preferred_theme);
                new_window.set_resize_increments(self.window.default_attributes.resize_increments);
                new_window.set_window_level(self.window.default_attributes.window_level);
                new_window.set_cursor(self.window.default_attributes.cursor.clone());
                new_window.set_fullscreen(self.window.default_attributes.fullscreen.clone());

                self.window.window = Some(new_window);
            }

            let mut events = std::mem::take(&mut self.signals.events);
            events.iter_mut().for_each(|i| {
                i.1.init(self);
            });
            std::mem::swap(&mut self.signals.events, &mut events);
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        self.simple_input.step();
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.simple_input.process_device_event(&event);

        let mut events = std::mem::take(&mut self.signals.events);
        events.iter_mut().for_each(|i| {
            i.1.device_events(self, &event);
        });
        std::mem::swap(&mut self.signals.events, &mut events);

        self.raw_input = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.simple_input.process_window_event(&event);
        let mut events = std::mem::take(&mut self.signals.events);
        events.iter_mut().for_each(|i| {
            i.1.window_events(self, &event);
        });
        std::mem::swap(&mut self.signals.events, &mut events);

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            WindowEvent::Resized(size) => {
                let size = (size.width, size.height);
                if size.0 == 0 || size.1 == 0 { return; }
                if let Err(error) = self.renderer.resize(size) { self.fail(event_loop, error); return; }
                self.camera.set_resolution(size);
                self.camera.update_view_projection(&mut self.renderer);
            }
            WindowEvent::RedrawRequested => {
                if self.window.should_close {
                    event_loop.exit();
                    return;
                }

                if let Some(window_ref) = self.window.as_ref() {
                    let size = window_ref.inner_size();
                    let size = (size.width, size.height);
                    let prepared = match self.renderer.pre_render(&self.objects, size, &self.camera) {
                        Ok(prepared) => prepared,
                        Err(error) => {
                            self.fail(event_loop, error);
                            return;
                        }
                    };
                    if let Some((mut encoder, view, frame, headless_output)) = prepared {
                        let mut events = std::mem::take(&mut self.signals.events);
                        events.iter_mut().for_each(|i| {
                            i.1.frame(self, &mut encoder, &view);
                        });
                        std::mem::swap(&mut self.signals.events, &mut events);

                        for camera_value in self.camera.values_mut() {
                            camera_value.update_view_projection(&mut self.renderer);
                        }
                        self.objects.iter_mut().for_each(|i| {
                            if i.1.changed {
                                i.1.update(&mut self.renderer);
                            }
                        });

                        let mut update_function = self.update_loop.take();
                        if let Some(ref mut update_function) = update_function {
                            update_function(self);
                        }
                        self.update_loop = update_function;

                        self.renderer.render(encoder, frame, headless_output);
                    }
                }

                if let Some(window_inner) = &self.window.window {
                    window_inner.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.simple_input.end_step();

        if self.simple_input.close_requested() || self.simple_input.destroyed() {
            event_loop.exit();
            // If you don't drop window, the app won't quit before any other window event.
            // https://github.com/rust-windowing/winit/issues/3673
            self.window.window = None;
            return;
        }
    }
}

macro_rules! gen_window_component_functions {
    ($fn_name:ident, $name:ident, String) => {
        /// see [winit::window::Window::$fn_name]
        pub fn $fn_name(&mut self, value: String) {
            if let Some(window) = self.window.as_mut() {
                window.$fn_name(&value);
            } else {
                self.default_attributes.$name = value;
            }
        }
    };
    ($fn_name:ident, $name:ident, winit::dpi::Position) => {
        /// see [winit::window::Window::$fn_name]
        pub fn $fn_name(&mut self, value: winit::dpi::Position) {
            if let Some(window) = self.window.as_mut() {
                window.$fn_name(value);
            } else {
                self.default_attributes.$name = Some(value);
            }
        }
    };
    ($fn_name:ident, $name:ident, $data_type:ty) => {
        /// see [winit::window::Window::$fn_name]
        pub fn $fn_name(&mut self, value: $data_type) {
            if let Some(window) = self.window.as_mut() {
                window.$fn_name(value);
            } else {
                self.default_attributes.$name = value;
            }
        }
    };
}

impl Window {
    /// create a new window
    pub fn new(default_attributes: winit::window::WindowAttributes) -> Self {
        Self {
            window: None,
            default_attributes,
            should_close: false,
        }
    }

    /// close the engine window
    pub fn close_engine(&mut self) {
        self.should_close = true;
    }

    // ====================================================== WINDOW SETTERS ====================================================== //
    //MARK: SETTERS

    gen_window_component_functions!(set_min_inner_size, min_inner_size, Option<winit::dpi::Size>);
    gen_window_component_functions!(set_max_inner_size, max_inner_size, Option<winit::dpi::Size>);
    gen_window_component_functions!(set_resizable, resizable, bool);
    gen_window_component_functions!(
        set_enabled_buttons,
        enabled_buttons,
        winit::window::WindowButtons
    );
    gen_window_component_functions!(set_maximized, maximized, bool);
    gen_window_component_functions!(set_visible, visible, bool);
    gen_window_component_functions!(set_transparent, transparent, bool);
    gen_window_component_functions!(set_blur, blur, bool);
    gen_window_component_functions!(set_decorations, decorations, bool);
    gen_window_component_functions!(set_window_icon, window_icon, Option<winit::window::Icon>);
    gen_window_component_functions!(
        set_resize_increments,
        resize_increments,
        Option<winit::dpi::Size>
    );
    gen_window_component_functions!(set_content_protected, content_protected, bool);
    gen_window_component_functions!(set_window_level, window_level, winit::window::WindowLevel);
    gen_window_component_functions!(set_cursor, cursor, winit::window::Cursor);
    gen_window_component_functions!(set_outer_position, position, winit::dpi::Position);
    gen_window_component_functions!(set_title, title, String);
    gen_window_component_functions!(set_theme, preferred_theme, Option<winit::window::Theme>);

    /// use `set_theme`
    #[deprecated]
    pub fn set_preferred_theme(&mut self, value: Option<winit::window::Theme>) {
        log::warn!("the function set_preferred_theme has been deprecated, use set_theme instead");
        if let Some(window) = self.window.as_mut() {
            window.set_theme(value);
        } else {
            self.default_attributes.preferred_theme = value;
        }
    }

    /// see [winit::window::Window::set_fullscreen]
    pub fn set_fullscreen_borderless(&mut self, value: bool) {
        let full_screen_result = if value {
            Some(winit::window::Fullscreen::Borderless(None))
        } else {
            None
        };

        if let Some(window) = self.window.as_mut() {
            window.set_fullscreen(full_screen_result);
        } else {
            self.default_attributes.fullscreen = full_screen_result;
        }
    }

    /// see [winit::window::Window::set_fullscreen]
    ///
    /// **Does not work unless during update_loop**
    pub fn set_fullscreen_exclusive(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            let full_screen_result = match value {
                true => match window.available_monitors().next() {
                    Some(monitor) => match monitor.video_modes().next() {
                        Some(vide_mode) => Some(winit::window::Fullscreen::Exclusive(vide_mode)),
                        None => None,
                    },
                    None => None,
                },
                false => None,
            };

            window.set_fullscreen(full_screen_result);
        } else {
            self.default_attributes.fullscreen = None;
        }
    }
}
