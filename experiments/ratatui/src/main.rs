use crossterm::{
    event::{
        self, DisableFocusChange, EnableFocusChange, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph, Wrap,
    },
    Frame, Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};
// Terminal character events carry Shift/Caps Lock case, unlike physical key codes.
fn action_key(key: KeyCode) -> KeyCode {
    match key {
        KeyCode::Char(c) => KeyCode::Char(c.to_ascii_lowercase()),
        other => other,
    }
}
#[derive(Default)]
struct Input {
    held: [bool; 4], // Up, W, Down, S: aliases release independently.
    latched: f32,
}
impl Input {
    fn clear(&mut self) {
        *self = Self::default();
    }
    fn axis(&self, releases: bool) -> f32 {
        if releases {
            (self.held[2] || self.held[3]) as u8 as f32
                - (self.held[0] || self.held[1]) as u8 as f32
        } else {
            self.latched
        }
    }
    fn key(&mut self, key: KeyEvent, releases: bool) {
        let binding = match key.code {
            KeyCode::Up => Some(0),
            KeyCode::Char('w' | 'W') => Some(1),
            KeyCode::Down => Some(2),
            KeyCode::Char('s' | 'S') => Some(3),
            KeyCode::Char('x' | 'X') => {
                if key.kind == KeyEventKind::Press { self.clear(); }
                None
            }
            _ => None,
        };
        if let Some(binding) = binding {
            if releases {
                match key.kind {
                    KeyEventKind::Press => self.held[binding] = true,
                    KeyEventKind::Release => self.held[binding] = false,
                    KeyEventKind::Repeat => {} // A repeat cannot re-arm a cleared binding.
                }
            } else if key.kind == KeyEventKind::Press {
                self.latched = if binding < 2 { -1. } else { 1. };
            }
        }
    }
}
static NEEDS_RESTORE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
struct Restore {
    enhanced: bool,
}
impl Drop for Restore {
    fn drop(&mut self) {
        restore(self.enhanced);
    }
}
fn restore(enhanced: bool) {
    if !NEEDS_RESTORE.swap(false, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    if enhanced {
        let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
    }
    let _ = execute!(
        stdout(),
        DisableFocusChange,
        LeaveAlternateScreen,
        crossterm::cursor::Show
    );
    let _ = terminal::disable_raw_mode();
}
fn too_small(width: u16, height: u16) -> bool { width < 50 || height < 18 }
// Time preceding a start, pause, focus loss, or resize belongs to the old state.
fn add_frame_elapsed(acc: &mut f32, elapsed: f32, transitioned: bool) {
    if transitioned { *acc = 0.; } else { *acc += elapsed.min(0.1); }
}
fn draw(f: &mut Frame, g: &Game, releases: bool) {
    let a = f.area();
    if too_small(a.width, a.height) {
        f.render_widget(
            Paragraph::new(
                "Terminal too small. Resize to at least 50 × 18.\nGame paused. Q quits.",
            ),
            a,
        );
        return;
    }
    let help = if releases {
        "Hold ↑/↓ or W/S · X stop · Space pause/start\nR restart · Q quit"
    } else {
        "LATCHED INPUT: ↑/W up · ↓/S down · X STOP\nSpace pause/start · R restart · Q quit"
    };
    let status = match g.state {
        State::Ready => "Ready",
        State::Playing => "Playing",
        State::Paused => "Paused",
        State::Over if g.scores[0] > g.scores[1] => "YOU WIN",
        State::Over if g.scores[1] > g.scores[0] => "CPU WINS",
        State::Over => "Match over",
    };
    f.render_widget(
        Paragraph::new(format!(
            "PONG / RATATUI     YOU {} : {} CPU     {}\n{}",
            g.scores[0], g.scores[1], status, help
        )).wrap(Wrap { trim: true }),
        Rect::new(a.x, a.y, a.width, 3),
    );
    // Terminal cells are approximately twice as tall as wide.
    let avail = Rect::new(a.x, a.y + 3, a.width, a.height - 3);
    let h = (avail.height as f32).min(avail.width as f32 * H / W / 2.) as u16;
    let w = (h as f32 * W / H * 2.) as u16;
    let area = Rect::new(
        avail.x + (avail.width - w) / 2,
        avail.y + (avail.height - h) / 2,
        w,
        h,
    );
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .marker(Marker::HalfBlock)
        .x_bounds([0., W as f64])
        .y_bounds([0., H as f64])
        .paint(|ctx| {
            ctx.draw(&Line {
                x1: W as f64 / 2.,
                y1: 0.,
                x2: W as f64 / 2.,
                y2: H as f64,
                color: Color::DarkGray,
            });
            for (x, y) in [(38., g.player), (W - 38., g.cpu)] {
                ctx.draw(&Rectangle {
                    x: (x - PADDLE_W / 2.) as f64,
                    y: (H - y - PADDLE_H / 2.) as f64,
                    width: PADDLE_W as f64,
                    height: PADDLE_H as f64,
                    color: Color::Cyan,
                });
            }
            ctx.draw(&Rectangle {
                x: (g.ball.x - BALL) as f64,
                y: (H - g.ball.y - BALL) as f64,
                width: (BALL * 2.) as f64,
                height: (BALL * 2.) as f64,
                color: Color::White,
            });
        });
    f.render_widget(canvas, area);
}
fn cell_evidence(buffer: &ratatui::buffer::Buffer) -> serde_json::Value {
    serde_json::json!({
        "evidence": "Ratatui TestBackend cell buffer, NOT a screenshot",
        "width": buffer.area.width, "height": buffer.area.height,
        "cells": buffer.content.iter().map(|c| serde_json::json!({
            "symbol": c.symbol(), "foreground": format!("{:?}", c.fg),
            "background": format!("{:?}", c.bg), "modifier": format!("{:?}", c.modifier)
        })).collect::<Vec<_>>()
    })
}
fn cell_text(buffer: &ratatui::buffer::Buffer) -> String {
    buffer
        .content
        .chunks(buffer.area.width as usize)
        .map(|row| row.iter().map(|c| c.symbol()).collect::<String>() + "\n")
        .collect()
}
fn scripted_cells(args: &[String]) -> io::Result<()> {
    assert!(
        !args.iter().any(|a| a == "--scripted-captures"),
        "Ratatui exports cells, not PNGs; use --scripted-cells NEW_DIRECTORY"
    );
    let directory = args.iter().position(|a| a == "--scripted-cells").map(|i| {
        let p = std::path::PathBuf::from(args.get(i + 1).expect("missing NEW cell directory"));
        std::fs::create_dir(&p).expect("create NEW cell directory");
        p
    });
    let mut demo = pong_scenario::Demo::new(directory);
    let mut game = Game::new(42);
    let mut term = Terminal::new(ratatui::backend::TestBackend::new(100, 30))?;
    while !demo.finished() {
        demo.advance(&mut game, pong_scenario::STEP);
        term.draw(|f| draw(f, &game, false))?;
        if let Some(path) = demo.capture_path() {
            let buffer = term.backend().buffer();
            std::fs::write(
                path.with_extension("json"),
                serde_json::to_vec_pretty(&cell_evidence(buffer))?,
            )?;
            std::fs::write(path.with_extension("txt"), cell_text(buffer))?;
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.iter().any(|a| a == "--scripted-test") {
        return scripted_cells(&args);
    }
    assert!(
        !args
            .iter()
            .any(|a| a == "--scripted-cells" || a == "--scripted-captures"),
        "cell export requires --scripted-test"
    );

    // Query before starting any event reader: the probe itself consumes a terminal response.
    let enhanced = terminal::supports_keyboard_enhancement().unwrap_or(false);
    let releases = cfg!(windows) || enhanced;
    let old = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore(enhanced);
        old(info);
    }));
    NEEDS_RESTORE.store(true, std::sync::atomic::Ordering::SeqCst);
    let _guard = Restore { enhanced };
    terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableFocusChange)?;
    if enhanced {
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.hide_cursor()?;
    let mut g = Game::new(42);
    let mut input = Input::default();
    let mut focused = true;
    let mut prev = Instant::now();
    let mut acc = 0.;
    let dt = 1. / 120.;
    'game: loop {
        let size = term.size()?;
        let mut blocked_by_size = too_small(size.width, size.height);
        let mut transitioned = false;
        // Bound event processing so autorepeat cannot starve simulation/rendering.
        for _ in 0..128 {
            if !event::poll(Duration::ZERO)? {
                break;
            }
            match event::read()? {
                Event::Key(k) if focused => {
                    if k.kind == KeyEventKind::Press {
                        match action_key(k.code) {
                            KeyCode::Esc | KeyCode::Char('q') => break 'game,
                            KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => {
                                break 'game
                            }
                            KeyCode::Char(' ') if !blocked_by_size => {
                                if matches!(g.state, State::Ready | State::Over) {
                                    g.start()
                                } else {
                                    g.toggle_pause()
                                }
                                input.clear();
                                acc = 0.;
                                transitioned = true;
                            }
                            KeyCode::Char('r') if !blocked_by_size => {
                                g.start();
                                input.clear();
                                acc = 0.;
                                transitioned = true;
                            }
                            _ => {}
                        }
                    }
                    if !blocked_by_size && g.state == State::Playing {
                        input.key(k, releases);
                    }
                }
                Event::FocusLost => {
                    focused = false;
                    g.pause();
                    input.clear();
                    acc = 0.;
                    transitioned = true;
                }
                Event::FocusGained => focused = true,
                Event::Resize(width, height) => {
                    blocked_by_size = too_small(width, height);
                    g.pause();
                    input.clear();
                    acc = 0.;
                    transitioned = true;
                }
                _ => {}
            }
        }
        let now = Instant::now();
        add_frame_elapsed(&mut acc, (now - prev).as_secs_f32(), transitioned);
        prev = now;
        let size = term.size()?;
        if too_small(size.width, size.height) {
            g.pause();
            input.clear();
            acc = 0.;
        }
        while acc >= dt {
            g.step(dt, input.axis(releases));
            acc -= dt;
        }
        term.draw(|f| draw(f, &g, releases))?;
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn key(c: KeyCode, k: KeyEventKind) -> KeyEvent {
        KeyEvent::new_with_kind(c, KeyModifiers::NONE, k)
    }
    #[test]
    fn stop_release_and_repeat_do_not_cancel_new_movement() {
        for releases in [true, false] {
            for kind in [KeyEventKind::Release, KeyEventKind::Repeat] {
                let mut input = Input::default();
                input.key(key(KeyCode::Char('x'), KeyEventKind::Press), releases);
                input.key(key(KeyCode::Char('w'), KeyEventKind::Press), releases);
                input.key(key(KeyCode::Char('x'), kind), releases);
                assert_eq!(input.axis(releases), -1., "{kind:?}, releases={releases}");
            }
        }
    }
    #[test]
    fn action_letters_are_case_insensitive() {
        for (lower, upper) in [('r', 'R'), ('q', 'Q'), ('c', 'C')] {
            assert_eq!(action_key(KeyCode::Char(upper)), KeyCode::Char(lower));
        }
        assert_eq!(action_key(KeyCode::Esc), KeyCode::Esc);
        assert_eq!(action_key(KeyCode::Char(' ')), KeyCode::Char(' '));
    }
    #[test]
    fn release_stops_and_opposites_cancel() {
        let mut i = Input::default();
        i.key(key(KeyCode::Up, KeyEventKind::Press), true);
        assert_eq!(i.axis(true), -1.);
        i.key(key(KeyCode::Down, KeyEventKind::Press), true);
        assert_eq!(i.axis(true), 0.);
        i.key(key(KeyCode::Up, KeyEventKind::Release), true);
        assert_eq!(i.axis(true), 1.);
        i.key(key(KeyCode::Down, KeyEventKind::Release), true);
        assert_eq!(i.axis(true), 0.);
    }
    #[test]
    fn overlapping_direction_bindings_release_independently() {
        for (arrow, letter, axis) in [(KeyCode::Up, 'w', -1.), (KeyCode::Down, 's', 1.)] {
            let mut input = Input::default();
            input.key(key(arrow, KeyEventKind::Press), true);
            input.key(key(KeyCode::Char(letter), KeyEventKind::Press), true);
            input.key(key(arrow, KeyEventKind::Release), true);
            assert_eq!(input.axis(true), axis);
            input.key(key(KeyCode::Char(letter), KeyEventKind::Release), true);
            assert_eq!(input.axis(true), 0.);
        }
    }
    #[test]
    fn case_changed_release_stops_enhanced_movement() {
        let mut input = Input::default();
        input.key(key(KeyCode::Char('w'), KeyEventKind::Press), true);
        input.key(key(KeyCode::Char('W'), KeyEventKind::Release), true);
        assert_eq!(input.axis(true), 0.);
    }
    #[test]
    fn fallback_repeat_cannot_rearm_after_stop_or_clear() {
        for (binding, axis) in [(KeyCode::Up,-1.),(KeyCode::Char('w'),-1.),(KeyCode::Down,1.),(KeyCode::Char('s'),1.)] {
            for stop in [true,false] {
                let mut input=Input::default();
                input.key(key(binding,KeyEventKind::Press),false);
                assert_eq!(input.axis(false),axis);
                if stop { input.key(key(KeyCode::Char('x'),KeyEventKind::Press),false); } else { input.clear(); }
                assert_eq!(input.axis(false),0.);
                input.key(key(binding,KeyEventKind::Repeat),false);
                assert_eq!(input.axis(false),0.,"repeat rearmed {binding:?} after stop={stop}");
                input.key(key(binding,KeyEventKind::Press),false);
                assert_eq!(input.axis(false),axis,"fresh press failed for {binding:?}");
            }
        }
    }
    #[test]
    fn fallback_explicit_stop() {
        let mut i = Input::default();
        i.key(key(KeyCode::Up, KeyEventKind::Press), false);
        assert_eq!(i.axis(false), -1.);
        i.key(key(KeyCode::Char('x'), KeyEventKind::Press), false);
        assert_eq!(i.axis(false), 0.);
    }
    #[test]
    fn enhanced_repeat_cannot_restore_movement_after_pause() {
        for (binding, axis) in [
            (KeyCode::Up, -1.),
            (KeyCode::Char('w'), -1.),
            (KeyCode::Down, 1.),
            (KeyCode::Char('s'), 1.),
        ] {
            let mut input = Input::default();
            input.key(key(binding, KeyEventKind::Press), true);
            input.clear();
            input.key(key(binding, KeyEventKind::Repeat), true);
            assert_eq!(input.axis(true), 0.);
            input.key(key(binding, KeyEventKind::Release), true);
            input.key(key(binding, KeyEventKind::Press), true);
            assert_eq!(input.axis(true), axis);
        }
    }
    #[test]
    fn enhanced_repeat_does_not_rearm_after_reset_and_aliases_are_independent() {
        let mut input = Input::default();
        input.key(key(KeyCode::Char('w'), KeyEventKind::Press), true);
        input.clear();
        input.key(key(KeyCode::Char('W'), KeyEventKind::Repeat), true);
        input.key(key(KeyCode::Down, KeyEventKind::Press), true);
        assert_eq!(input.axis(true), 1.);
        input.key(key(KeyCode::Down, KeyEventKind::Repeat), true);
        assert_eq!(input.axis(true), 1.);
        input.key(key(KeyCode::Down, KeyEventKind::Release), true);
        assert_eq!(input.axis(true), 0.);
    }
    #[test]
    fn clear_prevents_stuck_key() {
        let mut i = Input::default();
        i.key(key(KeyCode::Up, KeyEventKind::Press), true);
        i.clear();
        assert_eq!(i.axis(true), 0.);
    }
    #[test]
    fn render_small_and_normal() {
        use ratatui::backend::TestBackend;
        for (w, h) in [(20, 8), (100, 30)] {
            let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
            t.draw(|f| draw(f, &Game::new(1), false)).unwrap();
            let text = t
                .backend()
                .buffer()
                .content
                .iter()
                .map(|c| c.symbol())
                .collect::<String>();
            assert!(text.contains(if w < 50 { "too small" } else { "LATCHED INPUT" }));
        }
    }
}

