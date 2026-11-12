use pong_core::Game;
use pong_scenario::{Scenario, REPORT_EVERY, SEED};
use serde::Deserialize;

// Absolute world-unit (or world-unit/second) allowance, never relative to size.
// This is not a promise of bit-identical physics across platform math libraries.
const EPSILON: f64 = 0.001;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    scenario: String,
    seed: u32,
    tick: u32,
    state: String,
    player: f64,
    cpu: f64,
    ball: [f64; 4],
    scores: [u32; 2],
    finished: bool,
}

fn compare(actual: &Snapshot, expected: &Snapshot) -> Result<f64, String> {
    if actual.scenario != expected.scenario
        || actual.seed != expected.seed
        || actual.tick != expected.tick
        || actual.state != expected.state
        || actual.scores != expected.scores
        || actual.finished != expected.finished
    {
        return Err(format!("discrete state mismatch at tick {}", expected.tick));
    }
    let a = [
        actual.player,
        actual.cpu,
        actual.ball[0],
        actual.ball[1],
        actual.ball[2],
        actual.ball[3],
    ];
    let e = [
        expected.player,
        expected.cpu,
        expected.ball[0],
        expected.ball[1],
        expected.ball[2],
        expected.ball[3],
    ];
    let names = ["player", "cpu", "ball.x", "ball.y", "ball.vx", "ball.vy"];
    let mut largest: f64 = 0.0;
    for ((a, e), name) in a.into_iter().zip(e).zip(names) {
        let delta = (a - e).abs();
        if !a.is_finite() || !e.is_finite() || delta > EPSILON {
            return Err(format!(
                "tick {} {name}: actual={a}, expected={e}, delta={delta} (limit {EPSILON})",
                expected.tick
            ));
        }
        largest = largest.max(delta);
    }
    Ok(largest)
}

fn baseline() -> Vec<Snapshot> {
    include_str!("../../../docs/evidence/rust-core-contact-timing-model-20261112.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn portable_manifest_preserves_baseline_states_with_bounded_float_error() {
    let mut game = Game::new(SEED);
    let mut scenario = Scenario::default();
    let mut actual = vec![scenario.report(&game)];
    while !scenario.finished() {
        scenario.advance(&mut game);
        if scenario.tick().is_multiple_of(REPORT_EVERY) {
            actual.push(scenario.report(&game));
        }
    }
    let expected = baseline();
    assert_eq!(actual.len(), expected.len(), "all checkpoints required");
    let mut largest: f64 = 0.0;
    for (line, expected) in actual.iter().zip(&expected) {
        let actual = serde_json::from_str(line).unwrap();
        largest = largest.max(compare(&actual, expected).unwrap());
    }
    eprintln!("maximum absolute baseline delta: {largest}; limit: {EPSILON}");
}

#[test]
fn comparison_rejects_discrete_changes_and_every_material_float_change() {
    let expected = baseline().remove(0);
    for change in 0..6 {
        let mut actual = expected.clone();
        match change {
            0 => actual.scenario.push('x'),
            1 => actual.seed += 1,
            2 => actual.tick += 1,
            3 => actual.state = "Playing".into(),
            4 => actual.scores[0] += 1,
            _ => actual.finished = !actual.finished,
        }
        assert!(compare(&actual, &expected).is_err());
    }
    for field in 0..6 {
        for offset in [-0.002, 0.002, f64::NAN, f64::INFINITY] {
            let mut actual = expected.clone();
            let value = match field {
                0 => &mut actual.player,
                1 => &mut actual.cpu,
                _ => &mut actual.ball[field - 2],
            };
            *value += offset;
            assert!(
                compare(&actual, &expected).is_err(),
                "field {field} offset {offset}"
            );
        }
    }
}

#[test]
fn comparison_accepts_only_small_numeric_variation_and_valid_schema() {
    let expected = baseline().remove(0);
    let mut actual = expected.clone();
    actual.ball[2] += 0.0005;
    assert!(compare(&actual, &expected).is_ok());
    let line = include_str!("../../../docs/evidence/rust-core-contact-timing-model-20261112.jsonl")
        .lines()
        .next()
        .unwrap();
    assert!(serde_json::from_str::<Snapshot>(&line.replace("\"seed\":42,", "")).is_err());
    assert!(serde_json::from_str::<Snapshot>(
        &line.replace("\"seed\":42", "\"seed\":42,\"extra\":1")
    )
    .is_err());
    assert!(
        serde_json::from_str::<Snapshot>(&line.replace("[450,250,0,0]", "[450,250,0]")).is_err()
    );
}
