use pong_low_level_common::{viewport, App, BACKGROUND};
use sdl3::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    pixels::Color,
    render::FRect,
};
use std::time::{Duration, Instant};
fn main() -> Result<(), String> {
    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window("Pong SDL3", 900, 500)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas();
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    let mut app = App::default();
    let mut demo = pong_scenario::Demo::from_args();
    let mut last = Instant::now();
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } if demo.is_none() => app.action(),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } if demo.is_none() => app.game.start(),
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } if demo.is_none() => app.blur(),
                _ => {}
            }
        }
        let k = events.keyboard_state();
        app.up = k.is_scancode_pressed(Scancode::W) || k.is_scancode_pressed(Scancode::Up);
        app.down = k.is_scancode_pressed(Scancode::S) || k.is_scancode_pressed(Scancode::Down);
        let now = Instant::now();
        if let Some(demo) = &mut demo {
            demo.advance(&mut app.game, (now - last).as_secs_f32());
        } else {
            app.tick((now - last).as_secs_f32());
        }
        last = now;
        canvas.set_draw_color(Color::RGB(BACKGROUND[0], BACKGROUND[1], BACKGROUND[2]));
        canvas.clear();
        let (width, height) = canvas.output_size().map_err(|e| e.to_string())?;
        let (s, ox, oy) = viewport(width, height);
        for r in app.rectangles() {
            canvas.set_draw_color(Color::RGB(r.color[0], r.color[1], r.color[2]));
            canvas
                .fill_rect(FRect::new(
                    ox + r.x * s,
                    oy + r.y * s,
                    (r.w * s).ceil(),
                    (r.h * s).ceil(),
                ))
                .map_err(|e| e.to_string())?;
        }
        if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
            // Read this renderer's current target BEFORE presentation/swap.
            let surface = canvas
                .read_pixels(None)
                .map_err(|e| e.to_string())?
                .convert_format(sdl3::pixels::PixelFormat::RGB24)
                .map_err(|e| e.to_string())?;
            let pitch = surface.pitch() as usize;
            surface.with_lock(|bytes| {
                let mut rgb = Vec::with_capacity(width as usize * height as usize);
                for row in bytes.chunks(pitch).take(height as usize) {
                    for c in row[..width as usize * 3].as_chunks::<3>().0.iter() {
                        rgb.push(
                            (u32::from(c[0]) << 16) | (u32::from(c[1]) << 8) | u32::from(c[2]),
                        );
                    }
                }
                pong_scenario::Demo::save_rgb(&path, &rgb, width, height);
            });
        }
        canvas.present();
        if demo.as_ref().is_some_and(|d| d.finished()) {
            break;
        }
        canvas
            .window_mut()
            .set_title(&app.title(if demo.is_some() {
                "SDL3 SCRIPTED"
            } else {
                "SDL3 renderer"
            }))
            .map_err(|e| e.to_string())?;
        std::thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}
