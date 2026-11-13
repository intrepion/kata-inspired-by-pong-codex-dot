# Godot queued actions after focus loss — Omarchy, 2026-11-13

Godot paused on `focus_exited` but its unhandled key handler still accepted queued actions afterward. It now records focus exit and entry, ignores keydowns while unfocused, and still processes key releases to clear movement suppression. Startup preserves the prior assumption of focus until a focus signal arrives; actual platform focus-event delivery has not been verified. Scripted replay continues to bypass live keys.

A new headless scene test failed before the guard when an unfocused Space resumed play. After the fix it emits focus-exit and focus-entry signals, checks unfocused Space, Enter, P and R leave the match paused, checks an unfocused movement release clears suppression, and checks focused Space resumes. Existing headless simulated-input and status smoke scripts also passed. The early guard includes Escape, but a real queued Escape or native OS focus transition was not exercised.

Tests used an isolated project copy and the previously built Rust extension (SHA-256 `abf5d2cb9e9423e5c281cd58a8b63040a4b2f1870f79dadd5883d1c167882340`) with Godot 4.5.1 ARM64 runtime (SHA-256 `7999eb8304045a58adb9f5cbb2be0f988d595b1522a1f644f108f746ef3701f1`); no extension rebuild is claimed. Tested `main.gd` SHA-256 `42a5c76d5b6824d3e8dd82fb6be2b9d5ffa05d430321ed38257eeeb038727af9`, test script SHA-256 `f40268a33bb46a39e0032571d839cdb258dd5a35c4b199191aa5ae11cba0c9a1`; base checkpoint `1d7f2f2`. Isolated project and XDG files remain at `/tmp/pong-godot-focus-20261113-a`.

These are synthetic scene events, not physical input or operating-system focus verification. Other platforms, installation, release readiness and the binding's MPL restriction remain unresolved. Prior artifacts and evidence are preserved.
