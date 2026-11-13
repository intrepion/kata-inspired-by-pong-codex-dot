//! Bevy owns input and presentation; pong-core owns all gameplay.
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
    was_playing: bool,
}
impl MovementGate {
    fn finish_frame(&mut self, state: State) {
        self.was_playing &= state == State::Playing;
    }
}
fn movement_axis(state: State, gate: &mut MovementGate, held: [bool; 4]) -> f32 {
    let playing = state == State::Playing;
    let suppress = !playing || !gate.was_playing;
    gate.was_playing = playing;
    for (blocked, down) in gate.suppressed.iter_mut().zip(held) {
        *blocked = down && (suppress || *blocked);
    }
    let active = std::array::from_fn::<_, 4, _>(|i| held[i] && !gate.suppressed[i]);
    (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
    commands.spawn(Camera2d);
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
    windows: Query<&Window, With<PrimaryWindow>>,
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
    let Ok(window) = windows.single() else {
        return;
    };
    if !window.focused {
        game.0.pause();
        movement.was_playing = false;
        clock.0 = 0.;
        return;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        game.0.start();
        clock.0 = 0.;
    }
    if keys.just_pressed(KeyCode::Enter) {
        match game.0.state {
            State::Ready | State::Over => game.0.start(),
            State::Paused => game.0.toggle_pause(),
            State::Playing => {}
        }
        clock.0 = 0.;
    }
    if keys.just_pressed(KeyCode::Space) {
        game.0.toggle_pause();
        clock.0 = 0.;
    }
    let input = movement_axis(
        game.0.state,
        &mut movement,
        [
            keys.pressed(KeyCode::ArrowUp),
            keys.pressed(KeyCode::KeyW),
            keys.pressed(KeyCode::ArrowDown),
            keys.pressed(KeyCode::KeyS),
        ],
    );
    if game.0.state != State::Playing {
        clock.0 = 0.;
        return;
    }
    // Fixed simulation steps and a bounded catch-up window prevent focus/resize stalls from tunnelling.
    const DT: f32 = 1. / 120.;
    clock.0 += time.delta_secs().min(0.1);
    while clock.0 >= DT {
        game.0.step(DT, input);
        clock.0 -= DT;
    }
    movement.finish_frame(game.0.state);
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
