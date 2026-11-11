# Pong Ratatui

Terminal frontend for the same `pong-core` simulation as the desktop and web clients. No separate gameplay implementation.

```sh
cargo run --manifest-path experiments/ratatui/Cargo.toml --release
```

Run in an interactive terminal (not an IDE output panel). This standalone workspace deliberately keeps its dependency lock and build output separate from other engine experiments.

- Space: start / pause / resume. R: restart. Q, Escape or Ctrl+C: quit.
- Windows console and Unix terminals supporting keyboard event types: hold Up/Down or W/S, release to stop.
- Other terminals: **LATCHED INPUT** is shown. Press Up/W or Down/S to move continuously; **X stops**. This deliberate fallback does not pretend autorepeat is a reliable held-key signal.
- Resize or focus loss pauses and clears input. Space resumes. Focus reporting depends on the terminal emulator.
- Minimum 50 columns × 18 rows. Resize smaller pauses; resize larger then press Space.
- Half-block canvas, letterboxed for approximately 2:1 terminal cells; terminal font cell proportions can vary.
- Fixed 120 Hz simulation, bounded catch-up, roughly 60 Hz redraw. Terminal latency/rendering is not equivalent to a graphical frontend.
- Alternate screen, cursor, raw mode and keyboard flags restored on normal exit, errors and Rust panics. Forced process termination (SIGKILL/power loss) cannot run cleanup; use `reset` if needed.

## Checks

`cargo test --manifest-path experiments/ratatui/Cargo.toml` covers input release/cancellation, fallback stopping, clearing stale keys and rendering at normal/small sizes. Shared-core gameplay tests live in `crates/pong-core`.

This is an experiment, not a published package. See the project comparison notes for other frontends.

## Verified in this pass

- macOS arm64: `cargo test` passed all 4 frontend tests; debug executable built.
- Isolated macOS pseudo-terminal: fallback label, start, moving-frame output, pause, minimum-size resize warning, Q exit (code 0), alternate-screen leave and canonical/echo restoration exercised. ANSI diff rendering omits unchanged characters, so raw output is not a screenshot.
- Release-aware input is unit-tested; a real enhanced-terminal session still needs hands-on validation. Windows and Omarchy builds/gameplay remain unverified for this frontend (not inferred from other clients).
