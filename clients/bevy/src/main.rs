//! Bevy owns input and presentation; pong-core owns all gameplay.
use bevy::input::{ButtonState, keyboard::KeyboardInput};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::{prelude::*, window::PrimaryWindow};
use pong_core::{BALL, Game, H, PADDLE_H, PADDLE_W, State, W, WIN};
use pong_scenario::Demo;

fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
#[derive(Resource)]
struct Benchmark {
    enabled: bool,
    frames: Option<pong_benchmark::Frames>,
    done: bool,
    scenario: pong_scenario::Scenario,
}
#[derive(Resource)]
struct Replay {
    demo: Option<Demo>,
    pending: Option<(std::path::PathBuf, std::time::Instant)>,
}

#[derive(Resource, Default)]
struct MovementGate {
    suppressed: [bool; 4],
    pending: [bool; 4],
    was_playing: bool,
}
impl MovementGate {
    fn finish_frame(&mut self, state: State) {
        self.was_playing &= state == State::Playing;
        if !self.was_playing {
            self.pending = [false; 4];
        }
    }
}
fn restart_game(game: &mut Game, gate: &mut MovementGate) {
    game.start();
    gate.finish_frame(State::Ready);
}
fn active_movement(state: State, gate: &mut MovementGate, held: [bool; 4]) -> [bool; 4] {
    let playing = state == State::Playing;
    let suppress = !playing || !gate.was_playing;
    gate.was_playing = playing;
    for (blocked, down) in gate.suppressed.iter_mut().zip(held) {
        *blocked = down && (suppress || *blocked);
    }
    std::array::from_fn(|i| held[i] && !gate.suppressed[i])
}
fn axis_for(active: [bool; 4]) -> f32 {
    (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
}
#[cfg(test)]
fn movement_axis(state: State, gate: &mut MovementGate, held: [bool; 4]) -> f32 {
    axis_for(active_movement(state, gate, held))
}
#[derive(Resource)]
struct Match(Game);
#[derive(Resource, Default)]
struct Clock(f32);
#[derive(Component)]
enum Actor {
    Player,
    Cpu,
    Ball,
}
#[derive(Component)]
struct Score;
#[derive(Component)]
struct Status;

fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(1, |d| d.as_nanos() as u64);
    let demo = Demo::from_args();
    let seed = if demo.is_some() || benchmarking {
        pong_scenario::SEED
    } else {
        seed
    };
    // Prefer Direct3D 12 on Windows while honoring an explicit backend override.
    let plugins = DefaultPlugins;
    #[cfg(target_os = "windows")]
    let plugins = {
        use bevy::render::{
            settings::{Backends, WgpuSettings},
            RenderPlugin,
        };
        plugins.set(RenderPlugin {
            render_creation: WgpuSettings {
                backends: Some(Backends::from_env().unwrap_or(Backends::DX12)),
                ..default()
            }
            .into(),
            ..default()
        })
    };
    App::new()
        .insert_resource(Benchmark {
            enabled: benchmarking,
            frames: None,
            done: false,
            scenario: pong_scenario::Scenario::default(),
        })
        .insert_resource(Replay {
            demo,
            pending: None,
        })
        .insert_resource(Match(Game::new(seed)))
        .insert_resource(Clock::default())
        .insert_resource(MovementGate::default())
        .insert_resource(ClearColor(Color::srgb(0.035, 0.08, 0.075)))
        .add_plugins(plugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rallylight - Bevy / shared Rust core".into(),
                resolution: (1000, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Last, sample_benchmark)
        .add_systems(
            Update,
            (input_and_simulation, present, capture_demo).chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        bevy::render::view::Tonemapping::Linear,
        bevy::render::view::DebandDither::Disabled,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(W, H))),
        MeshMaterial2d(materials.add(Color::srgb(0.082, 0.247, 0.216))),
        Transform::from_xyz(0., 0., 0.),
    ));
    for y in (0..18).map(|i| -H / 2. + i as f32 * 28. + 10.) {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(2., 12.))),
            MeshMaterial2d(materials.add(Color::srgba(0.66, 0.90, 0.73, 0.25))),
            Transform::from_xyz(0., y, 1.),
        ));
    }
    for (actor, color) in [
        (Actor::Player, Color::srgb(0.85, 1., 0.68)),
        (Actor::Cpu, Color::srgb(0.89, 0.95, 0.91)),
    ] {
        commands.spawn((
            actor,
            Mesh2d(meshes.add(Rectangle::new(PADDLE_W, PADDLE_H))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0., 0., 2.),
        ));
    }
    commands.spawn((
        Actor::Ball,
        Mesh2d(meshes.add(Circle::new(BALL))),
        MeshMaterial2d(materials.add(Color::srgb(0.89, 0.95, 0.91))),
        Transform::from_xyz(0., 0., 2.),
    ));
    commands.spawn((
        Score,
        Text2d::new("YOU  0 : 0  CPU"),
        TextFont {
            font_size: FontSize::Px(36.),
            ..default()
        },
        TextColor(Color::srgb(0.85, 1., 0.68)),
        Transform::from_xyz(0., H / 2. + 46., 5.),
    ));
    commands.spawn((
        Status,
        Text2d::new("Enter to start"),
        TextFont {
            font_size: FontSize::Px(23.),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0., 75., 5.),
    ));
    commands.spawn((
        Text2d::new(
            "W/S or arrows: move   |   Enter: start/resume   |   Space: pause   |   R: restart",
        ),
        TextFont {
            font_size: FontSize::Px(17.),
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.8, 0.76)),
        Transform::from_xyz(0., -H / 2. - 40., 5.),
    ));
}

