use pong_low_level_common::{viewport, App, BACKGROUND};
use sdl3::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    pixels::Color,
    render::FRect,
};
use std::time::{Duration, Instant};
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
#[derive(Default)]
struct MovementGate {
    suppressed: [bool; 4],
    was_playing: bool,
    held: (bool, bool),
    pending: [bool; 2],
}
impl MovementGate {
    fn finish_frame(&mut self, state: pong_low_level_common::State) {
        self.was_playing &= state == pong_low_level_common::State::Playing;
        if !self.was_playing {
            self.held = (false, false);
            self.pending = [false; 2];
        }
    }
    fn take_axis(&mut self) -> f32 {
        let [up, down] = std::mem::take(&mut self.pending);
        (self.held.1 || down) as u8 as f32 - (self.held.0 || up) as u8 as f32
    }
    fn directions(&mut self, held: [bool; 4], entering_pause: bool) -> (bool, bool) {
        for (blocked, down) in self.suppressed.iter_mut().zip(held) {
            *blocked = down && (entering_pause || *blocked);
        }
        let active = std::array::from_fn::<_, 4, _>(|i| held[i] && !self.suppressed[i]);
        (active[0] || active[1], active[2] || active[3])
    }
}
fn sample_movement(
    app: &App,
    gate: &mut MovementGate,
    held: [bool; 4],
    entering_pause: bool,
) -> (bool, bool) {
    let playing = app.game.state == pong_low_level_common::State::Playing;
    let suppress = entering_pause || !playing || !gate.was_playing;
    gate.was_playing = playing;
    if suppress { gate.pending = [false; 2]; }
    gate.held = gate.directions(held, suppress);
    gate.held
}
fn sample_live(app: &mut App, gate: &mut MovementGate, held: [bool; 4],
               pressed: [bool; 4], entering_pause: bool) {
    let active = gate.was_playing && !entering_pause
        && app.game.state == pong_low_level_common::State::Playing;
    let (up, down) = gate.held;
    (app.up, app.down) = sample_movement(app, gate, held, entering_pause);
    if active {
        // Same-direction aliases coalesce; a sampled held direction does not queue a tail.
        let fresh = |i: usize| pressed[i] && !gate.suppressed[i];
        gate.pending[0] |= !up && (fresh(0) || fresh(1));
        gate.pending[1] |= !down && (fresh(2) || fresh(3));
    }
}
fn restart(app: &mut App, gate: &mut MovementGate) {
    gate.was_playing = false;
    app.restart();
}
fn lose_focus(app: &mut App, gate: &mut MovementGate) {
    gate.was_playing = false;
    app.blur();
}
fn explicit_action(app: &mut App) -> bool {
    let previous = app.game.state;
    app.action();
    previous == pong_low_level_common::State::Playing
        && app.game.state == pong_low_level_common::State::Paused
}
#[derive(Clone, Copy)]
enum LiveAction {
    Pause,
    Restart,
}
fn live_action(app: &mut App, gate: &mut MovementGate, focused: bool, action: LiveAction) -> bool {
    if !focused {
        return false;
    }
    match action {
        LiveAction::Pause => explicit_action(app),
        LiveAction::Restart => {
            restart(app, gate);
            false
        }
    }
}
fn exit_requested(event: &Event, focused: bool) -> bool {
    matches!(event, Event::Quit { .. })
        || (focused && matches!(event, Event::KeyDown { keycode: Some(Keycode::Escape), .. }))
}
// Read movement presses from the same drained batch as action events.
fn collect_press(event: &Event, pressed: &mut [bool; 4]) {
    if let Event::KeyDown { scancode: Some(code), repeat: false, .. } = event {
        let i = match *code {
            Scancode::W => 0, Scancode::Up => 1, Scancode::S => 2, Scancode::Down => 3,
            _ => return,
        };
        pressed[i] = true;
    }
}

