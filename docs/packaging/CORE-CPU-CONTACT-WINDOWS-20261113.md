# Shared core CPU contact timing: Windows

At checkpoint `bd811158d48afc28ce357f79c88b0e0764f4d0cb`, a new core unit regression checks CPU target arrival before and after a ball crosses the paddle face. It uses the same two controlled situations as the original browser model tests: a tangent return after reaching the target and a miss when the paddle has not yet arrived. No production physics changed.

On Windows stable MSVC Rust, `cargo test --locked --offline -p pong-core` passed all 20 tests. This is a model regression, not frontend rendering, physical input, complete-match behavior or release qualification. A TypeScript compiler was not found in the existing global Node installation, known editor paths, workspace checkouts or npm execution cache, so the original browser TypeScript build remains unverified here; no compiler was installed.

Follow-up: the after-contact target was corrected to be reached later in the same step, and both cases now assert target arrival by the end of the step. All 20 core tests passed again; no production physics changed.
