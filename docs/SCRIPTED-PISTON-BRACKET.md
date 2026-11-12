# Piston and bracket-lib scripted runs

Both use the common pong-scenario Demo, not a copied timeline:

```sh
cargo run --manifest-path experiments/piston/Cargo.toml -- --scripted-test --scripted-captures ./new-piston-captures
cargo run --manifest-path experiments/bracket-lib/Cargo.toml -- --scripted-test --scripted-captures ./new-bracket-captures
```

Use a NEW directory. Neither path reads other applications or injects OS input.

- **Piston 0.148:** its public draw_2d hides the surface texture and presents internally. Capture therefore invokes the exact same draw_scene against a separate app-owned wgpu render target, then copies this texture to a padded readback buffer. This proves renderer/state output, not literal swapchain/compositor presentation. RGBA8/BGRA8 only; unsupported formats fail explicitly. GPU waits are bounded. Window title status is not in the PNG.
- **bracket-lib:** native BTerm::screenshot requests the framework's own framebuffer export after drawing. The following tick verifies the completed PNG before progressing/exiting. Uses the existing process-local XWayland fallback; no desktop configuration changed. Web/terminal-backend capture is not claimed.

## Verified 2026-11-12, Omarchy ARM64

Both final native runs exit 0 at tick 1440, produce all six PNGs, and have 13 state records identical to the common baseline. Pause/reset images inspected: Piston's score bar resets and ball/paddles are framed; bracket-lib's header reads 0:1 Paused then 0:0 Playing, with legible retro text and visible ball/paddles. Piston's tall tiled window retains large letterboxing. These are sampled image checks, not real-input or smooth-motion passes.

Build and strict Clippy passed for both. Six bracket-lib regression tests and fourteen shared core/scenario tests passed. Existing vendored freetype warnings remain outside the strict application lint target.

Piston exposed a common driver defect: multiple update callbacks could advance beyond a capture checkpoint before the next render callback, losing two of six PNGs. The shared Demo now holds unconsumed capture ticks across repeated updates without accumulating an unbounded backlog. A public-interface regression test failed before the fix and passes after it. Both adapters were rebuilt and rerun after this correction.

Evidence: [hashes and dimensions](evidence/scripted-piston-bracket-20261112.json), [Piston telemetry](evidence/omarchy-piston-scripted-20261112.jsonl), [bracket telemetry](evidence/omarchy-bracket-scripted-20261112.jsonl).
