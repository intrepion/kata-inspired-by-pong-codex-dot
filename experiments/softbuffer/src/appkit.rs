use super::*;
use raw_window_handle::{
    AppKitWindowHandle, DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
    rc::Rc,
    time::Duration,
};

extern "C" {
    fn pong_host_new() -> *mut c_void;
    fn pong_host_free(host: *mut c_void);
    fn pong_host_view(host: *mut c_void) -> *mut c_void;
    fn pong_host_title(host: *mut c_void, title: *const c_char);
    fn pong_host_metrics(host: *mut c_void, width: *mut u32, height: *mut u32, scale: *mut f64);
    fn pong_host_poll(host: *mut c_void, seconds: f64, event: *mut HostEvent);
}

#[derive(Default)]
#[repr(C)]
struct HostEvent {
    action: u32,
    key: u32,
    repeat: u32,
    focused: u32,
    lost_focus: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum KeyCode {
    KeyW,
    ArrowUp,
    KeyS,
    ArrowDown,
    Space,
    KeyR,
    Escape,
}
impl KeyCode {
    fn from_id(code: u32) -> Option<Self> {
        use KeyCode::*;
        Some(match code {
            1 => KeyW,
            2 => ArrowUp,
            3 => KeyS,
            4 => ArrowDown,
            5 => Space,
            6 => KeyR,
            7 => Escape,
            _ => return None,
        })
    }
}

#[derive(Debug)]
struct Host(NonNull<c_void>);
impl Host {
    fn new() -> Self {
        Self(
            NonNull::new(unsafe { pong_host_new() }).expect("AppKit must start on the main thread"),
        )
    }
    fn ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
    fn metrics(&self) -> (u32, u32, f64) {
        let (mut w, mut h, mut scale) = (0, 0, 0.);
        unsafe {
            pong_host_metrics(self.ptr(), &mut w, &mut h, &mut scale);
        }
        (w, h, scale)
    }
    fn poll(&self, wait: Duration) -> HostEvent {
        let mut event = HostEvent::default();
        unsafe {
            pong_host_poll(self.ptr(), wait.as_secs_f64(), &mut event);
        }
        event
    }
    fn title(&self, title: &str) {
        let title = CString::new(title).expect("no embedded NUL in title");
        unsafe {
            pong_host_title(self.ptr(), title.as_ptr());
        }
    }
}
impl Drop for Host {
    fn drop(&mut self) {
        // NonNull is not Send; the strong host is shared only via Rc.
        unsafe {
            pong_host_free(self.ptr());
        }
    }
}
impl HasDisplayHandle for Host {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::appkit())
    }
}
impl HasWindowHandle for Host {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let view =
            NonNull::new(unsafe { pong_host_view(self.ptr()) }).ok_or(HandleError::Unavailable)?;
        // Host owns the window (which owns its content view) for this borrow.
        Ok(unsafe { WindowHandle::borrow_raw(AppKitWindowHandle::new(view).into()) })
    }
}

fn apply_event(frontend: &mut Frontend, event: &HostEvent) -> bool {
    let lost_focus = event.lost_focus != 0 || (frontend.focused && event.focused == 0);
    frontend.focused = event.focused != 0;
    if frontend.demo.is_none() && !frontend.benchmarking {
        if lost_focus {
            frontend.clear_movement();
        }
        if matches!(event.action, 1 | 2) {
            if let Some(key) = KeyCode::from_id(event.key) {
                return frontend.handle_focused_key(
                    frontend.focused,
                    key,
                    event.action == 1,
                    event.repeat != 0,
                );
            }
        }
    }
    false
}

