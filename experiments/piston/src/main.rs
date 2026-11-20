use piston_window::graphics::{clear, rectangle, Transformed};

use piston_window::*;
use pong_core::*;
mod clock;
use clock::UpdateClock;
use std::time::Instant;
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
fn window_title(game: &Game) -> String {
    let status: String = match game.state {
        State::Over if game.scores[0] > game.scores[1] => "YOU WIN - Enter to restart".into(),
        State::Over if game.scores[1] > game.scores[0] => "CPU WINS - Enter to restart".into(),
        State::Over => "MATCH OVER - Enter to restart".into(),
        state => format!("{state:?}"),
    };
    format!("Pong Piston | {status} | YOU {} : {} CPU | Enter: play | W/S/arrows | Space: pause | R: restart", game.scores[0], game.scores[1])
}
fn winner_label(game: &Game) -> Option<&'static str> {
    if game.state != State::Over {
        return None;
    }
    if game.scores[0] > game.scores[1] {
        Some("YOU WIN")
    } else if game.scores[1] > game.scores[0] {
        Some("CPU WINS")
    } else {
        None
    }
}
fn result_glyph(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [14, 17, 17, 31, 17, 17, 17],
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'E' => [31, 16, 16, 30, 16, 16, 31],
        b'I' => [14, 4, 4, 4, 4, 4, 14],
        b'M' => [17, 27, 21, 21, 17, 17, 17],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'R' => [30, 17, 17, 30, 20, 18, 17],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'T' => [31, 4, 4, 4, 4, 4, 4],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        _ => [0; 7],
    }
}
fn text_cells(message: &str, left: f64, top: f64, cell: f64) -> Vec<[f64; 4]> {
    let mut result = Vec::new();
    for (i, ch) in message.bytes().enumerate() {
        for (row, bits) in result_glyph(ch).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    result.push([left + i as f64 * 6. * cell + col as f64 * cell, top + row as f64 * cell, cell, cell]);
                }
            }
        }
    }
    result
}
fn small_text_cell(scale: f64) -> f64 {
    if scale.is_finite() && scale > 0.0 { (1.5 / scale).clamp(2.0, 8.0) } else { 2.0 }
}
fn score_owner_cells(scale: f64) -> Vec<[f64; 4]> {
    let cell = small_text_cell(scale);
    text_cells("YOU", 348. - 9. * cell, 45., cell).into_iter()
        .chain(text_cells("CPU", 518. - 9. * cell, 45., cell))
        .collect()
}
fn court_hint(state: State) -> Option<&'static str> {
    match state {
        State::Ready => Some("ENTER TO START"),
        State::Paused => Some("SPACE TO RESUME"),
        State::Over => Some("ENTER TO RESTART"),
        State::Playing => None,
    }
}
fn hint_backing(state: State, scale: f64) -> Option<[f64; 4]> {
    court_hint(state).map(|text| {
        let cell = small_text_cell(scale);
        let width = text.len() as f64 * 6. * cell;
        [(W as f64 - width) / 2. - 5., 282., width + 10., 7. * cell + 10.]
    })
}
fn hint_cells(state: State, scale: f64) -> Vec<[f64; 4]> {
    court_hint(state).map_or_else(Vec::new, |text| {
        let cell = small_text_cell(scale);
        text_cells(text, (W as f64 - text.len() as f64 * 6. * cell) / 2., 287., cell)
    })
}
fn winner_cells(game: &Game) -> Vec<[f64; 4]> {
    let Some(message) = winner_label(game) else {
        return Vec::new();
    };
    let left = (W as f64 - message.len() as f64 * 36.) / 2.;
    let mut cells = Vec::new();
    for (i, ch) in message.bytes().enumerate() {
        for (y, row) in result_glyph(ch).iter().enumerate() {
            for col in 0..5 {
                if row & (1 << (4 - col)) != 0 {
                    cells.push([
                        left + i as f64 * 36. + col as f64 * 6.,
                        225. + y as f64 * 6.,
                        6.,
                        6.,
                    ]);
                }
            }
        }
    }
    cells
}
fn state_indicator_cells(state: State) -> Vec<[f64; 4]> {
    match state {
        State::Ready => [8., 16., 24., 32., 24., 16., 8.]
            .into_iter()
            .enumerate()
            .map(|(i, width)| [430., 222. + i as f64 * 8., width, 8.])
            .collect(),
        State::Paused => vec![[420., 225., 12., 50.], [468., 225., 12., 50.]],
        State::Playing | State::Over => Vec::new(),
    }
}
fn indicator_backing(game: &Game) -> Option<[f64; 4]> {
    match game.state {
        State::Ready | State::Paused => Some([412., 214., 76., 66.]),
        State::Over => winner_label(game).map(|message| {
            let width = message.len() as f64 * 36.;
            [(W as f64 - width) / 2. - 8., 217., width + 16., 58.]
        }),
        State::Playing => None,
    }
}

