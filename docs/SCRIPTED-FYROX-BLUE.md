# Fyrox and Blue Engine scripted render tests

Both integrate the shared pong-scenario Demo, ignoring real input during scripted playback.

## Fyrox

```sh
cargo run --manifest-path experiments/fyrox/Cargo.toml -- --scripted-test --scripted-captures ./new-fyrox-images
```

The plugin uses the public before_rendering hook, Renderer::render_ui and a TextureResource render target. Public framebuffer read_pixels exports only that UI target. PNG rows are flipped from OpenGL origin. This is the actual framework UI renderer and scene state, but a separate target, not a desktop or literal compositor screenshot. The framework loop exits after the final capture. No generic capture fallback.

## Blue Engine

Default builds remain windowed. Windowed scripted replay is available, but windowed PNG capture is explicitly rejected because 0.10.0 does not expose a supported window-frame export hook.

The supported headless feature instead exposes the renderer's own headless_texture_data:

```sh
cargo run --manifest-path experiments/blue-engine/Cargo.toml --no-default-features --features capture-headless -- --scripted-test --scripted-captures ./new-blue-images
```

Use a NEW directory. Do not combine windowed and capture-headless features. The same objects, transforms, camera and shared scenario are used; the offscreen target is 1024x640 so the framework's unpadded readback row size meets wgpu's 256-byte alignment requirement. RGBA8/BGRA8 handling is explicit. The framework renders before uploading changed objects, so capture holds the checkpoint through another render cycle to avoid stale transforms. Only the completed own-buffer data is encoded. The framework's headless loop has no return/exit hook; this bounded demo exits the process after the last completed frame. Its GPU map wait is internally unbounded, so use an external timeout in automated jobs.

This is **offscreen GPU render evidence**, not proof of window presentation or native input.

## Verified on Omarchy ARM64, 2026-11-12

- Fyrox and Blue headless: exit 0, final tick 1440, thirteen state records byte-identical to the common baseline, six PNGs each.
- Inspected pause/reset samples. Fyrox shows 0:1 Paused then 0:0 Playing with ball/paddles; small header text and large letterboxing remain existing layout limitations. Blue shows the expected score pip followed by cleared score, ball and both paddles at reset.
- Both capture configurations and Blue's normal windowed configuration build and pass strict Clippy. Fourteen shared core/scenario tests pass.
- Real keyboard, focus, compositor output and smooth animation are not tested by these exports.

[Hashes/dimensions](evidence/scripted-fyrox-blue-20261112.json), [Fyrox telemetry](evidence/omarchy-fyrox-scripted-20261112.jsonl), [Blue telemetry](evidence/omarchy-blue-scripted-20261112.jsonl).
