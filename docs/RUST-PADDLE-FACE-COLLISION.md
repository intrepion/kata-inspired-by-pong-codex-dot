# Rust paddle face collision finding

Omarchy model test, 2026-11-12: a ball beginning at `(60, 180)` with velocity `(-350, 600)` steps for `0.033` seconds against the centered player paddle. At the paddle face (`x = 52`), the ball's center is approximately `y = 193.7`, outside the paddle's collision range (`197..303`). The previous simulation checked the end-of-step height (`199.8`) instead and incorrectly reversed its horizontal velocity.

The regression test `paddle_collision_uses_height_at_face_crossing_on_both_sides` now passes without being ignored.

An initial candidate using the crossing height passed the core regression but changed the portable replay baseline: at tick 600, ball x was 765.1032 versus 765.09827 expected, exceeding the 0.001 tolerance. That initial candidate was not included in the earlier checkpoint.

## Correction and replay review

The shared Rust simulation now evaluates collision height at the face crossing, reflecting a prior wall bounce within that step. Model tests cover hit and miss cases on both sides and a wall bounce before contact. This changes the impact angle and ball position, so it intentionally changes subsequent trajectories.

On Omarchy, all 13 fixed-seed model checkpoints retained the same state, score, and finished flag as the historical app replay. Ball fields differ at ticks 600, 720, 840, 1080, 1200, and 1440; the largest absolute ball-field difference was approximately 1.2222 world units at tick 1200. The updated portable reference is [model-generated telemetry](evidence/rust-core-face-crossing-model-20261112.jsonl), not an application capture. The [historical app-owned telemetry](evidence/omarchy-macroquad-scripted-20261112.jsonl) remains unchanged. No real browser or native input was exercised by this check.

Mac independently reviewed the collision patch and baseline at source `2fbf148`: 13 core, 8 scenario and 1 WASM model tests passed, as did WASM compilation. Windows independently tested published checkpoint `9cee3c3`: 13 core, 8 scenario, and 1 WASM model test passed, with zero portable replay delta against the revised reference. These checks do not qualify frontend behavior. Rebuild affected clients and repeat app replay and interaction qualification; earlier app captures and interaction evidence remain historical.

Two precision limits remain for future investigation: the paddle position is advanced for the full step before contact is evaluated, so moving-paddle contact timing has not been tested; and the remaining portion of the step after paddle impact is discarded. Neither has been changed in this revision.
