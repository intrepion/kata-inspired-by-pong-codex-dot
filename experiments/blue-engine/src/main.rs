use blue_engine::{prelude::*, primitive_shapes::square};
use pong_core::*;
use std::time::Instant;

#[cfg(any(feature = "windowed", test))]
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
#[cfg(any(feature = "windowed", test))]
#[derive(Default)]
struct MovementGate { suppressed: [bool; 4], was_playing: bool }
#[cfg(any(feature = "windowed", test))]
impl MovementGate {
    fn axis(&mut self, held: [bool; 4], entering_pause: bool) -> f32 {
        for (blocked, down) in self.suppressed.iter_mut().zip(held) {
            *blocked = down && (entering_pause || *blocked);
        }
        let active = std::array::from_fn::<_, 4, _>(|i| held[i] && !self.suppressed[i]);
        (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
    }
}
#[cfg(any(feature = "windowed", test))]
impl MovementGate {
    fn finish_frame(&mut self, state: State) {
        self.was_playing &= state == State::Playing;
    }
}
#[cfg(any(feature = "windowed", test))]
fn sample_movement(state: State, gate: &mut MovementGate, held: [bool; 4], entering_pause: bool) -> f32 {
    let playing = state == State::Playing;
    let suppress = entering_pause || !playing || !gate.was_playing;
    gate.was_playing = playing;
    gate.axis(held, suppress)
}
#[cfg(any(feature = "windowed", test))]
fn restart(game: &mut Game, gate: &mut MovementGate) { gate.was_playing = false; game.start(); }
#[cfg(any(feature = "windowed", test))]
fn status_label(game: &Game) -> &'static str {
    match game.state {
        State::Ready => "Ready", State::Playing => "Playing", State::Paused => "Paused",
        State::Over if game.scores[0] > game.scores[1] => "YOU WIN",
        State::Over if game.scores[1] > game.scores[0] => "CPU WINS",
        State::Over => "Match over",
    }
}
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
    #[cfg(feature = "capture-headless")]
    assert!(
        !std::env::args().any(|a| a == "--benchmark"),
        "benchmark requires windowed build"
    );
    #[cfg(feature = "windowed")]
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    #[cfg(feature = "windowed")]
    let mut benchmark: Option<pong_benchmark::Frames> = None;
    #[cfg(feature = "windowed")]
    let mut benchmark_done = false;
    #[cfg(feature = "windowed")]
    let mut scenario = pong_scenario::Scenario::default();
    let mut engine = Engine::new_config(EngineSettings {
        width: if cfg!(feature = "capture-headless") {
            1024
        } else {
            1000
        },
        height: if cfg!(feature = "capture-headless") {
            640
        } else {
            680
        },
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
    let mut game = Game::new(pong_scenario::SEED);
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
    let mut movement_gate = MovementGate::default();
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
        if benchmarking {
            accumulator+=dt;
            while accumulator>=pong_scenario::STEP {
                if scenario.finished() { scenario=pong_scenario::Scenario::default();game=Game::new(pong_scenario::SEED); }
                scenario.advance(&mut game);
                accumulator-=pong_scenario::STEP;
            }
        }
        #[cfg(feature="windowed")]
        if demo.is_none() && !benchmarking {
        let focused = engine.window.as_ref().is_some_and(|window| window.has_focus());
        if !focused {
            let input = &engine.simple_input;
            let held = [input.key_held(KeyCode::KeyW), input.key_held(KeyCode::ArrowUp), input.key_held(KeyCode::KeyS), input.key_held(KeyCode::ArrowDown)];
            sample_movement(State::Paused, &mut movement_gate, held, true);
            game.pause();
            accumulator = 0.;
        } else {
            let input = &engine.simple_input;
            let was_playing = game.state == State::Playing;
            if input.key_pressed(KeyCode::Enter) {
                if game.state == State::Paused { game.toggle_pause(); }
                else if game.state != State::Playing { game.start(); }
            }
            if input.key_pressed(KeyCode::KeyR) { restart(&mut game, &mut movement_gate); }
            if input.key_pressed(KeyCode::Space) { game.toggle_pause(); }
            if input.key_pressed(KeyCode::Escape) { game.pause(); }
            let held = [input.key_held(KeyCode::KeyW), input.key_held(KeyCode::ArrowUp), input.key_held(KeyCode::KeyS), input.key_held(KeyCode::ArrowDown)];
            let axis = sample_movement(game.state, &mut movement_gate, held, was_playing && game.state == State::Paused);
            accumulator += dt;
            while accumulator >= 1. / 120. { game.step(1. / 120., axis); accumulator -= 1. / 120.; }
            movement_gate.finish_frame(game.state);
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
            let next = format!("Pong Blue Engine | YOU {} : {} CPU | {} | Enter: play | W/S/arrows | Space: pause | R: restart",game.scores[0],game.scores[1],status_label(&game));
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
        #[cfg(feature="windowed")]
        if benchmarking && !benchmark_done {
            let (width,height)=(engine.renderer.config.width,engine.renderer.config.height);
            let frames=benchmark.get_or_insert_with(||pong_benchmark::Frames::new(width,height));
            if frames.frame(width,height) {
                benchmark_done=true;
                println!("PONG_BENCHMARK {}",frames.report());
                engine.window.close_engine();
            }
        }
        #[cfg(feature = "capture-headless")]
        if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
            // Rendering precedes changed-object upload in the framework loop.
            pending = Some((path, 1));
        }
    })?;
    // Blue Engine drops its window before camera/object GPU references. Keep the
    // native display alive until all renderer-owned resources have been dropped.
    #[cfg(feature = "windowed")]
    {
        let window_lifetime = engine.window.window.clone();
        drop(engine);
        drop(window_lifetime);
    }
    Ok(())
}

