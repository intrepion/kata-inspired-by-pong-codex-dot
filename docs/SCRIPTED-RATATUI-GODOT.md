# Ratatui and Godot scripted evidence

## Ratatui: cell-buffer evidence, not images

```sh
cargo run --manifest-path experiments/ratatui/Cargo.toml -- --scripted-test --scripted-cells ./new-cell-evidence
```

This uses the real draw() routine and Ratatui's supported TestBackend at 100x30 cells. The shared Demo advances deterministically without wall-clock pacing or terminal raw mode. Each of six checkpoints exports .txt and .json; JSON preserves each cell's symbol, foreground/background color and modifiers. It is **not a PNG, terminal screenshot, or OS-input test**. --scripted-captures is rejected rather than producing fake screenshots. The output directory must be new.

Five adapter tests pass, including a new rendered-cell regression checking pause/score text and cyan paddle cell styles. Build and strict Clippy pass. The complete run exits 0 and produces 13 baseline-identical telemetry records plus twelve artifacts. Paused text reads YOU 0 : 1 CPU Paused; final text reads YOU 0 : 0 CPU Playing. This does not verify actual terminal font, glyph/color presentation, focus or keyboard protocols.

## Godot: actual scene viewport image export

Build the Rust extension, copy the platform library into project/bin per the adapter README, and use official Godot 4.5.1:

```sh
godot --path experiments/godot/project -- --scripted-test --scripted-captures /absolute/new-godot-images
```

The Rust extension owns the same shared Demo. GDScript consumes its checkpoint path, redraws the ordinary scene, waits for RenderingServer.frame_post_draw, then calls get_viewport().get_texture().get_image().save_png(). Physics is held while awaiting that frame so state cannot drift. Scripted mode ignores interactive input/focus, and quits after final export. No desktop capture or OS input injection. PNG export requires a graphics renderer; --headless capture fails explicitly.

On fresh caches, the runtime must discover pong.gdextension before the scene can use PongSimulation. This run used a local .godot/extension_list.cfg containing res://pong.gdextension, avoiding the previously documented editor shutdown crash. No editor workflow pass is claimed.

Omarchy ARM64 verified with official portable 4.5.1.stable.official.f62fdbde1. Rust build and strict Clippy pass. Actual native scene run exits 0 with six PNGs and 13 baseline-identical telemetry records. Pause/reset images inspected: 0:1 paused and 0:0 playing, both paddles and ball visible. The subsequent [GUI batch](SCRIPTED-SLINT-GTK.md) removes the central pause overlay while retaining the footer status; a fresh native viewport export confirms the paused ball is unobscured. Normal headless smoke.gd integration assertions also pass (start/move/pause/resume/reset). Fourteen shared tests pass.

[Evidence hashes](evidence/scripted-ratatui-godot-20261112.json), [Ratatui state](evidence/omarchy-ratatui-scripted-20261112.jsonl), [Godot state](evidence/omarchy-godot-scripted-20261112.jsonl).