fn input_and_simulation(
    time: Res<Time>,
    mut replay: ResMut<Replay>,
    mut benchmark: ResMut<Benchmark>,
    keys: Res<ButtonInput<KeyCode>>,
    mut keyboard: MessageReader<KeyboardInput>,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut game: ResMut<Match>,
    mut clock: ResMut<Clock>,
    mut movement: ResMut<MovementGate>,
) {
    if replay.pending.is_some() {
        return;
    }
    if let Some(demo) = &mut replay.demo {
        demo.advance(&mut game.0, time.delta_secs());
        return;
    }
    if benchmark.enabled {
        clock.0 += time.delta_secs().min(0.1);
        while clock.0 >= pong_scenario::STEP {
            if benchmark.scenario.finished() {
                benchmark.scenario = pong_scenario::Scenario::default();
                game.0 = Game::new(pong_scenario::SEED);
            }
            benchmark.scenario.advance(&mut game.0);
            clock.0 -= pong_scenario::STEP;
        }
        return;
    }
    let Ok((window_entity, window)) = windows.single() else {
        keyboard.clear();
        movement.finish_frame(State::Ready);
        clock.0 = 0.;
        return;
    };
    if !window.focused {
        keyboard.clear();
        game.0.pause();
        movement.finish_frame(State::Paused);
        clock.0 = 0.;
        return;
    }
    let changed = apply_live_actions(
        &mut game.0,
        &mut movement,
        keys.just_pressed(KeyCode::KeyR),
        keys.just_pressed(KeyCode::Enter),
        keys.just_pressed(KeyCode::Space),
    );
    let bindings = [
        KeyCode::ArrowUp,
        KeyCode::KeyW,
        KeyCode::ArrowDown,
        KeyCode::KeyS,
    ];
    let capture = !changed && game.0.state == State::Playing && movement.was_playing;
    // Keep at most one pending press per binding until a physics tick.
    // Typed events let us ignore OS repeats even when Bevy's held snapshot is empty.
    for event in keyboard.read() {
        if !capture || event.window != window_entity {
            continue;
        }
        let Some(i) = bindings.iter().position(|key| *key == event.key_code) else {
            continue;
        };
        match event.state {
            ButtonState::Released => movement.suppressed[i] = false,
            ButtonState::Pressed if !event.repeat && keys.just_pressed(event.key_code) => {
                movement.pending[i] |= !movement.suppressed[i];
            }
            ButtonState::Pressed => {}
        }
    }
    let held = active_movement(
        game.0.state,
        &mut movement,
        bindings.map(|key| keys.pressed(key)),
    );
    advance_live_frame(
        &mut game.0,
        &mut clock.0,
        changed,
        time.delta_secs(),
        held,
        &mut movement.pending,
    );
    movement.finish_frame(game.0.state);
}

