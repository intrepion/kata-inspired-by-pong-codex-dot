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

// Browser adapters own scheduling and canvas export, never the scenario timeline.
thread_local! {
    static SCENARIO: RefCell<pong_scenario::Scenario> = RefCell::default();
    static REPORT: RefCell<String> = RefCell::new(String::new());
}
#[no_mangle]
pub extern "C" fn demo_init() {
    init(pong_scenario::SEED as u32);
    SCENARIO.with(|s| *s.borrow_mut() = Default::default());
}
#[no_mangle]
pub extern "C" fn demo_step() {
    SCENARIO.with(|s| GAME.with(|g| s.borrow_mut().advance(&mut g.borrow_mut())));
}
#[no_mangle]
pub extern "C" fn demo_tick() -> u32 {
    SCENARIO.with(|s| s.borrow().tick())
}
#[no_mangle]
pub extern "C" fn demo_finished() -> bool {
    SCENARIO.with(|s| s.borrow().finished())
}
#[no_mangle]
pub extern "C" fn demo_capture_due() -> bool {
    pong_scenario::CAPTURE_TICKS.contains(&demo_tick())
}
/// Pointer valid until the next demo_report call; copy immediately in JS.
#[no_mangle]
pub extern "C" fn demo_report() -> *const u8 {
    REPORT.with(|r| {
        *r.borrow_mut() = SCENARIO.with(|s| GAME.with(|g| s.borrow().report(&g.borrow())));
        r.borrow().as_ptr()
    })
}
#[no_mangle]
pub extern "C" fn demo_report_len() -> usize {
    REPORT.with(|r| r.borrow().len())
}

#[cfg(test)]
mod demo_tests {
    use super::*;
    #[test]
    fn exported_replay_matches_shared_scenario() {
        let mut expected = Game::new(pong_scenario::SEED);
        let mut scenario = pong_scenario::Scenario::default();
        demo_init();
        let mut captures = Vec::new();
        while !demo_finished() {
            demo_step();
            scenario.advance(&mut expected);
            assert_eq!(demo_tick(), scenario.tick());
            if demo_capture_due() {
                captures.push(demo_tick());
            }
        }
        assert_eq!(captures, pong_scenario::CAPTURE_TICKS);
        demo_report();
        REPORT.with(|r| assert_eq!(*r.borrow(), scenario.report(&expected)));
        demo_step();
        assert_eq!(demo_tick(), pong_scenario::END_TICK);
        demo_init();
        assert_eq!(demo_tick(), 0);
    }
}