// Returns whether the event resets the fixed-step clock.
fn key_press(
    game: &mut Game,
    held: &mut [bool; 4],
    pressed: &mut std::collections::HashSet<Key>,
    key: Key,
) -> bool {
    if !pressed.insert(key) {
        return false;
    }
    for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
        if key == k && game.state == State::Playing {
            held[i] = true;
        }
    }
    let previous = game.state;
    match key {
        Key::Return | Key::NumPadEnter => {
            if game.state == State::Paused {
                game.toggle_pause();
            } else if game.state != State::Playing {
                game.start();
            }
        }
        Key::R => {
            *held = [false; 4];
            game.start();
        }
        Key::Space => game.toggle_pause(),
        Key::Escape => game.pause(),
        _ => {}
    }
    if (previous == State::Playing && game.state == State::Paused)
        || (previous == State::Over && game.state == State::Playing)
    {
        *held = [false; 4];
    }
    matches!(key, Key::Return | Key::NumPadEnter | Key::R | Key::Space | Key::Escape)
}
fn focused_key_press(
    game: &mut Game,
    held: &mut [bool; 4],
    pressed: &mut std::collections::HashSet<Key>,
    focused: bool,
    key: Key,
) -> bool {
    focused && key_press(game, held, pressed, key)
}
fn key_release(held: &mut [bool; 4], pressed: &mut std::collections::HashSet<Key>, key: Key) {
    pressed.remove(&key);
    for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
        if key == k {
            held[i] = false;
        }
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;
    #[test]
    fn score_owner_glyph_cells_cover_a_pixel_at_small_surface() {
        let scale = 320.0 / 900.0;
        let cells = score_owner_cells(scale);
        assert!(!cells.is_empty());
        assert!(cells.iter().all(|cell| cell[2] * scale >= 1.0 && cell[3] * scale >= 1.0));
    }
    #[test]
    fn viewport_bumps_small_text_and_keeps_backing_clear_of_indicator() {
        let game = Game::new(42);
        for size in [[225., 125.], [900., 500.], [1800., 1000.]] {
            let scale = (size[0] / W as f64).min(size[1] / H as f64);
            let cell = small_text_cell(scale);
            assert!(cell * scale >= 1.5);
            assert_eq!(cell, if scale < 1. { 6. } else { 2. });
            let owners = score_owner_cells(scale);
            assert!(owners.iter().all(|[x, y, w, h]| *x >= 0. && *y >= 0.
                && x+w <= W as f64 && y+h <= H as f64));
            let indicator = indicator_backing(&game).unwrap();
            for state in [State::Ready, State::Paused, State::Over] {
                let [left, top, width, height] = hint_backing(state, scale).unwrap();
                assert!(left >= 0. && left+width <= W as f64 && top+height <= H as f64);
                assert!(top > indicator[1]+indicator[3]);
                for [x, y, w, h] in hint_cells(state, scale) {
                    assert!(x >= left && x+w <= left+width && y >= top && y+h <= top+height);
                }
            }
        }
    }
    #[test]
    fn court_labels_identify_scores_and_actions_without_title() {
        assert_eq!(court_hint(State::Ready), Some("ENTER TO START"));
        assert_eq!(court_hint(State::Paused), Some("SPACE TO RESUME"));
        assert_eq!(court_hint(State::Over), Some("ENTER TO RESTART"));
        assert_eq!(court_hint(State::Playing), None);
        for text in ["YOU", "CPU", "ENTER TO START", "SPACE TO RESUME", "ENTER TO RESTART"] {
            for ch in text.bytes().filter(|ch| *ch != b' ') {
                assert!(result_glyph(ch).iter().any(|r| *r != 0), "missing {}", ch as char);
            }
        }
        assert!(!score_owner_cells(1.0).is_empty());
        assert!(hint_cells(State::Playing, 1.0).is_empty());
        for state in [State::Ready, State::Paused, State::Over] {
            let panel = hint_backing(state, 1.0).unwrap();
            let cells = hint_cells(state, 1.0);
            assert!(!cells.is_empty());
            for [x, y, w, h] in cells {
                assert!(x >= panel[0] && y >= panel[1] && x + w <= panel[0] + panel[2] && y + h <= panel[1] + panel[3]);
            }
        }
    }
    #[test]
    fn keypad_enter_matches_return_without_repeats_or_unfocused_actions() {
        let mut game = Game::new(42);
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        assert!(!focused_key_press(&mut game, &mut held, &mut pressed, false, Key::NumPadEnter));
        assert_eq!(game.state, State::Ready);
        assert!(focused_key_press(&mut game, &mut held, &mut pressed, true, Key::NumPadEnter));
        assert_eq!(game.state, State::Playing);
        game.scores = [2, 3]; game.pause();
        assert!(!key_press(&mut game, &mut held, &mut pressed, Key::NumPadEnter));
        assert_eq!(game.state, State::Paused);
        key_release(&mut held, &mut pressed, Key::NumPadEnter);
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::NumPadEnter));
        assert_eq!(game.state, State::Playing); assert_eq!(game.scores, [2, 3]);
        key_release(&mut held, &mut pressed, Key::NumPadEnter);
        game.state = State::Over; held[0] = true;
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::NumPadEnter));
        assert_eq!(game.state, State::Playing); assert_eq!(game.scores, [0, 0]); assert_eq!(held, [false; 4]);
    }
    #[test]
    fn indicator_backing_contains_each_visible_cell() {
        let mut game = Game::new(42);
        for (state, scores) in [(State::Ready, [0, 0]), (State::Paused, [0, 0]), (State::Over, [7, 2]), (State::Over, [2, 7])] {
            game.state = state; game.scores = scores;
            let panel = indicator_backing(&game).unwrap();
            let cells: Vec<_> = state_indicator_cells(state).into_iter().chain(winner_cells(&game)).collect();
            assert!(!cells.is_empty());
            for [x, y, w, h] in cells {
                assert!(x >= panel[0] && y >= panel[1] && x+w <= panel[0]+panel[2] && y+h <= panel[1]+panel[3]);
            }
        }
        game.state = State::Playing;
        assert_eq!(indicator_backing(&game), None);
    }
    #[test]
    fn ready_play_symbol_and_pause_bars_match_state() {
        let mut game = Game::new(42);
        let ready = state_indicator_cells(game.state);
        assert_eq!(ready.len(), 7);
        assert!(ready.contains(&[430., 246., 32., 8.]));
        game.start();
        assert!(state_indicator_cells(game.state).is_empty());
        game.pause();
        assert_eq!(
            state_indicator_cells(game.state),
            vec![[420., 225., 12., 50.], [468., 225., 12., 50.]]
        );
        game.state = State::Over;
        assert!(state_indicator_cells(game.state).is_empty());
    }
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut g = Game::new(42);
        g.start();
        g.scores = [3, 2];
        g.pause();
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        focused_key_press(&mut g, &mut held, &mut pressed, false, Key::Return);
        assert_eq!(g.state, State::Paused);
        focused_key_press(&mut g, &mut held, &mut pressed, false, Key::R);
        assert_eq!(g.state, State::Paused);
        assert_eq!(g.scores, [3, 2]);
        focused_key_press(&mut g, &mut held, &mut pressed, true, Key::Return);
        assert_eq!(g.state, State::Playing);
    }
    #[test]
    fn title_names_winner_after_scoring_and_clears_on_restart() {
        let mut game = Game::new(42);
        assert!(window_title(&game).contains("Ready"));
        for (scores, x, vx, message) in [
            ([6, 3], W + BALL - 1., 350., "YOU WIN - Enter to restart"),
            ([2, 6], -BALL + 1., -350., "CPU WINS - Enter to restart"),
        ] {
            game.start();
            game.scores = scores;
            game.serve = 0.;
            game.ball.x = x;
            game.ball.y = H / 2.;
            game.ball.vx = vx;
            game.ball.vy = 0.;
            game.step(0.02, 0.);
            assert_eq!(game.state, State::Over);
            assert!(window_title(&game).contains(message));
            assert_eq!(
                winner_label(&game),
                Some(if scores[0] == 6 {
                    "YOU WIN"
                } else {
                    "CPU WINS"
                })
            );
            assert!(!winner_cells(&game).is_empty());
            game.start();
            assert!(winner_cells(&game).is_empty());
            assert!(!window_title(&game).contains(message));
            assert!(window_title(&game).contains("Playing"));
        }
    }
    #[test]
    fn active_restart_clears_held_movement() {
        let mut g = Game::new(42);
        g.start();
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        key_press(&mut g, &mut held, &mut pressed, Key::R);
        assert_eq!(held, [false; 4]);
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        assert_eq!(held, [false; 4]);
        key_release(&mut held, &mut pressed, Key::W);
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        assert!(held[0]);
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        for key in [Key::W, Key::Up, Key::S, Key::Down] {
            for restart in [Key::Return, Key::NumPadEnter, Key::R] {
                let mut game = Game::new(42);
                game.start();
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, key);
                game.scores = [6, 2];
                game.serve = 0.;
                game.ball = Ball {
                    x: W + BALL - 1.,
                    y: H / 2.,
                    vx: 350.,
                    vy: 0.,
                };
                game.step(0.02, 0.);
                assert_eq!(game.state, State::Over);
                key_press(&mut game, &mut held, &mut pressed, restart);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{key:?} {restart:?}");
                key_press(&mut game, &mut held, &mut pressed, key);
                assert_eq!(held, [false; 4]);
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
            }
        }
    }

    #[test]
    fn nonplaying_presses_do_not_rearm_after_start() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [Key::W, Key::Up, Key::S, Key::Down] {
                let mut game = Game::new(42);
                game.state = state;
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{state:?} {key:?}");
                key_press(&mut game, &mut held, &mut pressed, key);
                assert_eq!(held, [false; 4]);
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
            }
        }
    }

    #[test]
    fn actions_and_aliases_preserve_independent_key_behavior() {
        let mut game = Game::new(42);
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::Return));
        assert_eq!(game.state, State::Playing);
        key_release(&mut held, &mut pressed, Key::Return);
        key_press(&mut game, &mut held, &mut pressed, Key::W);
        key_press(&mut game, &mut held, &mut pressed, Key::Up);
        key_release(&mut held, &mut pressed, Key::W);
        assert!(held[1]);
        key_press(&mut game, &mut held, &mut pressed, Key::Space);
        assert_eq!(game.state, State::Paused);
        assert!(!key_press(&mut game, &mut held, &mut pressed, Key::Space));
        assert_eq!(game.state, State::Paused);
        key_release(&mut held, &mut pressed, Key::Space);
        key_press(&mut game, &mut held, &mut pressed, Key::Space);
        assert_eq!(game.state, State::Playing);
        key_press(&mut game, &mut held, &mut pressed, Key::Up);
        assert!(!held[1]);
        key_press(&mut game, &mut held, &mut pressed, Key::W);
        assert!(held[0]);
        game.scores = [2, 4];
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::R));
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn explicit_pause_clears_movement_until_release_and_fresh_press() {
        for pause in [Key::Space, Key::Escape] {
            for key in [Key::W, Key::Up, Key::S, Key::Down] {
                let mut game = Game::new(42);
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                key_release(&mut held, &mut pressed, Key::Return);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
                key_press(&mut game, &mut held, &mut pressed, pause);
                assert_eq!(game.state, State::Paused);
                assert!(!held.iter().any(|v| *v));
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                assert_eq!(game.state, State::Playing);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(!held.iter().any(|v| *v), "repeat restored movement");
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v), "fresh press should move");
            }
        }
    }
}
#[cfg(test)]
mod court_clip_tests {
    use super::*;
    use piston_window::graphics::{self, DrawState, Ellipse, Graphics, ImageSize};