fn apply_live_actions(
    game: &mut Game,
    movement: &mut MovementGate,
    restart: bool,
    enter: bool,
    space: bool,
) -> bool {
    let before = game.state;
    if restart {
        restart_game(game, movement);
    }
    if enter {
        match game.state {
            State::Ready | State::Over => game.start(),
            State::Paused => game.toggle_pause(),
            State::Playing => {}
        }
    }
    if space {
        game.toggle_pause();
    }
    restart || before != game.state
}

fn advance_live_frame(
    game: &mut Game,
    clock: &mut f32,
    changed: bool,
    elapsed: f32,
    held: [bool; 4],
    pending: &mut [bool; 4],
) {
    if changed || game.state != State::Playing {
        // Bevy's update delta covers the time before this frame's input actions.
        *clock = 0.;
        *pending = [false; 4];
        return;
    }
    if !elapsed.is_finite() {
        return;
    }
    const DT: f32 = 1. / 120.;
    *clock += elapsed.clamp(0., 0.1);
    while *clock >= DT && game.state == State::Playing {
        // Consume buffered presses on the first actual tick, not the next render.
        let taps = std::mem::take(pending);
        game.step(DT, axis_for(std::array::from_fn(|i| held[i] || taps[i])));
        *clock -= DT;
    }
    if game.state != State::Playing {
        *clock = 0.;
    }
}

fn present(
    game: Res<Match>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut Projection>,
    mut actors: Query<(&Actor, &mut Transform)>,
    mut texts: Query<(&mut Text2d, Option<&Score>, Option<&Status>)>,
) {
    if let Ok(window) = windows.single() {
        for mut projection in &mut cameras {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                ortho.scale =
                    ((W + 60.) / window.width().max(1.)).max((H + 150.) / window.height().max(1.));
            }
        }
    }
    let g = &game.0;
    for (actor, mut t) in &mut actors {
        let (x, y) = match actor {
            Actor::Player => (38., g.player),
            Actor::Cpu => (W - 38., g.cpu),
            Actor::Ball => (g.ball.x, g.ball.y),
        };
        t.translation.x = x - W / 2.;
        t.translation.y = H / 2. - y;
    }
    for (mut text, score, status) in &mut texts {
        if score.is_some() {
            text.0 = format!("YOU  {} : {}  CPU", g.scores[0], g.scores[1]);
        }
        if status.is_some() {
            text.0 = match g.state {
                State::Ready => "First to 7 - Enter to start".into(),
                State::Paused => "Paused - Enter or Space to resume".into(),
                State::Over if g.scores[0] >= WIN => "You won! - Enter to play again".into(),
                State::Over => "Computer wins - Enter to play again".into(),
                State::Playing if g.serve > 0. => "Next serve".into(),
                State::Playing => String::new(),
            };
        }
    }
}

// Freeze the replay while asynchronous GPU readback writes this exact checkpoint.
fn capture_demo(
    mut commands: Commands,
    mut replay: ResMut<Replay>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Some((path, since)) = &replay.pending {
        let complete = std::fs::read(path)
            .is_ok_and(|b| b.ends_with(&[0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130]));
        assert!(
            since.elapsed().as_secs() < 30,
            "screenshot readback timed out: {}",
            path.display()
        );
        if !complete {
            return;
        }
        replay.pending = None;
    }
    if let Some(demo) = &mut replay.demo {
        if let Some(path) = demo.capture_path() {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.clone()));
            replay.pending = Some((path, std::time::Instant::now()));
        } else if demo.finished() {
            exit.write(AppExit::Success);
        }
    }
}