fn main() -> Result<(), String> {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window("Pong SDL3", 900, 500)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas();
    if benchmarking {
        println!(
            "PONG_BENCHMARK_BACKEND {{\"video_driver\":{:?},\"renderer\":{:?}}}",
            video.current_video_driver(),
            canvas.renderer_name
        );
    }
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    let mut app = App::default();
    let mut movement = MovementGate::default();
    let mut demo = pong_scenario::Demo::from_args();
    let (width, height) = canvas.output_size().map_err(|e| e.to_string())?;
    let mut benchmark = benchmarking.then(|| pong_benchmark::Frames::new(width, height));
    let mut scenario = pong_scenario::Scenario::default();
    let mut accumulator = 0.;
    if benchmarking {
        app.game = pong_low_level_common::Game::new(pong_scenario::SEED);
    }
    let mut last = Instant::now();
    let mut focused = canvas.window().has_input_focus();
    'running: loop {
        let mut entered_pause = false;
        let mut pressed = [false; 4];
        for event in events.poll_iter() {
            if exit_requested(&event, focused) { break 'running; }
            if demo.is_none() && !benchmarking && focused {
                collect_press(&event, &mut pressed);
            }
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } if demo.is_none() && !benchmarking => {
                    entered_pause |=
                        live_action(&mut app, &mut movement, focused, LiveAction::Pause)
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } if demo.is_none() && !benchmarking => {
                    live_action(&mut app, &mut movement, focused, LiveAction::Restart);
                }
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } if demo.is_none() && !benchmarking => {
                    focused = false;
                    lose_focus(&mut app, &mut movement);
                }
                Event::Window {
                    win_event: WindowEvent::FocusGained,
                    ..
                } => focused = true,
                _ => {}
            }
        }
        let k = events.keyboard_state();
        sample_live(
            &mut app,
            &mut movement,
            [
                k.is_scancode_pressed(Scancode::W),
                k.is_scancode_pressed(Scancode::Up),
                k.is_scancode_pressed(Scancode::S),
                k.is_scancode_pressed(Scancode::Down),
            ],
            pressed,
            entered_pause,
        );
        let now = Instant::now();
        if benchmarking {
            accumulator += (now - last).as_secs_f32().clamp(0., 0.1);
            while accumulator >= pong_scenario::STEP {
                if scenario.finished() {
                    scenario = pong_scenario::Scenario::default();
                    app.game = pong_low_level_common::Game::new(pong_scenario::SEED);
                }
                scenario.advance(&mut app.game);
                accumulator -= pong_scenario::STEP;
            }
        } else if let Some(demo) = &mut demo {
            demo.advance(&mut app.game, (now - last).as_secs_f32());
        } else {
            app.tick_with_axis((now - last).as_secs_f32(), || movement.take_axis());
            movement.finish_frame(app.game.state);
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
        // Present-return callback boundary, not a display/GPU completion timestamp.
        if let Some(bench) = &mut benchmark {
            if bench.frame(width, height) {
                println!("PONG_BENCHMARK {}", bench.report());
                break;
            }
        }
        if demo.as_ref().is_some_and(|d| d.finished()) {
            break;
        }
        canvas
            .window_mut()
            .set_title(&app.title(if benchmarking {
                "SDL3 BENCHMARK"
            } else if demo.is_some() {
                "SDL3 SCRIPTED"
            } else {
                "SDL3 renderer"
            }))
            .map_err(|e| e.to_string())?;
        std::thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queued_escape_after_focus_loss_does_not_exit() {
        let escape = Event::KeyDown { timestamp: 0, window_id: 1, keycode: Some(Keycode::Escape), scancode: Some(Scancode::Escape), keymod: sdl3::keyboard::Mod::NOMOD, repeat: false, which: 0, raw: 0 };
        assert!(!exit_requested(&escape, false));
        assert!(exit_requested(&escape, true));
        assert!(exit_requested(&Event::Quit { timestamp: 0 }, false));
        assert!(exit_requested(&Event::Quit { timestamp: 0 }, true));
    }
    #[test]
    fn unfocused_actions_cannot_resume_or_restart_game() {
        let mut app = App::default();
        app.game.start();
        app.game.scores = [3, 2];
        let mut gate = MovementGate::default();
        lose_focus(&mut app, &mut gate);
        live_action(&mut app, &mut gate, false, LiveAction::Pause);
        assert_eq!(app.game.state, pong_low_level_common::State::Paused);
        live_action(&mut app, &mut gate, false, LiveAction::Restart);
        assert_eq!(app.game.state, pong_low_level_common::State::Paused);
        assert_eq!(app.game.scores, [3, 2]);
        live_action(&mut app, &mut gate, true, LiveAction::Pause);
        assert_eq!(app.game.state, pong_low_level_common::State::Playing);
    }
    #[test]
    fn focus_loss_then_immediate_resume_suppresses_held_direction() {
        let mut app = App::default();
        app.game.start();
        let mut gate = MovementGate::default();
        sample_movement(&app, &mut gate, [false; 4], false);
        let held = [true, false, false, false];
        assert_eq!(sample_movement(&app, &mut gate, held, false), (true, false));
        lose_focus(&mut app, &mut gate);
        assert!(!explicit_action(&mut app));
        assert_eq!(
            sample_movement(&app, &mut gate, held, false),
            (false, false)
        );
        assert_eq!(
            sample_movement(&app, &mut gate, held, false),
            (false, false)
        );
        sample_movement(&app, &mut gate, [false; 4], false);
        assert_eq!(sample_movement(&app, &mut gate, held, false), (true, false));
    }
    #[test]
    fn active_restart_suppresses_movement_until_fresh_press() {
        let mut app = App::default();
        app.game.start();
        let mut gate = MovementGate::default();
        sample_movement(&app, &mut gate, [false; 4], false);
        let held = [true, false, false, false];
        assert_eq!(sample_movement(&app, &mut gate, held, false), (true, false));
        restart(&mut app, &mut gate);
        assert_eq!(
            sample_movement(&app, &mut gate, held, false),
            (false, false)
        );
        sample_movement(&app, &mut gate, [false; 4], false);
        assert_eq!(sample_movement(&app, &mut gate, held, false), (true, false));
    }
    #[test]
    fn winning_score_keeps_held_movement_suppressed_on_restart() {
        use pong_low_level_common::{State, BALL, H, W};
        for i in 0..4 {
            for (scores, x, vx) in [([6, 2], W + BALL - 1., 350.), ([2, 6], -BALL + 1., -350.)] {
                let mut app = App::default();
                app.game.start();
                let mut gate = MovementGate::default();
                sample_movement(&app, &mut gate, [false; 4], false);
                let mut held = [false; 4];
                held[i] = true;
                assert_ne!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false)
                );
                app.game.scores = scores;
                app.game.serve = 0.;
                app.game.ball.x = x;
                app.game.ball.y = H / 2.;
                app.game.ball.vx = vx;
                app.game.ball.vy = 0.;
                app.game.step(0.02, 0.);
                assert_eq!(app.game.state, State::Over);
                gate.finish_frame(app.game.state);
                explicit_action(&mut app);
                assert_eq!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false)
                );
                held[i] = false;
                sample_movement(&app, &mut gate, held, false);
                held[i] = true;
                assert_ne!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false)
                );
            }
        }
    }
    #[test]
    fn game_over_between_samples_blocks_held_on_immediate_restart() {
        let mut app = App::default();
        app.game.start();
        let mut gate = MovementGate::default();
        sample_movement(&app, &mut gate, [false; 4], false);
        app.game.state = pong_low_level_common::State::Over;
        gate.finish_frame(app.game.state);
        app.game.start();
        assert_eq!(
            sample_movement(&app, &mut gate, [true, false, false, false], false),
            (false, false)
        );
    }
    #[test]
    fn movement_held_on_same_frame_as_resume_stays_suppressed() {
        let mut app = App::default();
        app.game.state = pong_low_level_common::State::Paused;
        let mut gate = MovementGate::default();
        let held = [true, false, false, false];
        explicit_action(&mut app);
        assert_eq!(
            sample_movement(&app, &mut gate, held, false),
            (false, false)
        );
        assert_eq!(
            sample_movement(&app, &mut gate, held, false),
            (false, false)
        );
    }
    #[test]
    fn nonplaying_held_keys_do_not_move_after_resume() {
        use pong_low_level_common::State;
        for state in [State::Ready, State::Paused, State::Over] {
            for i in 0..4 {
                let mut app = App::default();
                app.game.state = state;
                let mut gate = MovementGate::default();
                let mut held = [false; 4];
                held[i] = true;
                assert_eq!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false),
                    "{state:?}"
                );
                explicit_action(&mut app);
                assert_eq!(app.game.state, State::Playing);
                assert_eq!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false)
                );
                held[i] = false;
                sample_movement(&app, &mut gate, held, false);
                held[i] = true;
                assert_ne!(
                    sample_movement(&app, &mut gate, held, false),
                    (false, false)
                );
            }
        }
    }

    #[test]
    fn pause_suppression_keeps_aliases_independent() {
        let mut gate = MovementGate::default();
        assert_eq!(
            gate.directions([true, true, false, false], true),
            (false, false)
        );
        assert_eq!(
            gate.directions([true, false, false, false], false),
            (false, false)
        );
        assert_eq!(
            gate.directions([true, true, false, false], false),
            (true, false)
        );
        assert_eq!(
            gate.directions([true, true, true, false], false),
            (true, true)
        );
    }
    #[test]
    fn explicit_pause_suppresses_held_scancodes_until_release() {
        for index in 0..4 {
            let mut app = App::default();
            let mut gate = MovementGate::default();
            let mut held = [false; 4];
            assert!(!explicit_action(&mut app));
            held[index] = true;
            assert_ne!(gate.directions(held, false), (false, false));
            assert!(explicit_action(&mut app));
            assert_eq!(gate.directions(held, true), (false, false));
            assert!(!explicit_action(&mut app));
            assert_eq!(gate.directions(held, false), (false, false));
            held[index] = false;
            assert_eq!(gate.directions(held, false), (false, false));
            held[index] = true;
            assert_ne!(gate.directions(held, false), (false, false));
        }
    }
    #[test]
    fn benchmark_is_opt_in() {
        assert!(!benchmark_args(&[]));
        assert!(!benchmark_args(&["--scripted-test".into()]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "benchmark and replay/capture modes are distinct")]
    fn rejects_capture_mixing() {
        benchmark_args(&["--benchmark".into(), "--scripted-captures".into()]);
    }
}