pub(super) fn run(mut frontend: Frontend) {
    let host = Rc::new(Host::new());
    let context = softbuffer::Context::new(host.clone()).expect("AppKit context");
    let mut surface = softbuffer::Surface::new(&context, host.clone()).expect("AppKit surface");
    #[cfg(feature = "appkit-native-qa")]
    if std::env::args().any(|arg| arg == "--appkit-native-qa") {
        return native_qa(&host, &mut surface, &mut frontend);
    }
    if frontend.benchmarking {
        let (w, h, _) = host.metrics();
        frontend.benchmark = Some(pong_benchmark::Frames::new(w, h));
        frontend.app.game = pong_low_level_common::Game::new(pong_scenario::SEED);
    }
    let mut next = Instant::now();
    let mut title = String::new();
    loop {
        let event = host.poll(next.saturating_duration_since(Instant::now()));
        if event.action == 3 {
            break;
        }
        if apply_event(&mut frontend, &event) {
            break;
        }
        if Instant::now() < next {
            continue;
        }
        next = Instant::now() + Duration::from_millis(8);
        frontend.advance_frame();
        let (w, h, _scale) = host.metrics();
        if let (Some(width), Some(height)) = (NonZeroU32::new(w), NonZeroU32::new(h)) {
            surface
                .resize(width, height)
                .expect("valid AppKit dimensions");
            let mut pixels = surface.buffer_mut().expect("AppKit pixels");
            pong_low_level_common::raster(&frontend.app, &mut pixels, w, h);
            if let Some(path) = frontend.demo.as_mut().and_then(|d| d.capture_path()) {
                pong_scenario::Demo::save_rgb(&path, &pixels, w, h);
            }
            pixels.present().expect("AppKit present");
            if let Some(bench) = &mut frontend.benchmark {
                if bench.frame(w, h) {
                    println!("PONG_BENCHMARK {}", bench.report());
                    break;
                }
            }
        }
        let label = frontend.app.title(if frontend.benchmarking {
            "softbuffer / BENCHMARK"
        } else if frontend.demo.is_some() {
            "softbuffer / SCRIPTED"
        } else {
            "softbuffer / CPU"
        });
        if label != title {
            host.title(&label);
            title = label;
        }
        if frontend.demo.as_ref().is_some_and(|d| d.finished()) {
            break;
        }
    }
    // Drop the pixel layer (and its borrowed/retained NSView) before window.
    drop(surface);
}