    struct Texture;
    impl ImageSize for Texture { fn get_size(&self) -> (u32, u32) { (1, 1) } }
    #[derive(Default)]
    struct Recording { ellipses: Vec<(Option<[u32; 4]>, [f64; 4])>, rectangles: Vec<Option<[u32; 4]>> }
    impl Graphics for Recording {
        type Texture = Texture;
        fn clear_color(&mut self, _: [f32; 4]) {}
        fn clear_stencil(&mut self, _: u8) {}
        fn tri_list<F>(&mut self, d: &DrawState, _: &[f32; 4], _f: F) where F: FnMut(&mut dyn FnMut(&[[f32; 2]])) { self.rectangles.push(d.scissor); }
        fn tri_list_c<F>(&mut self, _: &DrawState, _: F) where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 4]])) {}
        fn tri_list_uv<F>(&mut self, _: &DrawState, _: &[f32; 4], _: &Texture, _: F) where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]])) {}
        fn tri_list_uv_c<F>(&mut self, _: &DrawState, _: &Texture, _: F) where F: FnMut(&mut dyn FnMut(&[[f32; 2]], &[[f32; 2]], &[[f32; 4]])) {}
        fn ellipse<R: Into<graphics::types::Rectangle>>(&mut self, _: &Ellipse, rect: R, d: &DrawState, _: graphics::math::Matrix2d) { self.ellipses.push((d.scissor, rect.into())); }
    }

    #[test]
    fn framebuffer_scissor_handles_hidpi_fractional_and_invalid_sizes() {
        assert_eq!(court_scissor([900., 500.], [900, 500]), Some([0, 0, 900, 500]));
        assert_eq!(court_scissor([1000., 500.], [2000, 1000]), Some([100, 0, 1800, 1000]));
        assert_eq!(court_scissor([900., 600.], [1800, 1200]), Some([0, 100, 1800, 1000]));
        assert_eq!(court_scissor([1000., 500.], [1600, 900]), Some([80, 0, 1440, 900]));
        for size in [[0., 500.], [-1., 500.], [f64::NAN, 500.], [900., f64::INFINITY]] { assert_eq!(court_scissor(size, [1000, 500]), None); }
        assert_eq!(court_scissor([900., 500.], [0, 500]), None);
        for (w, h) in [(1001., 501.), (320., 731.), (1921., 1081.)] {
            for fb in [[1502, 752], [503, 281], [2880, 1800]] {
                let [x,y,cw,ch] = court_scissor([w,h],fb).unwrap();
                let scale = (w/900_f64).min(h/500_f64);
                let left=(w-900.*scale)*0.5*fb[0] as f64/w;
                let right=(w+900.*scale)*0.5*fb[0] as f64/w;
                let top=(h-500.*scale)*0.5*fb[1] as f64/h;
                let bottom=(h+500.*scale)*0.5*fb[1] as f64/h;
                assert!(x as f64+1e-9>=left && (x+cw) as f64<=right+1e-9);
                assert!(y as f64+1e-9>=top && (y+ch) as f64<=bottom+1e-9);
                assert!(x+cw<=fb[0] && y+ch<=fb[1]);
            }
        }
    }

    #[test]
    fn ball_clips_at_all_four_edges_using_actual_framebuffer() {
        let vp=graphics::Viewport { rect:[0,0,2000,1000], draw_size:[2000,1000], window_size:[1000.,500.] };
        for (x,y) in [(-BALL/2.,250.),(W+BALL/2.,250.),(450.,-BALL/2.),(450.,H+BALL/2.)] {
            let mut game=Game::new(42);game.ball.x=x;game.ball.y=y;
            let mut recording=Recording::default();
            draw_scene(graphics::Context::new_viewport(vp),&mut recording,&game,[1600,900]);
            assert_eq!(recording.ellipses,vec![(Some([80,0,1440,900]),[(x-BALL) as f64,(y-BALL) as f64,(BALL*2.) as f64,(BALL*2.) as f64])]);
            assert!(!recording.rectangles.is_empty());
            assert!(recording.rectangles.iter().all(Option::is_none));
        }
    }

    #[test]
    fn unsupported_offset_viewport_does_not_draw_unclipped_ball() {
        let vp=graphics::Viewport { rect:[1,0,1999,1000], draw_size:[2000,1000], window_size:[1000.,500.] };
        let mut recording=Recording::default();
        draw_scene(graphics::Context::new_viewport(vp),&mut recording,&Game::new(42),[2000,1000]);
        assert!(recording.ellipses.is_empty());
    }
}