#[cfg(test)]
mod transition_clock_adapter_test {
    use super::*;
    #[test]
    fn focused_restart_discards_stale_frame_then_advances() {
        let mut app=App::default();
        let mut gate=MovementGate::default();
        live_action(&mut app,&mut gate,true,LiveAction::Restart);
        app.down=true;
        app.tick(0.1);
        assert_eq!(app.game.player,250.);
        app.tick(1./60.);
        assert!(app.game.player>250.);
    }
}


#[cfg(test)]
mod quick_tap_tests {
    use super::*;

    fn key_event(code: Scancode, down: bool, repeat: bool) -> Event {
        if down {
            Event::KeyDown { timestamp: 0, window_id: 1, keycode: None,
                scancode: Some(code), keymod: sdl3::keyboard::Mod::NOMOD,
                repeat, which: 0, raw: 0 }
        } else {
            Event::KeyUp { timestamp: 0, window_id: 1, keycode: None,
                scancode: Some(code), keymod: sdl3::keyboard::Mod::NOMOD,
                repeat, which: 0, raw: 0 }
        }
    }

    #[test]
    fn drained_quick_tap_survives_zero_step_frame() {
        let mut app = App::default();
        let mut gate = MovementGate::default();
        app.game.start();
        sample_movement(&app, &mut gate, [false; 4], false);
        let start = app.game.player;
        let mut pressed = [false; 4];
        for event in [key_event(Scancode::W, true, false), key_event(Scancode::W, false, false)] {
            collect_press(&event, &mut pressed);
        }
        sample_live(&mut app, &mut gate, [false; 4], pressed, false);
        app.tick_with_axis(0.004, || gate.take_axis());
        assert_eq!(app.game.player, start);
        sample_live(&mut app, &mut gate, [false; 4], [false; 4], false);
        let mut axes = Vec::new();
        app.tick_with_axis(0.022, || { let a = gate.take_axis(); axes.push(a); a });
        assert_eq!(axes, [-1., 0., 0.]);
        assert!((start - app.game.player - 470. / 120.).abs() < 0.001,
            "expected exactly one step; paddle moved {}", start - app.game.player);
    }

