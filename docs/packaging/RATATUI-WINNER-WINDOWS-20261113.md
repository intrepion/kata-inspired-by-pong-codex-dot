# Ratatui winner header: Windows cross-check

At checkpoint `15bee7c6a62d5e454628cc141ae1779fe23129c9`, Ratatui source Git blob `320f577883caf7fc03357d952df68db4c322850c` was tested with stable MSVC Rust. `cargo test --manifest-path experiments/ratatui/Cargo.toml --locked --offline` passed all 11 tests, including the controlled TestBackend regression for both winner labels at 50 x 18 and clearing after restart. A locked offline debug build succeeded.

The executable SHA-256 was `9ecef99fc1981c0dcc75ed05d3a084318358ccc18356953e04c14e57b1e0080b`. Its `--scripted-test --scripted-cells` run exited 0 with empty stderr, and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six TestBackend cell exports were generated; the inspected tick-480 text header showed 0-1 and Paused (text SHA-256 `ee362c1eb620fddd579050ba158d483fa829117be3437034a59fb71b7312c0a7`). The previous executable, logs and exports remain in sibling workspace directory `ratatui-winner-windows-20261113`.

The winner check is a controlled test-buffer result; the scripted scenario does not reach game over. Cell exports are not screenshots of a real terminal or physical keyboard interaction. Complete-match terminal rendering, terminal compatibility, clean-machine operation, dependency-license clearance and release qualification remain open.
