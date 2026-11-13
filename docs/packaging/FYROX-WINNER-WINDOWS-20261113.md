# Fyrox winner indication: Windows cross-check

The shared Fyrox change at commit `0641a3569202aeaa5abd53d9634afddb145fd1dd` identifies the winner in the existing header rather than showing only `Over`. This Windows check used `experiments/fyrox/src/main.rs` Git blob `82d2c4c317a96c76e6e396aad624876c543f833c`. An overlapping local implementation was reconciled to the published source before these checks.

On Windows with stable MSVC Rust, `cargo test --manifest-path experiments/fyrox/Cargo.toml --locked --offline` passed all nine tests, including the regression that exercises both winners and restart. A fresh locked offline debug build produced executable SHA-256 `53905984fa0296ad16532ffea29b90788ff247d17cf451a43421885732841670`.

An isolated scripted run exited 0 with empty stderr; all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stdout included OpenGL fragment-shader recompilation performance warnings. Six app-owned UI captures were produced. The tick-480 capture has SHA-256 `e1a6460f3b97856901d31a18b7473dd039ce336382022abc96f3d847ce1fff5e`, matching the visually inspected capture of the paused header and court from the equivalent local build. The standard replay does not reach game over, so the rendered winner message remains unverified. Logs, captures, and prior binaries remain in the sibling workspace directory `fyrox-winner-windows-20261113`.

These are model, scripted replay, and app-owned capture results. They do not establish physical input, a complete interactive match, cross-platform validation, license clearance, clean-machine testing, or release qualification. The unrelated local Pixels draft and its patch remain preserved.