// Main-world frame boundary only; render-subapp/GPU work is not timed directly.
fn sample_benchmark(
    mut benchmark: ResMut<Benchmark>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut exit: MessageWriter<AppExit>,
) {
    if !benchmark.enabled || benchmark.done {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let (width, height) = (window.physical_width(), window.physical_height());
    if width == 0 || height == 0 {
        return;
    }
    let frames = benchmark
        .frames
        .get_or_insert_with(|| pong_benchmark::Frames::new(width, height));
    if frames.frame(width, height) {
        let report = frames.report();
        benchmark.done = true;
        println!("PONG_BENCHMARK {}", report);
        exit.write(AppExit::Success);
    }
}
#[cfg(test)]
mod benchmark_tests {
    use super::benchmark_args;
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
    fn live_actions_do_not_consume_pre_action_frame_time() {
        let mut game = Game::new(42);
        let mut movement = MovementGate::default();
        let mut clock = 0.004;
        for (restart, enter, space, state) in [
            (false, true, false, State::Playing),
            (false, false, true, State::Paused),
            (false, false, true, State::Playing),
            (true, false, false, State::Playing),
        ] {
            let changed = apply_live_actions(&mut game, &mut movement, restart, enter, space);
            assert!(changed);
            assert_eq!(game.state, state);
            let player = game.player;
            let serve = game.serve;
            advance_live_frame(
                &mut game,
                &mut clock,
                changed,
                0.1,
                [false, false, true, false],
                &mut movement.pending,
            );
            assert_eq!((game.player, game.serve, clock), (player, serve, 0.));
            if state == State::Playing {
                advance_live_frame(
                    &mut game,
                    &mut clock,
                    false,
                    0.004,
                    [false, false, true, false],
                    &mut movement.pending,
                );
                assert_eq!(game.player, player);
                advance_live_frame(
                    &mut game,
                    &mut clock,
                    false,
                    0.006,
                    [false, false, true, false],
                    &mut movement.pending,
                );
                assert!(game.player > player);
            }
        }
        // Enter during an active match is a no-op and must not discard time.
        assert!(!apply_live_actions(
            &mut game,
            &mut movement,
            false,
            true,
            false
        ));
    }

    #[test]
    fn first_playing_sample_and_focus_loss_suppress_keys_independently() {
        let mut gate = MovementGate::default();
        assert_eq!(
            movement_axis(State::Playing, &mut gate, [true, false, false, false]),
            0.
        );
        movement_axis(State::Playing, &mut gate, [false; 4]);
        assert_eq!(
            movement_axis(State::Playing, &mut gate, [true, false, false, false]),
            -1.
        );
        gate.was_playing = false;
        assert_eq!(
            movement_axis(State::Playing, &mut gate, [true, false, false, false]),
            0.
        );
        assert_eq!(
            movement_axis(State::Playing, &mut gate, [true, true, false, false]),
            -1.
        );
    }
    #[test]
    fn winning_score_keeps_held_movement_suppressed_on_restart() {
        for i in 0..4 {
            for (scores, x, vx) in [([6, 2], W + BALL - 1., 350.), ([2, 6], -BALL + 1., -350.)] {
                let mut game = Game::new(42);
                game.start();
                let mut gate = MovementGate::default();
                let mut held = [false; 4];
                movement_axis(game.state, &mut gate, held);
                held[i] = true;
                assert_ne!(movement_axis(game.state, &mut gate, held), 0.);
                game.scores = scores;
                game.serve = 0.;
                game.ball.x = x;
                game.ball.y = H / 2.;
                game.ball.vx = vx;
                game.ball.vy = 0.;
                game.step(0.02, 0.);
                assert_eq!(game.state, State::Over);
                gate.finish_frame(game.state);
                game.start();
                assert_eq!(movement_axis(game.state, &mut gate, held), 0.);
                held[i] = false;
                movement_axis(game.state, &mut gate, held);
                held[i] = true;
                assert_ne!(movement_axis(game.state, &mut gate, held), 0.);
            }
        }
    }
    #[test]
    fn nonplaying_held_keys_do_not_move_after_resume() {
        for state in [State::Ready, State::Paused, State::Over] {
            for i in 0..4 {
                let mut gate = MovementGate::default();
                let mut held = [false; 4];
                held[i] = true;
                assert_eq!(movement_axis(state, &mut gate, held), 0., "{state:?}");
                assert_eq!(movement_axis(State::Playing, &mut gate, held), 0.);
                held[i] = false;
                movement_axis(State::Playing, &mut gate, held);
                held[i] = true;
                assert_ne!(movement_axis(State::Playing, &mut gate, held), 0.);
            }
        }
    }
}

#[cfg(test)]
mod active_restart_tests {
    use super::*;
    #[test]
    fn held_key_must_not_survive_active_restart() {
        for i in 0..4 {
            let mut game = Game::new(42);
            let mut gate = MovementGate::default();
            game.start();
            movement_axis(game.state, &mut gate, [false; 4]);
            let mut held = [false; 4];
            held[i] = true;
            assert_ne!(movement_axis(game.state, &mut gate, held), 0.);
            restart_game(&mut game, &mut gate);
            assert_eq!(movement_axis(game.state, &mut gate, held), 0.);
            held[i] = false;
            movement_axis(game.state, &mut gate, held);
            held[i] = true;
            assert_ne!(movement_axis(game.state, &mut gate, held), 0.);
        }
    }
}

#[cfg(test)]
#[path = "presentation_tests.rs"]
mod presentation_tests;

#[cfg(test)]
mod quick_tap_audit {
    use super::*;
    use bevy::input::{InputPlugin, keyboard::Key};
    use std::time::Duration;

    fn world() -> (App, Entity) {
        let mut game = Game::new(42);
        game.start();
        let mut app = App::new();
        app.add_plugins(InputPlugin)
            .insert_resource(Time::<()>::default())
            .insert_resource(Replay {
                demo: None,
                pending: None,
            })
            .insert_resource(Benchmark {
                enabled: false,
                frames: None,
                done: false,
                scenario: Default::default(),
            })
            .insert_resource(Match(game))
            .insert_resource(Clock::default())
            .insert_resource(MovementGate::default())
            .add_systems(Update, input_and_simulation);
        let window = app
            .world_mut()
            .spawn((
                PrimaryWindow,
                Window {
                    focused: true,
                    ..Default::default()
                },
            ))
            .id();
        app.update();
        (app, window)
    }
    fn event(app: &mut App, window: Entity, key_code: KeyCode, state: ButtonState, repeat: bool) {
        let logical_key = match key_code {
            KeyCode::ArrowUp => Key::ArrowUp,
            KeyCode::ArrowDown => Key::ArrowDown,
            KeyCode::KeyW => Key::Character("w".into()),
            KeyCode::KeyR => Key::Character("r".into()),
            KeyCode::Space => Key::Space,
            _ => unreachable!(),
        };
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key,
            state,
            text: None,
            repeat,
            window,
        });
    }

    fn key(app: &mut App, window: Entity, state: ButtonState) {
        event(app, window, KeyCode::ArrowUp, state, false);
    }

    fn tap(app: &mut App, window: Entity, key: KeyCode) {
        for state in [ButtonState::Pressed, ButtonState::Released] {
            event(app, window, key, state, false);
        }
    }
    fn next(app: &mut App, micros: u64) -> f32 {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_micros(micros));
        app.update();
        app.world().resource::<Match>().0.player
    }
    #[test]
    fn quick_taps_survive_zero_tick_and_only_move_once() {
        for span_frames in [false, true] {
            let (mut app, window) = world();
            let start = app.world().resource::<Match>().0.player;
            key(&mut app, window, ButtonState::Pressed);
            if span_frames {
                assert_eq!(next(&mut app, 4_000), start);
            }
            key(&mut app, window, ButtonState::Released);
            if !span_frames {
                assert_eq!(next(&mut app, 4_000), start);
                let keys = app.world().resource::<ButtonInput<KeyCode>>();
                assert!(keys.just_pressed(KeyCode::ArrowUp));
                assert!(!keys.pressed(KeyCode::ArrowUp));
            }
            let observed = next(&mut app, 22_000);
            let (mut held, window) = world();
            assert_eq!(next(&mut held, 4_000), start);
            key(&mut held, window, ButtonState::Pressed);
            let held_pos = next(&mut held, 22_000);
            let (mut single, window) = world();
            key(&mut single, window, ButtonState::Pressed);
            let intended = next(&mut single, 9_000);
            eprintln!(
                "span_frames={span_frames}; start={start}; quick_tap={observed}; one_step={intended}; three_held_steps={held_pos}"
            );
            assert_eq!(observed, intended);
            assert_eq!(next(&mut app, 25_000), intended);
            assert!(held_pos < intended && intended < start);
            assert!((start - held_pos - 3.0 * (start - intended)).abs() < 0.0001);
        }
    }

    #[test]
    fn aliases_coalesce_and_opposites_cancel_at_first_tick() {
        let (mut app, window) = world();
        let start = next(&mut app, 0);
        tap(&mut app, window, KeyCode::ArrowUp);
        tap(&mut app, window, KeyCode::KeyW);
        assert_eq!(next(&mut app, 4_000), start);
        let moved = next(&mut app, 22_000);
        let (mut single, window_single) = world();
        key(&mut single, window_single, ButtonState::Pressed);
        assert_eq!(moved, next(&mut single, 9_000));
        let (mut app, window) = world();
        tap(&mut app, window, KeyCode::ArrowUp);
        assert_eq!(next(&mut app, 4_000), start);
        tap(&mut app, window, KeyCode::ArrowDown);
        assert_eq!(next(&mut app, 22_000), start);
        assert_eq!(next(&mut app, 25_000), start);
    }

    #[test]
    fn repeat_is_not_a_new_tap_but_a_held_key_keeps_moving() {
        let (mut app, window) = world();
        let start = next(&mut app, 0);
        event(
            &mut app,
            window,
            KeyCode::ArrowUp,
            ButtonState::Pressed,
            true,
        );
        key(&mut app, window, ButtonState::Released);
        assert_eq!(next(&mut app, 9_000), start);
        key(&mut app, window, ButtonState::Pressed);
        assert!(next(&mut app, 9_000) < start);
        event(
            &mut app,
            window,
            KeyCode::ArrowUp,
            ButtonState::Pressed,
            true,
        );
        key(&mut app, window, ButtonState::Released);
        let current = app.world().resource::<Match>().0.player;
        assert_eq!(next(&mut app, 25_000), current);
    }

    #[test]
    fn pause_restart_and_focus_loss_drop_unconsumed_taps() {
        for transition in ["pause", "restart", "focus"] {
            let (mut app, window) = world();
            let start = next(&mut app, 0);
            tap(&mut app, window, KeyCode::ArrowUp);
            assert_eq!(next(&mut app, 4_000), start);
            match transition {
                "pause" => tap(&mut app, window, KeyCode::Space),
                "restart" => tap(&mut app, window, KeyCode::KeyR),
                "focus" => app.world_mut().get_mut::<Window>(window).unwrap().focused = false,
                _ => unreachable!(),
            }
            assert_eq!(next(&mut app, 22_000), start);
            if transition == "focus" {
                app.world_mut().get_mut::<Window>(window).unwrap().focused = true;
            }
            if transition != "restart" {
                tap(&mut app, window, KeyCode::Space);
                assert_eq!(next(&mut app, 22_000), start);
            }
            assert_eq!(next(&mut app, 25_000), start);
            tap(&mut app, window, KeyCode::ArrowUp);
            assert!(next(&mut app, 22_000) < start);
        }
    }
}
