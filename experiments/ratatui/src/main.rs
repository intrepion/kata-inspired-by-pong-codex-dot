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
#[derive(Default)]
struct Input {
    up: bool,
    down: bool,
    latched: f32,
}
impl Input {
    fn clear(&mut self) {
        *self = Self::default();
    }
    fn axis(&self, releases: bool) -> f32 {
        if releases {
            self.down as u8 as f32 - self.up as u8 as f32
        } else {
            self.latched
        }
    }
    fn key(&mut self, key: KeyEvent, releases: bool) {
        let down = key.kind != KeyEventKind::Release;
        match key.code {
            KeyCode::Up | KeyCode::Char('w') => {
                if releases {
                    self.up = down
                } else if down {
                    self.latched = -1.
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                if releases {
                    self.down = down
                } else if down {
                    self.latched = 1.
                }
            }
            KeyCode::Char('x') => self.clear(),
            _ => {}
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
fn main() -> io::Result<()> {
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
                        match k.code {
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