#[derive(Default)]
struct LiveInput {
    held: [bool; 4],
    pending: [bool; 2],
    pressed: std::collections::HashSet<Key>,
}

impl LiveInput {
    fn event(&mut self, game: &mut Game, focused: &mut bool, event: &Event,
             clock: &mut UpdateClock, now: Instant) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let before = game.state;
            let held_before = self.directions();
            if focused_key_press(game, &mut self.held, &mut self.pressed, *focused, key) {
                clock.reset(now);
                if key == Key::R || game.state != before {
                    self.pending = [false; 2];
                }
            }
            // Fold fresh direction presses into one pulse each until physics samples them.
            let held_after = self.directions();
            for direction in 0..2 {
                self.pending[direction] |= !held_before[direction] && held_after[direction];
            }
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            key_release(&mut self.held, &mut self.pressed, key);
        }
        if let Some(now_focused) = event.focus_args() {
            *focused = now_focused;
            if !*focused {
                game.pause();
                *self = Self::default();
                clock.reset(now);
            }
        }
    }

    fn directions(&self) -> [bool; 2] {
        [self.held[0] || self.held[1], self.held[2] || self.held[3]]
    }

    fn step_axis(&mut self) -> f32 {
        let [up, down] = self.directions();
        let [tap_up, tap_down] = std::mem::take(&mut self.pending);
        (down || tap_down) as u8 as f32 - (up || tap_up) as u8 as f32
    }
}

