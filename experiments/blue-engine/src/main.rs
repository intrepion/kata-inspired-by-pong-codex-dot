use blue_engine::{prelude::*, primitive_shapes::square};
use pong_core::*;
use std::time::Instant;

fn rect(engine: &mut Engine, name: &str, x: f32, y: f32, width: f32, height: f32, z: f32) {
    if let Some(object) = engine.objects.get_mut(name) {
        // Hide zero-sized score markers without creating a singular inverse matrix.
        object.set_visibility(width > 0. && height > 0.);
        if width <= 0. || height <= 0. {
            return;
        }
        // Blue Engine's square spans -1..1 and Y points up; pong-core uses top-down pixels.
        // set_scale is multiplicative in Blue Engine 0.10; resize resets the matrix.
        object.resize([width / 2., height / 2., 1.]);
        object.set_position([x + width / 2. - W / 2., H / 2. - y - height / 2., z]);
    }
}
#[cfg(all(feature = "windowed", feature = "capture-headless"))]
compile_error!("Use --no-default-features --features capture-headless for headless export");
fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new_config(EngineSettings {
        width: if cfg!(feature = "capture-headless") { 1024 } else { 1000 },
        height: if cfg!(feature = "capture-headless") { 640 } else { 680 },
        title: "Pong Blue Engine",
        present_mode: blue_engine::wgpu::PresentMode::AutoVsync,
        ..Default::default()
    })?;
    engine.renderer.set_clear_color(0.047, 0.129, 0.118, 1.);
    for name in ["court", "player", "cpu", "ball"] {
        square(
            name,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )?;
        let color = if name == "court" {
            [0.082, 0.247, 0.216]
        } else {
            [0.85, 1., 0.68]
        };
        engine
            .objects
            .get_mut(name)
            .unwrap()
            .set_color(color[0], color[1], color[2], 1.);
    }
    engine
        .objects
        .get_mut("ball")
        .unwrap()
        .set_color(1., 1., 1., 1.);
    rect(&mut engine, "court", 0., 0., W, H, -0.1);
    for i in 0..18 {
        let name = format!("net-{i}");
        square(
            &name,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )?;
        engine
            .objects
            .get_mut(name.as_str())
            .unwrap()
            .set_color(0.27, 0.43, 0.35, 1.);
        rect(&mut engine, &name, 449., i as f32 * 28., 2., 12., 0.);
    }
    for side in 0..2 {
        for point in 0..WIN {
            let name = format!("score-{side}-{point}");
            square(
                &name,
                ObjectSettings::default(),
                &mut engine.renderer,
                &mut engine.objects,
            )?;
            engine
                .objects
                .get_mut(name.as_str())
                .unwrap()
                .set_color(0.85, 1., 0.68, 1.);
            rect(&mut engine, &name, 0., 0., 0., 0., 0.);
        }
    }
    let mut game = Game::new(42);
    let mut demo = pong_scenario::Demo::from_args();
    #[cfg(feature = "windowed")]
    assert!(
        !std::env::args().any(|a| a == "--scripted-captures"),
        "Blue Engine capture requires --no-default-features --features capture-headless"
    );
    #[cfg(feature = "capture-headless")]
    assert!(demo.is_some(), "headless export requires --scripted-test");
    #[cfg(feature = "capture-headless")]
    let mut pending: Option<(std::path::PathBuf, u8)> = None;

    let mut previous = Instant::now();
    #[cfg(feature = "windowed")]
    let mut accumulator = 0.;
    #[cfg(feature = "windowed")]
    let mut title = String::new();
    #[cfg(feature = "windowed")]
    let mut dimensions = (0, 0);
    #[cfg(feature = "capture-headless")]
    {
        engine.camera.set_projection(Projection::Orthographic {
            zoom: (W + 30.).max((H + 30.) * 1.6),
        });
        engine.camera.update_view_projection(&mut engine.renderer);
    }
    engine.update_loop(move |engine| {
        let now = Instant::now();
        let dt = now.duration_since(previous).as_secs_f32().min(0.1);
        previous = now;
        #[cfg(feature = "capture-headless")]
        if let Some((path, remaining)) = &mut pending {
            if *remaining > 0 { *remaining -= 1; return; }
            let rgba = &engine.renderer.headless_texture_data;
            let (width, height) = (engine.renderer.config.width, engine.renderer.config.height);
            assert_eq!(rgba.len(), width as usize * height as usize * 4);
            use blue_engine::wgpu::TextureFormat;
            let bgra = match engine.renderer.config.format {
                TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => true,
                TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => false,
                other => panic!("Unsupported app-frame format {other:?}"),
            };
            let rgb: Vec<u32> = rgba.as_chunks::<4>().0.iter().map(|p| { let (r,b) = if bgra {(p[2],p[0])} else {(p[0],p[2])}; (u32::from(r)<<16)|(u32::from(p[1])<<8)|u32::from(b) }).collect();
            pong_scenario::Demo::save_rgb(path, &rgb, width, height);
            pending = None;
        }
        if demo.as_ref().is_some_and(|d| d.finished()) { std::process::exit(0); }
        if let Some(demo) = &mut demo { demo.advance(&mut game, dt); }
        #[cfg(feature = "windowed")]
        if demo.is_none() {
        let focused = engine.window.as_ref().is_some_and(|window| window.has_focus());
        if !focused {
            game.pause();
            accumulator = 0.;
        } else {
            let input = &engine.simple_input;
            if input.key_pressed(KeyCode::Enter) {
                if game.state == State::Paused { game.toggle_pause(); }
                else if game.state != State::Playing { game.start(); }
            }
            if input.key_pressed(KeyCode::KeyR) { game.start(); }
            if input.key_pressed(KeyCode::Space) { game.toggle_pause(); }
            if input.key_pressed(KeyCode::Escape) { game.pause(); }
            let axis = (input.key_held(KeyCode::KeyS) || input.key_held(KeyCode::ArrowDown)) as u8 as f32
                - (input.key_held(KeyCode::KeyW) || input.key_held(KeyCode::ArrowUp)) as u8 as f32;
            accumulator += dt;
            while accumulator >= 1. / 120. { game.step(1. / 120., axis); accumulator -= 1. / 120.; }
        }
        }
        #[cfg(feature = "windowed")]
        if let Some(window) = engine.window.as_ref() {
            let size = window.inner_size();
            if size.width > 0 && size.height > 0 && dimensions != (size.width, size.height) {
                dimensions = (size.width, size.height);
                let aspect = size.width as f32 / size.height as f32;
                engine.camera.set_projection(Projection::Orthographic { zoom: (W + 30.).max((H + 30.) * aspect) });
                engine.camera.update_view_projection(&mut engine.renderer);
            }
            let next = format!("Pong Blue Engine | YOU {} : {} CPU | {:?} | Enter: play | W/S/arrows | Space: pause | R: restart",game.scores[0],game.scores[1],game.state);
            if title != next { window.set_title(&next); title = next; }
        }
        rect(engine, "player", 32., game.player-PADDLE_H/2., PADDLE_W, PADDLE_H, 0.);
        rect(engine, "cpu", W-44., game.cpu-PADDLE_H/2., PADDLE_W, PADDLE_H, 0.);
        rect(engine, "ball", game.ball.x-BALL, game.ball.y-BALL, BALL*2., BALL*2., 0.);
        for side in 0..2 { for point in 0..WIN {
            let name = format!("score-{side}-{point}");
            let width = if point < game.scores[side] { 8. } else { 0. };
            rect(engine, &name, 330.+side as f32*170.+point as f32*12., 20., width, 18., 0.);
        }}
        #[cfg(feature = "capture-headless")]
        if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
            // Rendering precedes changed-object upload in the framework loop.
            pending = Some((path, 1));
        }
    })?;
    Ok(())
}
