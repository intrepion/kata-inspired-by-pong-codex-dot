use std::io::{self, BufRead};
use pong_core::{Game, Ball};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v: Vec<f32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let mut g = Game::new(42); g.start(); g.serve = 0.;
        g.ball = Ball { x:v[0], y:v[1], vx:v[2], vy:v[3] }; g.player=v[4]; g.cpu=v[5];
        g.step(v[6],v[7]);
        println!("{},{},{},{},{},{}", g.ball.x,g.ball.y,g.ball.vx,g.ball.vy,g.player,g.cpu);
    }
}
