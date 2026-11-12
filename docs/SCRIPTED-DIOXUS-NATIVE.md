# Dioxus native shared replay and SVG-source evidence

```sh
cargo run --locked --manifest-path experiments/dioxus/Cargo.toml -- --scripted-test --scripted-scenes /absolute/new-dioxus-scenes
```

Native CLI opt-in calls the existing shared Session's `enable_demo()`. A new borrowed `completed_demo_exports()` accessor returns the normal common renderer's reports/SVG sources only after completion; before completion or in normal play it returns None. No scenario or renderer is duplicated. The adapter writes fixed checkpoint filenames plus telemetry with create-new semantics, then exits the bounded test process. Without `--scripted-scenes`, it prints telemetry only. Normal launch has no exporter. `--scripted-captures` is explicitly rejected because these are not screenshots.

This is generated SVG geometry/state evidence, **not proof that the webview presented a frame**. The process may exit before the final DOM update is displayed. No OS input injection, desktop capture, new webview capabilities or screenshot fabrication is used.

## Omarchy verification, 2026-11-12

Locked native build and strict all-target Clippy pass; the wasm32 web-feature regression check also passes. The four common tests pass, including assertions that the accessor is unavailable before completion and returns the existing exact reports/scenes afterward. The native run exits 0 with six SVGs and thirteen byte-identical Macroquad baseline records through tick 1440. XML inspection verifies all six scenes' ball coordinates, player-paddle position and scores against state records. Real-input, focus, compositor and visual QA remain unverified.

## Linux linkage prerequisite

Dioxus's upstream menu dependency requires `libxdo`; the host did not have it installed. This run used only the library files extracted locally from the official Arch Linux ARM xdotool package, with its Arch Linux ARM Build System signature verified. No xdotool executable was extracted/run, no OS input was injected, and no system configuration was changed. The official geo mirror's HTTPS certificate mismatch was not bypassed; an alternate official mirror with valid TLS was used.

Set `LIBRARY_PATH` to the local dependency's `usr/lib` during build and `LD_LIBRARY_PATH` to it during run, or use a normal system-provided libxdo dependency. The local dependency is not bundled into this repository. [Package checksum/signature and artifact hashes](evidence/omarchy-native-dioxus-scripted-20261112.json).

[Telemetry](evidence/omarchy-native-dioxus-scripted-20261112.jsonl)
