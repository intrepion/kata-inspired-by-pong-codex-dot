# Pong / Blue Engine

Isolated native graphics experiment pinned to `blue_engine = 0.10.0`, using the same dependency-free `pong-core` as the other frontends. Uses the official engine's square meshes, orthographic camera, winit input helper and wgpu renderer; no webview. It is a 2D test of a general-purpose 3D-capable engine.

```sh
cargo run --manifest-path experiments/blue-engine/Cargo.toml
```

Run from the repository root. W/S or arrows move; Enter starts/resumes, Space pauses, R restarts, Escape pauses. Focus loss pauses; Blue Engine's input helper clears held keys on focus loss. The window title displays numeric scores/state/controls, and the court displays score markers without depending on external font files. Resizing fits the court with an orthographic projection. Shared simulation uses fixed 120 Hz steps with catch-up capped at 100 ms.

## Validation (2026-11-11)

Mac cargo check and full build passed. Native court, movement, scoring, pause and restart verified. Absolute resize and non-singular hidden score markers corrected. Windows/Omarchy untested. Upstream block 0.1.6 emitted a future-Rust compatibility warning.

## API references

Implementation based on official registry sources for Blue Engine 0.10.0: `Engine::new_config/update_loop`, `primitive_shapes::square`, `Object::set_scale/set_position/set_color`, `Projection::Orthographic`, and `WinitInputHelper::key_pressed/key_held`.

- https://docs.rs/blue_engine/0.10.0/blue_engine/
- https://github.com/AryanpurTech/BlueEngine

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
