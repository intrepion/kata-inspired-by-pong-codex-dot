//! Bevy owns input and presentation; pong-core owns all gameplay.
use bevy::{prelude::*, window::PrimaryWindow};
use pong_core::{BALL, Game, H, PADDLE_H, PADDLE_W, State, W, WIN};

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
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(1, |d| d.as_nanos() as u64);
    App::new()
        .insert_resource(Match(Game::new(seed)))
        .insert_resource(Clock::default())
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
        .add_systems(Update, (input_and_simulation, present).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.082, 0.247, 0.216), Vec2::new(W, H)),
        Transform::from_xyz(0., 0., 0.),
    ));
    for y in (0..18).map(|i| -H / 2. + i as f32 * 28. + 10.) {
        commands.spawn((
            Sprite::from_color(Color::srgba(0.66, 0.90, 0.73, 0.25), Vec2::new(2., 12.)),
            Transform::from_xyz(0., y, 1.),
        ));
    }
    for (actor, color) in [
        (Actor::Player, Color::srgb(0.85, 1., 0.68)),
        (Actor::Cpu, Color::srgb(0.89, 0.95, 0.91)),
    ] {
        commands.spawn((
            actor,
            Sprite::from_color(color, Vec2::new(PADDLE_W, PADDLE_H)),
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
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut game: ResMut<Match>,
    mut clock: ResMut<Clock>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    if !window.focused {
        game.0.pause();
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
    let input = (keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS)) as i32
        - (keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW)) as i32;
    if game.0.state != State::Playing {
        clock.0 = 0.;
        return;
    }
    // Fixed simulation steps and a bounded catch-up window prevent focus/resize stalls from tunnelling.
    const DT: f32 = 1. / 120.;
    clock.0 += time.delta_secs().min(0.1);
    while clock.0 >= DT {
        game.0.step(DT, input as f32);
        clock.0 -= DT;
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
