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
        Block, Borders, Paragraph,
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
        let down = key.kind != KeyEventKind::Release;
        let binding = match key.code {
            KeyCode::Up => Some(0),
            KeyCode::Char('w' | 'W') => Some(1),
            KeyCode::Down => Some(2),
            KeyCode::Char('s' | 'S') => Some(3),
            KeyCode::Char('x' | 'X') => {
                self.clear();
                None
            }
            _ => None,
        };
        if let Some(binding) = binding {
            if releases {
                self.held[binding] = down;
            } else if down {
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
fn draw(f: &mut Frame, g: &Game, releases: bool) {
    let a = f.area();
    if a.width < 50 || a.height < 18 {
        f.render_widget(
            Paragraph::new(
                "Terminal too small. Resize to at least 50 × 18.\nGame paused. Q quits.",
            ),
            a,
        );
        return;
    }
    let help = if releases {
        "Hold ↑/↓ or W/S · X stop · Space pause/start · R restart · Q quit"
    } else {
        "LATCHED INPUT: ↑/W up · ↓/S down · X STOP · Space pause · R restart · Q quit"
    };
    f.render_widget(
        Paragraph::new(format!(
            "PONG / RATATUI     YOU {} : {} CPU     {:?}\n{}",
            g.scores[0], g.scores[1], g.state, help
        )),
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
    let mut prev = Instant::now();
    let mut acc = 0.;
    let dt = 1. / 120.;
    'game: loop {
        // Bound event processing so autorepeat cannot starve simulation/rendering.
        for _ in 0..128 {
            if !event::poll(Duration::ZERO)? {
                break;
            }
            match event::read()? {
                Event::Key(k) => {
                    if k.kind == KeyEventKind::Press {
                        match action_key(k.code) {
                            KeyCode::Esc | KeyCode::Char('q') => break 'game,
                            KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => {
                                break 'game
                            }
                            KeyCode::Char(' ') => {
                                if matches!(g.state, State::Ready | State::Over) {
                                    g.start()
                                } else {
                                    g.toggle_pause()
                                }
                                input.clear();
                                acc = 0.;
                            }
                            KeyCode::Char('r') => {
                                g.start();
                                input.clear();
                                acc = 0.;
                            }
                            _ => {}
                        }
                    }
                    if g.state == State::Playing {
                        input.key(k, releases);
                    }
                }
                Event::FocusLost | Event::Resize(_, _) => {
                    g.pause();
                    input.clear();
                    acc = 0.;
                }
                _ => {}
            }
        }
        let now = Instant::now();
        acc += (now - prev).as_secs_f32().min(0.1);
        prev = now;
        let size = term.size()?;
        if size.width < 50 || size.height < 18 {
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
    fn fallback_explicit_stop() {
        let mut i = Input::default();
        i.key(key(KeyCode::Up, KeyEventKind::Press), false);
        assert_eq!(i.axis(false), -1.);
        i.key(key(KeyCode::Char('x'), KeyEventKind::Press), false);
        assert_eq!(i.axis(false), 0.);
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
