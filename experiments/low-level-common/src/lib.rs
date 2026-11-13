//! Identical rectangles, input, and timing for the three low-level comparisons.
pub use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
pub const BACKGROUND: [u8; 3] = [12, 20, 35];
// Minimal 5x7 uppercase glyphs for in-court status text.
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
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [u8; 3],
}
pub struct App {
    pub game: Game,
    pub up: bool,
    pub down: bool,
    acc: f32,
}
impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            up: false,
            down: false,
            acc: 0.,
        }
    }
}
impl App {
    pub fn tick(&mut self, seconds: f32) {
        if !seconds.is_finite() {
            return;
        }
        self.acc += seconds.clamp(0., 0.1);
        while self.acc >= 1. / 120. {
            self.game
                .step(1. / 120., self.down as u8 as f32 - self.up as u8 as f32);
            self.acc -= 1. / 120.;
        }
    }
    pub fn action(&mut self) {
        match self.game.state {
            State::Ready | State::Over => self.game.start(),
            _ => self.game.toggle_pause(),
        }
    }
    pub fn blur(&mut self) {
        self.up = false;
        self.down = false;
        self.game.pause();
        self.acc = 0.;
    }
    pub fn title(&self, backend: &str) -> String {
        let status: String = match self.game.state {
            State::Over if self.game.scores[0] > self.game.scores[1] => "YOU WIN".into(),
            State::Over if self.game.scores[1] > self.game.scores[0] => "CPU WINS".into(),
            State::Over => "MATCH OVER".into(),
            state => format!("{state:?}"),
        };
        format!("Pong {backend} | {status} | {} : {} | W/S or arrows • Space start/pause • R restart • Esc quit",self.game.scores[0],self.game.scores[1])
    }
    pub fn rectangles(&self) -> Vec<Rect> {
        let mut out = Vec::new();
        let mut rect = |x, y, w, h, color| out.push(Rect { x, y, w, h, color });
        for y in (0..500).step_by(25) {
            rect(449., y as f32, 2., 12., [45, 65, 90]);
        }
        for (x, y) in [(38., self.game.player), (W - 38., self.game.cpu)] {
            rect(
                x - PADDLE_W / 2.,
                y - PADDLE_H / 2.,
                PADDLE_W,
                PADDLE_H,
                [89, 242, 217],
            );
        }
        rect(
            self.game.ball.x - BALL,
            self.game.ball.y - BALL,
            BALL * 2.,
            BALL * 2.,
            [248, 240, 213],
        );
        // Seven-segment digits: score stays visible even if OS hides the title bar.
        const DIGITS: [u8; 10] = [0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f];
        let segments = [
            (0., 0., 22., 4.),
            (18., 0., 4., 20.),
            (18., 18., 4., 20.),
            (0., 34., 22., 4.),
            (0., 18., 4., 20.),
            (0., 0., 4., 20.),
            (0., 17., 22., 4.),
        ];
        for (left, label) in [(343., "YOU"), (521., "CPU")] {
            for (i, ch) in label.bytes().enumerate() {
                for (y, row) in result_glyph(ch).iter().enumerate() {
                    for col in 0..5 {
                        if row & (1 << (4 - col)) != 0 {
                            rect(left + i as f32 * 12. + col as f32 * 2., 5. + y as f32 * 2., 2., 2., [220, 230, 245]);
                        }
                    }
                }
            }
        }
        for (x, n) in [(350., self.game.scores[0]), (528., self.game.scores[1])] {
            for (i, (a, b, w, h)) in segments.iter().enumerate() {
                if DIGITS[n.min(9) as usize] & (1 << i) != 0 {
                    rect(x + a, 25. + b, *w, *h, [220, 230, 245]);
                }
            }
        }
        if matches!(self.game.state, State::Ready | State::Paused) {
            rect(300., 210., 300., 110., BACKGROUND);
        }
        if self.game.state == State::Ready {
            for (i, width) in [8., 16., 24., 32., 24., 16., 8.].into_iter().enumerate() {
                rect(430., 222. + i as f32 * 8., width, 8., [255, 180, 80]);
            }
        } else if self.game.state == State::Paused {
            rect(420., 225., 12., 50., [255, 180, 80]);
            rect(468., 225., 12., 50., [255, 180, 80]);
        }
        let instruction = match self.game.state {
            State::Ready => Some("SPACE TO START"),
            State::Paused => Some("SPACE TO RESUME"),
            _ => None,
        };
        if let Some(message) = instruction {
            let left = (W - message.len() as f32 * 18.) / 2.;
            for (i, ch) in message.bytes().enumerate() {
                for (y, row) in result_glyph(ch).iter().enumerate() {
                    for col in 0..5 {
                        if row & (1 << (4 - col)) != 0 {
                            rect(left + i as f32 * 18. + col as f32 * 3., 290. + y as f32 * 3., 3., 3., [255, 180, 80]);
                        }
                    }
                }
            }
        }
        if self.game.state == State::Over {
            let winner = if self.game.scores[0] > self.game.scores[1] {
                Some((344., "YOU WIN"))
            } else if self.game.scores[1] > self.game.scores[0] {
                Some((522., "CPU WINS"))
            } else {
                None
            };
            if let Some((x, message)) = winner {
                rect(x, 70., 34., 6., [255, 180, 80]);
                let left = (W - message.len() as f32 * 36.) / 2.;
                rect(left - 8., 213., message.len() as f32 * 36. + 16., 66., BACKGROUND);
                for (i, ch) in message.bytes().enumerate() {
                    for (y, row) in result_glyph(ch).iter().enumerate() {
                        for col in 0..5 {
                            if row & (1 << (4 - col)) != 0 {
                                rect(
                                    left + i as f32 * 36. + col as f32 * 6.,
                                    225. + y as f32 * 6.,
                                    6.,
                                    6.,
                                    [255, 180, 80],
                                );
                            }
                        }
                    }
                }
            }
        }
        out
    }
}
pub fn viewport(width: u32, height: u32) -> (f32, f32, f32) {
    let s = (width as f32 / W).min(height as f32 / H);
    (s, (width as f32 - W * s) / 2., (height as f32 - H * s) / 2.)
}
pub fn raster(app: &App, pixels: &mut [u32], width: u32, height: u32) {
    pixels.fill(0x000c1423);
    let (s, ox, oy) = viewport(width, height);
    for r in app.rectangles() {
        let x0 = (ox + r.x * s).max(0.) as u32;
        let y0 = (oy + r.y * s).max(0.) as u32;
        let x1 = ((ox + (r.x + r.w) * s).ceil() as u32).min(width);
        let y1 = ((oy + (r.y + r.h) * s).ceil() as u32).min(height);
        let c = (r.color[0] as u32) << 16 | (r.color[1] as u32) << 8 | r.color[2] as u32;
        for y in y0..y1 {
            for x in x0..x1 {
                if let Some(p) = pixels.get_mut((y * width + x) as usize) {
                    *p = c;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn instruction_glyphs_exist() {
        for ch in b"SPACE TO STARTSPACE TO RESUME" {
            if *ch != b' ' {
                assert!(result_glyph(*ch).iter().any(|row| *row != 0), "missing glyph: {}", *ch as char);
            }
        }
    }
    #[test]    fn ready_play_indicator_differs_from_paused_bars() {
        let mut app = App::default();
        let ready = app.rectangles();
        assert!(ready
            .iter()
            .any(|r| r.x == 430. && r.y == 246. && r.w == 32. && r.h == 8.));
        assert!(!ready
            .iter()
            .any(|r| r.x == 420. && r.y == 225. && r.w == 12. && r.h == 50.));
        app.action();
        app.action();
        assert_eq!(app.game.state, State::Paused);
        let paused = app.rectangles();
        assert!(paused
            .iter()
            .any(|r| r.x == 420. && r.y == 225. && r.w == 12. && r.h == 50.));
        assert!(!paused
            .iter()
            .any(|r| r.x == 430. && r.y == 246. && r.w == 32. && r.h == 8.));
    }
    #[test]
    fn winner_title_and_score_marker_follow_winning_point() {
        for (scores, x, vx, text, marker_x, other_x) in [
            ([6, 3], W + BALL - 1., 350., "YOU WIN", 350, 528),
            ([2, 6], -BALL + 1., -350., "CPU WINS", 528, 350),
        ] {
            let mut a = App::default();
            a.game.start();
            a.game.scores = scores;
            a.game.serve = 0.;
            a.game.ball.x = x;
            a.game.ball.y = H / 2.;
            a.game.ball.vx = vx;
            a.game.ball.vy = 0.;
            a.game.step(0.02, 0.);
            assert_eq!(a.game.state, State::Over);
            assert!(a.title("test").contains(text));
            let mut pixels = vec![0; 900 * 500];
            raster(&a, &mut pixels, 900, 500);
            assert_eq!(pixels[72 * 900 + marker_x], 0xffb450);
            assert_ne!(pixels[72 * 900 + other_x], 0xffb450);
            assert!(!a
                .rectangles()
                .iter()
                .any(|r| r.x == 420. && r.y == 225. && r.w == 12. && r.h == 50.));
            a.action();
            assert_eq!(a.game.state, State::Playing);
            assert!(!a.title("test").contains(text));
            raster(&a, &mut pixels, 900, 500);
            assert_ne!(pixels[72 * 900 + marker_x], 0xffb450);
        }
    }
    #[test]
    fn start_pause_focus() {
        let mut a = App::default();
        a.action();
        assert_eq!(a.game.state, State::Playing);
        a.up = true;
        a.tick(0.1);
        assert!(a.game.player < 250.);
        a.blur();
        assert!(!a.up);
        assert_eq!(a.game.state, State::Paused);
    }
    #[test]
    fn render_and_letterbox() {
        let a = App::default();
        let mut p = vec![0; 900 * 500];
        raster(&a, &mut p, 900, 500);
        assert_eq!(p[250 * 900 + 38], 0x59f2d9);
        assert_eq!(viewport(1000, 500), (1., 50., 0.));
    }
    #[test]
    fn bounded_time() {
        let mut a = App::default();
        a.action();
        a.up = true;
        a.tick(100.);
        assert!(a.game.player > 200.);
        a.tick(f32::NAN);
        assert!(a.game.player.is_finite());
    }
}