    #[test]
    fn held_aliases_repeats_opposites_and_transitions_preserve_movement_rules() {
        let mut app = App::default();
        let mut gate = MovementGate::default();
        app.game.start();
        sample_live(&mut app, &mut gate, [false; 4], [false; 4], false);
        let mut press = [false; 4];
        collect_press(&key_event(Scancode::W, true, true), &mut press);
        sample_live(&mut app, &mut gate, [false; 4], press, false);
        assert_eq!(gate.take_axis(), 0., "repeat cannot rearm");
        let w = [true, false, false, false];
        sample_live(&mut app, &mut gate, w, w, false);
        let mut axes = Vec::new();
        app.tick_with_axis(0.026, || { let a = gate.take_axis(); axes.push(a); a });
        assert_eq!(axes, [-1., -1., -1.]);
        sample_live(&mut app, &mut gate, [false; 4], [false, true, false, false], false);
        assert_eq!(gate.take_axis(), 0., "held alias cannot queue a tail");
        sample_live(&mut app, &mut gate, [false; 4], [true; 4], false);
        assert_eq!(gate.take_axis(), 0., "opposite taps cancel");
        assert_eq!(gate.take_axis(), 0.);
        for transition in 0..3 {
            app.game.start();
            sample_live(&mut app, &mut gate, [false; 4], [false; 4], false);
            sample_live(&mut app, &mut gate, [false; 4], w, false);
            let entered_pause = match transition {
                0 => live_action(&mut app, &mut gate, true, LiveAction::Pause),
                1 => live_action(&mut app, &mut gate, true, LiveAction::Restart),
                _ => { lose_focus(&mut app, &mut gate); false }
            };
            sample_live(&mut app, &mut gate, [false; 4], [false; 4], entered_pause);
            assert_eq!(gate.take_axis(), 0., "transition {transition}");
        }
    }

}
