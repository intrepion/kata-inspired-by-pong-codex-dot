//! Minimal WASM ABI: all simulation remains in pong-core; JS only draws and handles input.
use pong_core::*;
use std::cell::RefCell;
thread_local! {static GAME:RefCell<Game>=RefCell::new(Game::new(42));}
#[no_mangle]
pub extern "C" fn init(seed: u32) {
    GAME.with(|g| *g.borrow_mut() = Game::new(seed as u64));
}
#[no_mangle]
pub extern "C" fn start() {
    GAME.with(|g| g.borrow_mut().start());
}
#[no_mangle]
pub extern "C" fn toggle_pause() {
    GAME.with(|g| g.borrow_mut().toggle_pause());
}
#[no_mangle]
pub extern "C" fn pause() {
    GAME.with(|g| g.borrow_mut().pause());
}
#[no_mangle]
pub extern "C" fn step(dt: f32, input: f32) {
    GAME.with(|g| g.borrow_mut().step(dt, input));
}
#[no_mangle]
pub extern "C" fn value(index: u32) -> f32 {
    GAME.with(|g| {
        let g = g.borrow();
        match index {
            0 => g.player,
            1 => g.cpu,
            2 => g.ball.x,
            3 => g.ball.y,
            4 => g.scores[0] as f32,
            5 => g.scores[1] as f32,
            6 => match g.state {
                State::Ready => 0.,
                State::Playing => 1.,
                State::Paused => 2.,
                State::Over => 3.,
            },
            7 => g.serve,
            _ => 0.,
        }
    })
}