#[cfg(test)]
mod quick_tap_tests {
    use super::*;

    fn key_event(key: Key, state: ButtonState) -> Event {
        Event::Input(Input::Button(ButtonArgs {
            state, button: Button::Keyboard(key), scancode: None,
        }), None)
    }

    #[test]
    fn quick_tap_moves_on_first_physics_step_after_zero_step_update() {
        let time = Instant::now();
        let mut clock = UpdateClock::new(time);
        let mut game = Game::new(42);
        let mut input = LiveInput::default();
        let mut focused = true;
        game.start();
        for state in [ButtonState::Press, ButtonState::Release] {
            input.event(&mut game, &mut focused, &key_event(Key::W, state), &mut clock, time);
        }
        assert_eq!(clock.steps(0.004), 0);
        let sampled: Vec<_> = (0..clock.steps(0.022)).map(|_| input.step_axis()).collect();
        assert_eq!(sampled, [-1., 0., 0.]);
    }

    #[test]
    fn held_input_moves_each_step_and_aliases_do_not_queue_a_tail() {
        let now = Instant::now();
        let mut clock = UpdateClock::new(now);
        let mut game = Game::new(42);
        let mut input = LiveInput::default();
        let mut focused = true;
        game.start();
        for (key, state) in [(Key::W, ButtonState::Press), (Key::W, ButtonState::Press)] {
            input.event(&mut game, &mut focused, &key_event(key, state), &mut clock, now);
        }
        let sampled: Vec<_> = (0..clock.steps(0.026)).map(|_| input.step_axis()).collect();
        assert_eq!(sampled, [-1., -1., -1.]);
        for (key, state) in [
            (Key::Up, ButtonState::Press),
            (Key::W, ButtonState::Release),
            (Key::Up, ButtonState::Release),
        ] {
            input.event(&mut game, &mut focused, &key_event(key, state), &mut clock, now);
        }
        assert_eq!(input.step_axis(), 0.);
    }

