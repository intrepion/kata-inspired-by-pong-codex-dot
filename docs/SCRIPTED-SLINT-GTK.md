# Slint and GTK scripted rendering evidence

Both GUI adapters use the existing `pong-scenario::Demo` timeline and core simulation, not duplicated scenarios. Scripted mode ignores real keyboard/focus events, advances fixed simulation ticks from the event-loop timer, and exits after tick 1440. Capture directories must be new. Normal interactive behavior remains unchanged.

```sh
SLINT_BACKEND=winit-software cargo run --locked --manifest-path experiments/gui-slint/Cargo.toml -- --scripted-test --scripted-captures /absolute/new-slint-images
cargo run --locked --manifest-path experiments/gui-gtk/Cargo.toml -- --scripted-test --scripted-captures /absolute/new-gtk-images
```

## What the images establish

| Adapter | Supported application-owned export | Boundary |
| --- | --- | --- |
| Slint | `Window::take_snapshot()` with the software renderer, after updating normal component properties | Renders the app's component tree; not desktop/window-manager capture or proof of compositor presentation. |
| GTK | The ordinary `DrawingArea` callback and export both call the same `draw_scene`; export uses Cairo `ImageSurface::ARgb32` | A separate software rendering of the same app drawing routine at the current drawing-area size. It does not read the actual GTK/compositor framebuffer. |

Slint's renderer snapshot includes its normal AboutSlint component. The existing bottom attribution URL is partially clipped at the tested size; the court, score, controls, ball and paddles are readable. GTK retains its existing spare vertical space below the court. PNG dimensions reflect the configured scale/window allocation, not a promise of fixed dimensions on every machine.

No OS input injection, desktop screenshot service, or other app capture is used. Six stills do not establish smooth animation, input latency, actual keyboard/focus behavior, resize correctness, or compositor presentation. Those remain interactive QA tasks.

## Verified on Omarchy ARM64, 2026-11-12

- Both builds and strict Clippy pass; four shared GUI-model tests pass.
- GTK's new Cairo regression passes: the image has the expected extent/white ball pixels and changes when the player paddle moves.
- Fourteen shared core/scenario tests pass.
- Both bounded native runs exit 0 with six PNGs and thirteen telemetry records, byte-identical to the Macroquad baseline through final tick 1440.
- Paused tick 480 and restarted tick 1440 inspected: score 0:1 Paused and 0:0 Playing, respectively, with ball and both paddles visible.

## Godot pause-label correction

The previous Godot scene placed a central pause overlay over the ball at tick 480. The central message is now limited to Ready and Over; the existing footer still shows Paused. No simulation or controls changed. A fresh native Godot 4.5.1 viewport export run exits 0 with the same thirteen baseline states and six PNGs. Tick 480 inspected: ball unobscured, footer Paused, score 0:1. This verifies the application's scene image, not OS-level interaction.

[Image hashes and dimensions](evidence/scripted-slint-gtk-20261112.json), [Slint telemetry](evidence/omarchy-slint-scripted-20261112.jsonl), [GTK telemetry](evidence/omarchy-gtk-scripted-20261112.jsonl), [Godot rerun telemetry](evidence/omarchy-godot-pause-layout-scripted-20261112.jsonl).
