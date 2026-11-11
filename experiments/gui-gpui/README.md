# Pong gpui

Independent GUI-toolkit comparison using `../../crates/pong-core` (via gui-support), not a separate implementation of Pong rules. No edits to the main workspace are needed.

```sh
cargo run --manifest-path experiments/gui-gpui/Cargo.toml -j 1
```

W/S or Up/Down moves; Enter starts/resumes; Space pauses/resumes; R restarts; Escape pauses. Focus loss clears held input and pauses. Fixed 120 Hz simulation with a 100 ms maximum catch-up frame prevents huge jumps after a stall.

This is experimental. Compilation, native window launch, and interactive gameplay are separate checks. See `../GUI-TOOLKITS.md` for current verification and license notes.

On macOS without Apple’s optional Metal Toolchain, the checked alternative is:

```sh
cargo run --manifest-path experiments/gui-gpui/Cargo.toml --features gpui/runtime_shaders -j 1
```

This compiles shaders at startup; only cargo check is verified so far, not native launch.