    #[test]
    fn pause_restart_and_focus_loss_discard_pending_pulses() {
        for action in [
            key_event(Key::Space, ButtonState::Press),
            key_event(Key::R, ButtonState::Press),
            Event::Input(Input::Focus(false), None),
        ] {
            let now = Instant::now();
            let mut clock = UpdateClock::new(now);
            let mut game = Game::new(42);
            let mut input = LiveInput::default();
            let mut focused = true;
            game.start();
            for state in [ButtonState::Press, ButtonState::Release] {
                input.event(&mut game, &mut focused, &key_event(Key::W, state), &mut clock, now);
            }
            assert_eq!(clock.steps(0.004), 0);
            input.event(&mut game, &mut focused, &action, &mut clock, now);
            assert_eq!(clock.steps(0.004), 0);
            assert_eq!(input.step_axis(), 0.);
        }
    }

    #[test]
    fn opposite_quick_taps_cancel() {
        let now = Instant::now();
        let mut clock = UpdateClock::new(now);
        let mut game = Game::new(42);
        let mut input = LiveInput::default();
        let mut focused = true;
        game.start();
        for key in [Key::W, Key::S] {
            for state in [ButtonState::Press, ButtonState::Release] {
                input.event(&mut game, &mut focused, &key_event(key, state), &mut clock, now);
            }
        }
        assert_eq!(input.step_axis(), 0.);
        assert_eq!(input.step_axis(), 0.);
    }

}

fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let mut window: PistonWindow = WindowSettings::new(
        "Pong Piston | Enter: start | W/S/arrows: move | Space: pause | R: restart",
        [1000, 680],
    )
    .exit_on_esc(false)
    .resizable(true)
    .build()
    .expect("create Piston window");
    window.set_ups(120);
    let mut game = Game::new(pong_scenario::SEED);
    let mut scenario = pong_scenario::Scenario::default();
    let mut benchmark: Option<pong_benchmark::Frames> = None;
    let mut demo = pong_scenario::Demo::from_args();
    let mut input = LiveInput::default();
    let mut clock = UpdateClock::new(Instant::now());
    let mut last_title = String::new();
    let mut focused = true;
    while let Some(e) = window.next() {
        if demo.is_none() && !benchmarking {
            input.event(&mut game, &mut focused, &e, &mut clock, Instant::now());
        }
        if e.update_args().is_some() {
            let elapsed = clock.elapsed(Instant::now());
            if let Some(demo) = &mut demo {
                demo.advance(&mut game, elapsed as f32);
            } else {
                for _ in 0..clock.steps(elapsed) {
                    if benchmarking {
                        if scenario.finished() {
                            scenario = pong_scenario::Scenario::default();
                            game = Game::new(pong_scenario::SEED);
                        }
                        scenario.advance(&mut game);
                    } else {
                        game.step(1. / 120., input.step_axis());
                    }
                }
            }
        }
        if e.render_args().is_some() {
            let title = window_title(&game);
            if title != last_title {
                window.set_title(title.clone());
                last_title = title;
            }
        }
        let framebuffer = [window.surface_config.width, window.surface_config.height];
        window.draw_2d(&e, |c, g, _| draw_scene(c, g, &game, framebuffer));
        if let Some(args) = e.render_args() {
            if benchmarking {
                let frames = benchmark.get_or_insert_with(|| {
                    pong_benchmark::Frames::new(args.draw_size[0], args.draw_size[1])
                });
                if frames.frame(args.draw_size[0], args.draw_size[1]) {
                    println!("PONG_BENCHMARK {}", frames.report());
                    break;
                }
            }
            if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
                capture(&mut window, args, &game, &path).expect("app-owned render target capture");
            }
            if demo.as_ref().is_some_and(|d| d.finished()) {
                break;
            }
        }
    }
}

// Wgpu maps the context's normalized coordinates to the actual render target.
// Use surface dimensions, not event draw_size, which can differ during resize.
fn court_scissor(size: [f64; 2], framebuffer: [u32; 2]) -> Option<[u32; 4]> {
    if size.iter().any(|v| !v.is_finite() || *v <= 0.) || framebuffer.contains(&0) {
        return None;
    }
    let scale = (size[0] / W as f64).min(size[1] / H as f64);
    let left = (size[0] - W as f64 * scale) / 2.;
    let top = (size[1] - H as f64 * scale) / 2.;
    let xs = framebuffer[0] as f64 / size[0];
    let ys = framebuffer[1] as f64 / size[1];
    let x0 = (left * xs).ceil().clamp(0., framebuffer[0] as f64) as u32;
    let y0 = (top * ys).ceil().clamp(0., framebuffer[1] as f64) as u32;
    let x1 = ((left + W as f64 * scale) * xs).floor().clamp(0., framebuffer[0] as f64) as u32;
    let y1 = ((top + H as f64 * scale) * ys).floor().clamp(0., framebuffer[1] as f64) as u32;
    (x1 > x0 && y1 > y0).then_some([x0, y0, x1.saturating_sub(x0), y1.saturating_sub(y0)])
}

fn draw_scene<G: piston_window::graphics::Graphics>(
    c: piston_window::graphics::Context,
    g: &mut G,
    game: &Game,
    framebuffer: [u32; 2],
) {
    clear([0.047, 0.129, 0.118, 1.], g);
    let size = c.get_view_size();
    let s = (size[0] / W as f64).min(size[1] / H as f64);
    let t = c
        .transform
        .trans((size[0] - W as f64 * s) / 2., (size[1] - H as f64 * s) / 2.)
        .scale(s, s);
    rectangle(
        [0.082, 0.247, 0.216, 1.],
        [0., 0., W as f64, H as f64],
        t,
        g,
    );
    for y in (0..500).step_by(28) {
        rectangle([0.27, 0.43, 0.35, 1.], [449., y as f64, 2., 12.], t, g);
    }
    for (x, y) in [(32., game.player), (W - 44., game.cpu)] {
        rectangle(
            [0.85, 1., 0.68, 1.],
            [
                x as f64,
                (y - PADDLE_H / 2.) as f64,
                PADDLE_W as f64,
                PADDLE_H as f64,
            ],
            t,
            g,
        );
    }
    // The convenience ellipse function uses a default DrawState and drops scissor.
    // RenderArgs provides a full-frame viewport; skip the ball for other layouts.
    if let Some(viewport) = c.viewport {
        if viewport.rect[0] == 0 && viewport.rect[1] == 0
            && u32::try_from(viewport.rect[2]).ok() == Some(viewport.draw_size[0])
            && u32::try_from(viewport.rect[3]).ok() == Some(viewport.draw_size[1])
        {
            if let Some(clip) = court_scissor(size, framebuffer) {
                piston_window::graphics::Ellipse::new([1.; 4]).draw(
                    [
                        (game.ball.x - BALL) as f64,
                        (game.ball.y - BALL) as f64,
                        (BALL * 2.) as f64,
                        (BALL * 2.) as f64,
                    ],
                    &c.draw_state.scissor(clip),
                    t,
                    g,
                );
            }
        }
    }
    // Score bars avoid platform-dependent font files; full numeric score/status in title.
    for (side, n) in game.scores.iter().enumerate() {
        for i in 0..*n {
            rectangle(
                [0.85, 1., 0.68, 1.],
                [330. + side as f64 * 170. + i as f64 * 12., 20., 8., 18.],
                t,
                g,
            );
        }
    }
    for cell in score_owner_cells(s) {
        rectangle([0.85, 1., 0.68, 1.], cell, t, g);
    }
    // Cover the ball and net before drawing the state, preserving legibility.
    if let Some(panel) = indicator_backing(game) {
        rectangle([0.047, 0.129, 0.118, 1.], panel, t, g);
    }
    if let Some(panel) = hint_backing(game.state, s) {
        rectangle([0.047, 0.129, 0.118, 1.], panel, t, g);
    }
    for cell in state_indicator_cells(game.state)
        .into_iter()
        .chain(winner_cells(game))
        .chain(hint_cells(game.state, s))
    {
        rectangle([0.85, 1., 0.68, 1.], cell, t, g);
    }
}

