//! Rendering-independent simulation. Coordinates match the original Canvas game.
pub const W: f32 = 900.;
pub const H: f32 = 500.;
pub const PADDLE_H: f32 = 90.;
pub const PADDLE_W: f32 = 12.;
pub const BALL: f32 = 8.;
pub const WIN: u32 = 7;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Ready,
    Playing,
    Paused,
    Over,
}
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}
#[derive(Clone, Debug)]
pub struct Game {
    pub player: f32,
    pub cpu: f32,
    pub ball: Ball,
    pub scores: [u32; 2],
    pub state: State,
    pub serve: f32,
    cpu_target: f32,
    cpu_reaction: f32,
    rng: u64,
}
impl Game {
    pub fn new(seed: u64) -> Self {
        Self {
            player: H / 2.,
            cpu: H / 2.,
            ball: Ball {
                x: W / 2.,
                y: H / 2.,
                vx: 0.,
                vy: 0.,
            },
            scores: [0, 0],
            state: State::Ready,
            serve: 0.,
            cpu_target: H / 2.,
            cpu_reaction: 0.,
            rng: seed.max(1),
        }
    }
    fn random(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        ((self.rng >> 40) as f32) / 16777216.
    }
    fn reset_ball(&mut self, dir: f32) {
        self.ball = Ball {
            x: W / 2.,
            y: H / 2.,
            vx: dir * 350.,
            vy: (self.random() - 0.5) * 180.,
        };
        self.serve = 0.65;
        self.cpu_target = H / 2.;
        self.cpu_reaction = 0.3;
    }
    pub fn start(&mut self) {
        self.scores = [0, 0];
        self.player = H / 2.;
        self.cpu = H / 2.;
        self.state = State::Playing;
        let dir = if self.random() < 0.5 { -1. } else { 1. };
        self.reset_ball(dir);
    }
    pub fn pause(&mut self) {
        if self.state == State::Playing {
            self.state = State::Paused
        }
    }
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            State::Playing => State::Paused,
            State::Paused => State::Playing,
            s => s,
        };
    }
    pub fn step(&mut self, dt: f32, input: f32) {
        if self.state != State::Playing || !dt.is_finite() || !input.is_finite() {
            return;
        }
        let dt = dt.clamp(0., 0.033);
        let old_player = self.player;
        let old_cpu = self.cpu;
        let player_speed = input.clamp(-1., 1.) * 470.;
        self.player = (self.player + player_speed * dt).clamp(PADDLE_H / 2., H - PADDLE_H / 2.);
        self.cpu_reaction -= dt;
        if self.cpu_reaction <= 0. {
            self.cpu_target = if self.ball.vx > 0. {
                (self.ball.y + (self.random() - 0.5) * 50.).clamp(PADDLE_H / 2., H - PADDLE_H / 2.)
            } else {
                H / 2.
            };
            self.cpu_reaction = 0.3;
        }
        let speed = if self.ball.vx > 0. { 190. } else { 130. };
        self.cpu = (self.cpu + (self.cpu_target - self.cpu).clamp(-speed * dt, speed * dt))
            .clamp(PADDLE_H / 2., H - PADDLE_H / 2.);
        if self.serve > 0. {
            self.serve -= dt;
            return;
        }
        let b = &mut self.ball;
        let old_x = b.x;
        let old_y = b.y;
        let old_vy = b.vy;
        b.x += b.vx * dt;
        b.y += b.vy * dt;
        if b.y < BALL {
            b.y = 2. * BALL - b.y;
            b.vy = b.vy.abs();
        }
        if b.y > H - BALL {
            b.y = 2. * (H - BALL) - b.y;
            b.vy = -b.vy.abs();
        }
        for (x, dir) in [(38., 1.), (W - 38., -1.)] {
            let face = x + dir * (PADDLE_W / 2. + BALL);
            let crosses = if dir == 1. {
                old_x >= face && b.x <= face
            } else {
                old_x <= face && b.x >= face
            };
            if !crosses || old_x == b.x {
                continue;
            }
            // Evaluate at the face crossing, including a wall bounce that
            // happened earlier in this step. At game speeds there is at most
            // one wall crossing per bounded simulation step.
            let fraction = (face - old_x) / (b.x - old_x);
            let mut hit_y = old_y + old_vy * dt * fraction;
            if hit_y < BALL {
                hit_y = 2. * BALL - hit_y;
            } else if hit_y > H - BALL {
                hit_y = 2. * (H - BALL) - hit_y;
            }
            let contact_time = dt * fraction;
            let y = if dir == 1. {
                (old_player + player_speed * contact_time).clamp(PADDLE_H / 2., H - PADDLE_H / 2.)
            } else {
                (old_cpu
                    + (self.cpu_target - old_cpu)
                        .clamp(-speed * contact_time, speed * contact_time))
                .clamp(PADDLE_H / 2., H - PADDLE_H / 2.)
            };
            if (hit_y - y).abs() <= PADDLE_H / 2. + BALL {
                b.y = hit_y;
                let angle = ((hit_y - y) / (PADDLE_H / 2.)).clamp(-1., 1.) * 1.05;
                let speed = b.vx.hypot(b.vy).mul_add(1.055, 0.).min(720.);
                b.x = face;
                b.vx = dir * speed * angle.cos();
                b.vy = speed * angle.sin();
                let remaining = dt - contact_time;
                b.x += b.vx * remaining;
                b.y += b.vy * remaining;
                if b.y < BALL {
                    b.y = 2. * BALL - b.y;
                    b.vy = b.vy.abs();
                } else if b.y > H - BALL {
                    b.y = 2. * (H - BALL) - b.y;
                    b.vy = -b.vy.abs();
                }
                break;
            }
        }
        if b.x < -BALL || b.x > W + BALL {
            let scorer = if b.x < 0. { 1 } else { 0 };
            self.scores[scorer] += 1;
            if self.scores[scorer] >= WIN {
                self.state = State::Over
            } else {
                self.reset_ball(if scorer == 0 { -1. } else { 1. });
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn game() -> Game {
        let mut g = Game::new(42);
        g.start();
        g.serve = 0.;
        g
    }
    #[test]
    fn start_and_bounds() {
        let mut g = game();
        for _ in 0..100 {
            g.step(0.033, -1.);
        }
        assert_eq!(g.player, 45.);
        assert_eq!(g.state, State::Playing);
    }
    #[test]
    fn paddle_steers() {
        let mut g = game();
        g.ball = Ball {
            x: 60.,
            y: 275.,
            vx: -350.,
            vy: 0.,
        };
        g.step(0.033, 0.);
        assert!(g.ball.vx > 0. && g.ball.vy > 0.);
    }
    #[test]
    fn paddle_collision_uses_height_at_face_crossing_on_both_sides() {
        for (x, vx) in [(60., -350.), (840., 350.)] {
            for (y, vy, should_bounce) in [(180., 600., false), (215., -600., true)] {
                let mut g = game();
                g.ball = Ball { x, y, vx, vy };
                g.step(0.033, 0.);
                assert_eq!(
                    g.ball.vx.signum() != vx.signum(),
                    should_bounce,
                    "crossing at x={x}, y={y}, vy={vy}"
                );
            }
        }
    }
    #[test]
    fn paddle_crossing_accounts_for_a_prior_wall_bounce() {
        for (x, vx) in [(60., -350.), (840., 350.)] {
            let mut g = game();
            g.player = 45.;
            g.cpu = 45.;
            g.ball = Ball {
                x,
                y: 10.,
                vx,
                vy: -600.,
            };
            g.step(0.033, 0.);
            assert_eq!(g.ball.vx.signum(), -vx.signum());
            // Reflected position at the face is inside the court.
            assert!(g.ball.y >= BALL && g.ball.y < 30.);
        }
    }
    #[test]
    fn moving_paddle_is_tested_at_contact_time() {
        let mut g = game();
        g.ball = Ball {
            x: 54.,
            y: 306.3,
            vx: -350.,
            vy: 0.,
        };
        g.step(1. / 120., 1.);
        assert!(g.ball.vx < 0., "ball clears the paddle at contact time");
    }
    #[test]
    fn ball_advances_after_paddle_contact() {
        let mut g = game();
        g.ball = Ball {
            x: 54.,
            y: 250.,
            vx: -350.,
            vy: 0.,
        };
        g.step(1. / 120., 0.);
        assert!(g.ball.x > 52.8, "ball should travel away after bouncing");
    }
    #[test]
    fn moving_cpu_is_tested_at_contact_time() {
        let mut g = game();
        g.cpu_target = 400.;
        g.cpu_reaction = 1.;
        g.ball = Ball {
            x: 846.,
            y: 304.3,
            vx: 350.,
            vy: 0.,
        };
        g.step(1. / 120., 0.);
        assert!(g.ball.vx > 0., "ball clears CPU paddle at contact time");
    }
    #[test]
    fn cpu_bounce_advances_after_contact() {
        let mut g = game();
        g.cpu_target = 250.;
        g.cpu_reaction = 1.;
        g.ball = Ball {
            x: 846.,
            y: 250.,
            vx: 350.,
            vy: 0.,
        };
        g.step(1. / 120., 0.);
        assert!(g.ball.x < 847.2);
    }
    #[test]
    fn wall_reflects_during_remaining_time_after_either_paddle_hit() {
        for (x, vx) in [(54., -350.), (846., 350.)] {
            let mut g = game();
            g.player = 45.;
            g.cpu = 45.;
            g.cpu_target = 45.;
            g.cpu_reaction = 1.;
            g.ball = Ball {
                x,
                y: 8.3,
                vx,
                vy: 0.,
            };
            g.step(1. / 120., 0.);
            assert_eq!(g.ball.vx.signum(), -vx.signum());
            assert!(g.ball.y >= BALL && g.ball.vy > 0.);
        }
    }
    #[test]
    fn paddle_clamp_applies_at_contact_not_end_of_step() {
        let mut g = game();
        g.player = H - PADDLE_H / 2. - 0.5;
        g.ball = Ball {
            x: 54.,
            y: 490.,
            vx: -350.,
            vy: 0.,
        };
        g.step(1. / 120., 1.);
        assert!(g.ball.vx > 0.);
    }
    #[test]
    fn wall_and_score() {
        let mut g = game();
        g.ball = Ball {
            x: 450.,
            y: 9.,
            vx: 300.,
            vy: -200.,
        };
        g.step(0.033, 0.);
        assert!(g.ball.vy > 0.);
        g.scores = [6, 0];
        g.ball = Ball {
            x: 905.,
            y: 20.,
            vx: 350.,
            vy: 0.,
        };
        g.step(0.033, 0.);
        assert_eq!(g.scores, [7, 0]);
        assert_eq!(g.state, State::Over);
        g.start();
        assert_eq!(g.scores, [0, 0]);
    }
    #[test]
    fn nonwinning_scores_reset_ball_and_serve_toward_conceding_side() {
        for (ball_x, velocity, expected_scores, serve_direction) in
            [(905., 350., [1, 0], -1.), (-5., -350., [0, 1], 1.)]
        {
            let mut g = game();
            g.ball = Ball {
                x: ball_x,
                y: 20.,
                vx: velocity,
                vy: 0.,
            };
            g.step(0.033, 0.);
            assert_eq!(g.scores, expected_scores);
            assert_eq!(g.state, State::Playing);
            assert_eq!((g.ball.x, g.ball.y), (W / 2., H / 2.));
            assert_eq!(g.ball.vx.signum(), serve_direction);
            assert!(g.serve > 0.);
            let ball = (g.ball.x, g.ball.y);
            g.step(0.033, 0.);
            assert_eq!((g.ball.x, g.ball.y), ball);
        }
    }
    #[test]
    fn either_winner_freezes_until_restart() {
        for (ball_x, velocity, initial, expected) in
            [(905., 350., [6, 0], [7, 0]), (-5., -350., [0, 6], [0, 7])]
        {
            let mut g = game();
            g.scores = initial;
            g.ball = Ball {
                x: ball_x,
                y: 20.,
                vx: velocity,
                vy: 0.,
            };
            g.step(0.033, 0.);
            assert_eq!(g.scores, expected);
            assert_eq!(g.state, State::Over);
            let (ball, player) = ((g.ball.x, g.ball.y), g.player);
            g.step(0.033, 1.);
            g.toggle_pause();
            assert_eq!(g.state, State::Over);
            assert_eq!((g.ball.x, g.ball.y), ball);
            assert_eq!(g.player, player);
            assert_eq!(g.scores, expected);
            g.start();
            assert_eq!(g.state, State::Playing);
            assert_eq!(g.scores, [0, 0]);
            assert_eq!((g.player, g.cpu), (H / 2., H / 2.));
            assert_eq!((g.ball.x, g.ball.y), (W / 2., H / 2.));
            assert!(g.serve > 0.);
        }
    }
    #[test]
    fn pause_freezes() {
        let mut g = game();
        g.pause();
        let x = g.ball.x;
        g.step(0.033, 1.);
        assert_eq!(g.ball.x, x);
        assert_eq!(g.player, 250.);
        g.toggle_pause();
        assert_eq!(g.state, State::Playing);
    }
    #[test]
    fn cpu_delayed() {
        let mut g = game();
        g.ball = Ball {
            x: 500.,
            y: 400.,
            vx: 350.,
            vy: 0.,
        };
        g.step(0.033, 0.);
        assert_eq!(g.cpu, 250.);
        for _ in 0..10 {
            g.step(0.033, 0.);
        }
        assert!(g.cpu > 250. && g.cpu - 250. <= 190. * 0.363);
    }
    #[test]
    fn cpu_fallible() {
        let mut g = game();
        g.ball = Ball {
            x: 750.,
            y: 50.,
            vx: 700.,
            vy: 0.,
        };
        for _ in 0..20 {
            g.step(0.033, 0.);
        }
        assert_eq!(g.scores[0], 1);
    }
    #[test]
    fn cpu_returns_center() {
        let mut g = game();
        g.ball = Ball {
            x: 650.,
            y: 250.,
            vx: 350.,
            vy: 0.,
        };
        for _ in 0..40 {
            if g.ball.vx < 0. {
                break;
            }
            g.step(0.033, 0.);
        }
        assert!(g.ball.vx < 0.);
    }
    #[test]
    fn deterministic() {
        let mut a = game();
        let mut b = game();
        for _ in 0..200 {
            a.step(0.016, 0.);
            b.step(0.016, 0.);
        }
        assert_eq!(a.ball.x, b.ball.x);
        assert_eq!(a.cpu, b.cpu);
    }
    #[test]
    fn invalid_time_ignored() {
        let mut g = game();
        let x = g.ball.x;
        g.step(f32::NAN, 0.);
        assert_eq!(g.ball.x, x);
        g.step(-1., 0.);
        assert_eq!(g.ball.x, x);
    }
}
