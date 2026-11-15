# Pong / raylib-rs

Rust bindings to the C raylib graphics/game-development library, rather than a pure-Rust engine. Uses the shared deterministic `pong-core` and an isolated workspace pinned to raylib-rs 6.0.0.

```sh
cargo run --manifest-path experiments/raylib-rs/Cargo.toml
```

W/S or arrows move, Enter/R starts or restarts, Space starts/pauses/resumes, Escape exits. Focus loss pauses. Fixed 120 Hz simulation with 100 ms maximum catch-up. Numeric score and status are drawn in the court; resizing preserves its aspect ratio.

## Dependencies and verification

The official crate builds native raylib and requires CMake and a C compiler. On macOS its documented prerequisite is `brew install cmake`. Linux needs the corresponding audio/window development libraries; Windows needs a compatible C toolchain. See https://github.com/raylib-rs/raylib-rs for current platform requirements. raylib-rs/raylib use the zlib license; this frontend follows the repository license.

Mac cargo check, full build and viewport unit test passed. Native court rendering, keyboard movement, scoring, pause and restart verified after raising the window. Windows/Omarchy remain unverified. CMake 4.4.5 was installed from Homebrew core on the Mac to satisfy the documented prerequisite. No browser file URLs are used for testing.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