#[cfg(test)]
mod scripted_tests {
    use super::*;
    #[test]
    fn match_winner_is_visible_at_minimum_terminal_width() {
        for (scores, x, vx, expected) in [
            ([6, 2], W + BALL - 1., 350., "YOU WIN"),
            ([2, 6], -BALL + 1., -350., "CPU WINS"),
        ] {
            let mut game = Game::new(42);
            game.start();
            game.scores = scores;
            game.serve = 0.;
            game.ball.x = x;
            game.ball.y = H / 2.;
            game.ball.vx = vx;
            game.ball.vy = 0.;
            game.step(0.02, 0.);
            assert_eq!(game.state, State::Over);
            let mut term = Terminal::new(ratatui::backend::TestBackend::new(50, 18)).unwrap();
            term.draw(|f| draw(f, &game, true)).unwrap();
            assert!(cell_text(term.backend().buffer()).contains(expected));
            game.start();
            term.draw(|f| draw(f, &game, true)).unwrap();
            assert!(!cell_text(term.backend().buffer()).contains(expected));
        }
    }
    #[test]
    fn exported_cells_preserve_rendered_symbols_and_styles() {
        let mut term = Terminal::new(ratatui::backend::TestBackend::new(100, 30)).unwrap();
        let mut game = Game::new(42);
        game.start();
        game.pause();
        game.scores = [0, 1];
        term.draw(|f| draw(f, &game, false)).unwrap();
        let buffer = term.backend().buffer();
        let evidence = cell_evidence(buffer);
        assert_eq!(evidence["cells"].as_array().unwrap().len(), 3000);
        assert!(cell_text(buffer).contains("YOU 0 : 1 CPU     Paused"));
        assert!(evidence["cells"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["foreground"] == "Cyan" || c["background"] == "Cyan"));
        assert!(evidence["evidence"]
            .as_str()
            .unwrap()
            .contains("NOT a screenshot"));
    }
}


