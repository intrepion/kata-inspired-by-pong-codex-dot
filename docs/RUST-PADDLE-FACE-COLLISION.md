# Rust paddle face collision finding

Omarchy model test, 2026-11-12: a ball beginning at `(60, 180)` with velocity `(-350, 600)` steps for `0.033` seconds against the centered player paddle. At the paddle face (`x = 52`), the ball's center is approximately `y = 193.7`, outside the paddle's collision range (`197..303`). The previous simulation checked the end-of-step height (`199.8`) instead and incorrectly reversed its horizontal velocity.

The regression test `paddle_collision_uses_height_at_face_crossing_on_both_sides` now passes without being ignored.

An initial candidate using the crossing height passed the core regression but changed the portable replay baseline: at tick 600, ball x was 765.1032 versus 765.09827 expected, exceeding the 0.001 tolerance. That initial candidate was not included in the earlier checkpoint.

## Correction and replay review

The shared Rust simulation now evaluates collision height at the face crossing, reflecting a prior wall bounce within that step. Model tests cover hit and miss cases on both sides and a wall bounce before contact. This changes the impact angle and ball position, so it intentionally changes subsequent trajectories.

On Omarchy, all 13 fixed-seed model checkpoints retained the same state, score, and finished flag as the historical app replay. Ball fields differ at ticks 600, 720, 840, 1080, 1200, and 1440; the largest absolute ball-field difference was approximately 1.2222 world units at tick 1200. The updated portable reference is [model-generated telemetry](evidence/rust-core-face-crossing-model-20261112.jsonl), not an application capture. The [historical app-owned telemetry](evidence/omarchy-macroquad-scripted-20261112.jsonl) remains unchanged. No real browser or native input was exercised by this check.

Mac independently reviewed the collision patch and baseline at source `2fbf148`: 13 core, 8 scenario and 1 WASM model tests passed, as did WASM compilation. Windows independently tested published checkpoint `9cee3c3`: 13 core, 8 scenario, and 1 WASM model test passed, with zero portable replay delta against the revised reference. These checks do not qualify frontend behavior. Rebuild affected clients and repeat app replay and interaction qualification; earlier app captures and interaction evidence remain historical.

Two precision limits remain for future investigation: the paddle position is advanced for the full step before contact is evaluated, so moving-paddle contact timing has not been tested; and the remaining portion of the step after paddle impact is discarded. Neither has been changed in this revision.

## Further timing investigation on Omarchy

Two ignored model regressions reproduce at the normal 1/120-second step; run `cargo test --manifest-path crates/pong-core/Cargo.toml -- --ignored` to see the expected failures. They are not counted as passing tests:

- With the player starting at y=250 and moving down at 470 units/s, a ball at (54, 306.3) moving left at 350 reaches the x=52 face after about 0.005714 seconds. The paddle center then is about 252.686, so the 53-unit collision threshold is missed; the current end-of-step center is about 253.917 and incorrectly produces a hit.
- A centered ball from x=54 reaches the face after the same time. A center hit leaves about 0.002619 seconds in the step and, with the current rebound speed 369.25, would move about 0.967 units to x=52.967. The current code stops at x=52, discarding that remainder. This is a model integration limitation; its visible impact has not been measured.

No further physics or portable reference changes are made here. Coordinate the expected collision semantics and replay consequences across platforms before changing them.

## Fresh Omarchy Ratatui replay

A fresh offline optimized Ratatui build using the published `9cee3c3` physics completed on Omarchy ARM64. Its bounded `--scripted-test --scripted-cells` run exited 0 and produced 13 telemetry records exactly matching the revised model reference, plus six app-owned cell-buffer JSON/text pairs. Binary SHA-256: `9de92c5f52c0817edecaa49071dd76f7c12127b8a702167418e9ca25ee55e36a`. Cell data and telemetry were retained locally under `/tmp/pong-ratatui-collision-replay-20261112` and `/tmp/pong-ratatui-collision-telemetry-20261112.jsonl`. This test backend is not a physical terminal, screenshot, or native-input interaction.
