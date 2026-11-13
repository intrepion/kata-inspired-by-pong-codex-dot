# Windows shared GUI non-playing input recheck

At checkpoint `b810af19e336a873b1825e46066e8c29273a23e3`, Windows MSVC stable Rust 1.99.1 ran `cargo test --manifest-path experiments/gui-support/Cargo.toml --locked --offline`: all 11 shared-model tests and the zero doctests passed. The tested support source Git blob is `68032c026c5fcdf2c23a96540f2f78d72f1fb965`. The isolated build target remains at `target-gui-nonplaying-windows-20261113`.

The new test checks movement presses outside play and a fresh press after release. This is model-level evidence shared by GTK, Slint and GPUI, not evidence of native keyboard delivery or physical input. No frontend was rebuilt or replayed in this check; accessibility, clean-machine operation, dependency and license review, and release readiness remain open.