#[cfg(test)]
mod benchmark_tests {
    use super::benchmark_args;
    #[test]
    fn active_restart_suppresses_movement_until_fresh_press() {
        use super::*;
        let mut game = Game::new(42); game.start();
        let mut gate = MovementGate::default();
        sample_movement(game.state, &mut gate, [false; 4], false);
        let held = [true, false, false, false];
        assert_eq!(sample_movement(game.state, &mut gate, held, false), -1.);
        restart(&mut game, &mut gate);
        assert_eq!(sample_movement(game.state, &mut gate, held, false), 0.);
        sample_movement(game.state, &mut gate, [false; 4], false);
        assert_eq!(sample_movement(game.state, &mut gate, held, false), -1.);
    }
    #[test]
    fn opt_in_only() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "distinct")]
    fn reject_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    #[test]
    fn status_names_winner_and_clears_after_restart() {
        let mut game = Game::new(42);
        assert_eq!(status_label(&game), "Ready");
        game.start(); assert_eq!(status_label(&game), "Playing");
        game.pause(); assert_eq!(status_label(&game), "Paused");
        for (scores, x, vx, expected) in [([6, 2], W + BALL - 1., 350., "YOU WIN"), ([2, 6], -BALL + 1., -350., "CPU WINS")] {
            game.start(); game.scores = scores; game.serve = 0.; game.ball.x = x; game.ball.y = H / 2.; game.ball.vx = vx; game.ball.vy = 0.;
            game.step(0.02, 0.); assert_eq!(status_label(&game), expected);
            game.start(); assert_eq!(status_label(&game), "Playing");
        }
    }
    #[test]
    fn first_playing_sample_suppresses_held_key() {
        let mut gate=MovementGate::default();
        assert_eq!(sample_movement(State::Playing,&mut gate,[true,false,false,false],false),0.);
        assert_eq!(sample_movement(State::Playing,&mut gate,[true,false,false,false],false),0.);
        sample_movement(State::Playing,&mut gate,[false;4],false);
        assert_eq!(sample_movement(State::Playing,&mut gate,[true,false,false,false],false),-1.);
    }
    #[test]
    fn winning_score_keeps_held_movement_suppressed_on_restart() {
        for i in 0..4 {
            for (scores,x,vx) in [([6,2],W+BALL-1.,350.),([2,6],-BALL+1.,-350.)] {
                let mut game=Game::new(42);game.start();let mut gate=MovementGate::default();
                let mut held=[false;4];sample_movement(game.state,&mut gate,held,false);
                held[i]=true;assert_ne!(sample_movement(game.state,&mut gate,held,false),0.);
                game.scores=scores;game.serve=0.;game.ball.x=x;game.ball.y=H/2.;game.ball.vx=vx;game.ball.vy=0.;
                game.step(0.02,0.);assert_eq!(game.state,State::Over);
                gate.finish_frame(game.state);game.start();assert_eq!(sample_movement(game.state,&mut gate,held,false),0.);
                held[i]=false;sample_movement(game.state,&mut gate,held,false);held[i]=true;assert_ne!(sample_movement(game.state,&mut gate,held,false),0.);
            }
        }
    }
    #[test]
    fn nonplaying_held_keys_do_not_move_after_resume() {
        for state in [State::Ready, State::Paused, State::Over] {
            for i in 0..4 {
                let mut gate=MovementGate::default(); let mut held=[false;4]; held[i]=true;
                assert_eq!(sample_movement(state,&mut gate,held,false),0.,"{state:?}");
                assert_eq!(sample_movement(State::Playing,&mut gate,held,false),0.);
                held[i]=false; sample_movement(State::Playing,&mut gate,held,false);
                held[i]=true; assert_ne!(sample_movement(State::Playing,&mut gate,held,false),0.);
            }
        }
    }

    #[test]
    fn held_movement_stays_suppressed_through_resume() {
        for index in 0..4 {
            let mut gate = MovementGate::default();
            let mut held = [false; 4];
            held[index] = true;
            assert_ne!(gate.axis(held, false), 0.);
            assert_eq!(gate.axis(held, true), 0.);
            assert_eq!(gate.axis(held, false), 0.);
            held[index] = false;
            assert_eq!(gate.axis(held, false), 0.);
            held[index] = true;
            assert_ne!(gate.axis(held, false), 0.);
        }
    }
    #[test]
    fn aliases_and_opposing_keys_remain_independent() {
        let mut gate = MovementGate::default();
        assert_eq!(gate.axis([true, false, false, false], true), 0.);
        assert_eq!(gate.axis([true, true, false, false], false), -1.);
        assert_eq!(gate.axis([true, true, true, false], false), 0.);
        assert_eq!(gate.axis([true, false, true, false], false), 1.);
        assert_eq!(gate.axis([false, false, true, false], false), 1.);
        assert_eq!(gate.axis([true, false, true, false], false), 0.);
    }

}