#[cfg(feature = "appkit-native-qa")]
fn native_qa(host: &Host, surface: &mut softbuffer::Surface<Rc<Host>, Rc<Host>>, f: &mut Frontend) {
    extern "C" {
        fn pong_host_qa_resize(host: *mut c_void, w: u32, h: u32);
        fn pong_host_qa_focus(host: *mut c_void, focused: i32);
        fn pong_host_qa_key(host: *mut c_void, key: u32, down: i32, repeat: i32, modifiers: u32);
        fn pong_host_qa_pixels(
            host: *mut c_void,
            w: u32,
            h: u32,
            samples: *mut u32,
            scale: *mut f64,
        ) -> i32;
        fn pong_host_qa_close(host: *mut c_void);
    }
    assert!(std::thread::spawn(|| unsafe { pong_host_new().is_null() })
        .join()
        .unwrap());
    assert!(surface.resize(NonZeroU32::MAX, NonZeroU32::MAX).is_err());
    // Registration and event queue startup complete before probing focus.
    for _ in 0..10 {
        host.poll(Duration::from_millis(4));
    }
    let (mut tested_scale, mut tested_dimensions) = (0.0, (0, 0));
    for (i, (logical_width, logical_height)) in [
        (360, 200),
        (780, 440),
        (492, 271),
        (812, 451),
        (360, 200),
        (780, 440),
    ]
    .into_iter()
    .enumerate()
    {
        unsafe {
            pong_host_qa_resize(host.ptr(), logical_width, logical_height);
        }
        for _ in 0..5 {
            host.poll(Duration::from_millis(4));
        }
        let (w, h, scale) = host.metrics();
        assert_eq!(
            (w, h),
            (
                (logical_width as f64 * scale).round() as u32,
                (logical_height as f64 * scale).round() as u32
            )
        );
        surface
            .resize(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap())
            .unwrap();
        let mut b = surface.buffer_mut().unwrap();
        let mut colors = [0x00ff0000, 0x0000ff00, 0x000000ff, 0x00ffff00];
        colors.rotate_left(i % 4);
        for (y, row) in b.chunks_exact_mut(w as usize).enumerate() {
            let half = w as usize / 2;
            let pair = if y < h as usize / 2 { 0 } else { 2 };
            row[..half].fill(colors[pair]);
            row[half..].fill(colors[pair + 1]);
        }
        b.present().unwrap();
        let mut layer_scale = 0.0;
        let mut samples = [0; 4];
        assert_eq!(
            unsafe {
                pong_host_qa_pixels(host.ptr(), w, h, samples.as_mut_ptr(), &mut layer_scale)
            },
            1
        );
        assert_eq!(
            colors, samples,
            "asymmetric corner orientation/crop probe {i}"
        );
        assert_eq!(scale, layer_scale);
        tested_scale = scale;
        tested_dimensions = (w, h);
    }
    unsafe {
        pong_host_qa_focus(host.ptr(), 1);
    }
    assert!((0..10).any(|_| host.poll(Duration::from_millis(20)).focused == 1));
    f.app.game.start();
    let key = |f: &mut Frontend, code, down, repeat, modifiers, expected| {
        unsafe {
            pong_host_qa_key(host.ptr(), code, down, repeat, modifiers);
        }
        let event = host.poll(Duration::from_millis(20));
        assert_eq!((event.action, event.key, event.repeat), expected);
        assert!(!apply_event(f, &event));
    };
    for modifier in [1, 2] {
        // A shortcut down reaches AppKit, and an unmodified OS repeat of
        // that same held key does not start movement.
        key(f, 13, 1, 0, modifier, (4, 0, 0));
        assert!(!f.app.up);
        key(f, 13, 1, 1, 0, (1, 1, 1));
        assert!(!f.app.up);
        key(f, 13, 0, 0, 0, (2, 1, 0));
        // Two aliases held: modified W-up clears only W, then modified
        // Up-up fully clears it. R with modifiers must not restart.
        key(f, 13, 1, 0, 0, (1, 1, 0));
        key(f, 126, 1, 0, 0, (1, 2, 0));
        key(f, 13, 0, 0, modifier, (2, 1, 0));
        assert_eq!(&f.movement_held[..2], &[false, true]);
        assert!(f.app.up);
        key(f, 126, 0, 0, modifier, (2, 2, 0));
        assert!(!f.app.up);
        f.app.game.scores = [2, 1];
        key(f, 15, 1, 0, modifier, (4, 0, 0));
        assert_eq!(f.app.game.scores, [2, 1]);
    }
    key(f, 13, 1, 0, 0, (1, 1, 0));
    assert!(f.app.up);
    unsafe {
        pong_host_qa_focus(host.ptr(), 0);
    }
    let blur = host.poll(Duration::from_millis(20));
    assert_eq!((blur.focused, blur.lost_focus), (0, 1));
    assert!(!apply_event(f, &blur));
    assert_eq!(f.app.game.state, pong_low_level_common::State::Paused);
    assert!(!f.app.up && f.pending_movement == [false; 2]);
    unsafe {
        pong_host_qa_focus(host.ptr(), 1);
    }
    assert!((0..10).any(|_| host.poll(Duration::from_millis(20)).focused == 1));
    key(f, 49, 1, 0, 0, (1, 5, 0));
    assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
    key(f, 13, 1, 1, 0, (1, 1, 1));
    assert!(!f.app.up);
    key(f, 13, 0, 0, 1, (2, 1, 0));
    key(f, 13, 1, 0, 0, (1, 1, 0));
    assert!(f.app.up);
    key(f, 13, 0, 0, 2, (2, 1, 0));
    assert!(!f.app.up);
    unsafe {
        pong_host_qa_close(host.ptr());
    }
    assert_eq!(host.poll(Duration::from_millis(20)).action, 3);
    println!("AppKit native QA: asymmetric app-owned layer readback, six native resizes, backing={tested_dimensions:?} scale={tested_scale}, synthetic Command/Control routing, movement alias releases, suppressed repeats, retained hold cleared on blur/refocus, native close and off-main rejection");
}