fn capture(
    window: &mut PistonWindow,
    args: RenderArgs,
    game: &Game,
    path: &std::path::Path,
) -> Result<(), String> {
    let capture = Some(path);
    let texture = window.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Pong app capture target"),
        size: wgpu::Extent3d {
            width: window.surface_config.width,
            height: window.surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: window.surface_config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let framebuffer = [window.surface_config.width, window.surface_config.height];
    let (_, commands) = window
        .g2d
        .draw(&window.surface_config, &view, args.viewport(), |c, g| {
            draw_scene(c, g, game, framebuffer)
        });
    window.queue.submit([commands]);
    let mut encoder = window.device.create_command_encoder(&Default::default());
    let readback = if capture.is_some() {
        match window.surface_config.format {
            wgpu::TextureFormat::Rgba8Unorm
            | wgpu::TextureFormat::Rgba8UnormSrgb
            | wgpu::TextureFormat::Bgra8Unorm
            | wgpu::TextureFormat::Bgra8UnormSrgb => {}
            other => return Err(format!("Capture pixel format unsupported: {other:?}")),
        }
        let stride = (window.surface_config.width * 4).div_ceil(256) * 256;
        let buffer = window.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("app capture readback"),
            size: u64::from(stride) * u64::from(window.surface_config.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: Some(window.surface_config.height),
                },
            },
            wgpu::Extent3d {
                width: window.surface_config.width,
                height: window.surface_config.height,
                depth_or_array_layers: 1,
            },
        );
        Some((buffer, stride))
    } else {
        None
    };
    window.queue.submit([encoder.finish()]);
    if let (Some(path), Some((buffer, stride))) = (capture, readback) {
        let (tx, rx) = std::sync::mpsc::channel();
        buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            let _ = tx.send(result);
        });
        window
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .map_err(|e| e.to_string())?;
        rx.recv_timeout(std::time::Duration::from_secs(1))
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        let bytes = buffer.get_mapped_range(..);
        let bgra = matches!(
            window.surface_config.format,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
        );
        let mut rgb = Vec::with_capacity(
            window.surface_config.width as usize * window.surface_config.height as usize,
        );
        for row in bytes
            .chunks(stride as usize)
            .take(window.surface_config.height as usize)
        {
            for c in row[..window.surface_config.width as usize * 4]
                .as_chunks::<4>()
                .0
            {
                let (r, b) = if bgra { (c[2], c[0]) } else { (c[0], c[2]) };
                rgb.push((u32::from(r) << 16) | (u32::from(c[1]) << 8) | u32::from(b));
            }
        }
        pong_scenario::Demo::save_rgb(
            path,
            &rgb,
            window.surface_config.width,
            window.surface_config.height,
        );
        drop(bytes);
        buffer.unmap();
    }

    Ok(())
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    #[test]
    fn benchmark_is_opt_in() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "benchmark and replay/capture modes are distinct")]
    fn rejects_capture_mixing() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
}
