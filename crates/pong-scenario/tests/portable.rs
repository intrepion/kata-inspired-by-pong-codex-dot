use pong_core::Game;
use pong_scenario::{Scenario, REPORT_EVERY, SEED};
#[test]
fn portable_manifest_preserves_all_existing_rust_baseline_states() {
    let mut game = Game::new(SEED);
    let mut scenario = Scenario::default();
    let mut actual = vec![scenario.report(&game)];
    while !scenario.finished() {
        scenario.advance(&mut game);
        if scenario.tick().is_multiple_of(REPORT_EVERY) { actual.push(scenario.report(&game)); }
    }
    let expected: Vec<_> = include_str!("../../../docs/evidence/omarchy-macroquad-scripted-20261112.jsonl").lines().collect();
    assert_eq!(actual, expected);
}
