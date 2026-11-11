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
                } => app.action(),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } => app.game.start(),
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => app.blur(),
                _ => {}
            }
        }
        let k = events.keyboard_state();
        app.up = k.is_scancode_pressed(Scancode::W) || k.is_scancode_pressed(Scancode::Up);
        app.down = k.is_scancode_pressed(Scancode::S) || k.is_scancode_pressed(Scancode::Down);
        let now = Instant::now();
        app.tick((now - last).as_secs_f32());
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
        canvas.present();
        canvas
            .window_mut()
            .set_title(&app.title("SDL3 renderer"))
            .map_err(|e| e.to_string())?;
        std::thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}