#[cfg(test)]
mod minimum_help_test {
    use super::*;
    #[test]
    fn minimum_terminal_shows_restart_and_quit_in_both_keyboard_modes() {
        for releases in [false, true] {
            for state in [State::Ready, State::Playing, State::Paused, State::Over] {
                let mut game = Game::new(42);
                game.state = state;
                game.scores = [7, 2];
                let mut term = Terminal::new(ratatui::backend::TestBackend::new(50,18)).unwrap();
                term.draw(|frame| draw(frame, &game, releases)).unwrap();
                let text = cell_text(term.backend().buffer()).to_ascii_lowercase();
                for label in ["x stop", "space pause/start", "r restart", "q quit"] {
                    assert!(text.contains(label), "{label} missing in release mode {releases}, state {state:?}:\n{text}");
                }
            }
        }
    }
}

#[cfg(test)]
mod transition_clock_test {
    use super::*;
    #[test]
    fn resume_does_not_simulate_elapsed_time_from_before_the_action() {
        let mut game=Game::new(42); game.start(); game.pause();
        let mut acc=0.004f32;
        game.toggle_pause();
        let before=game.player;
        add_frame_elapsed(&mut acc, 0.1, true);
        while acc >= 1./120. { game.step(1./120., 1.); acc -= 1./120.; }
        assert_eq!(game.player, before, "resume consumed elapsed time from before transition");
        add_frame_elapsed(&mut acc, 1./60., false);
        while acc >= 1./120. { game.step(1./120., 1.); acc -= 1./120.; }
        assert!(game.player > before, "subsequent fresh elapsed time must progress");
    }
}
