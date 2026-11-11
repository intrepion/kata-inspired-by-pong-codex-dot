# Pong: Godot 4 + Rust GDExtension

An isolated comparison, not a replacement for the original browser game. Godot owns the window, input and 2D drawing; the Rust extension calls the same `pong-core` simulation as the other clients. No gameplay logic is duplicated in GDScript.

## Pinned toolchain

- Godot **4.5.1 stable**, standard build (not .NET).
- `godot` / godot-rust **0.4.5**, `api-4-5`, reduced code generation.
- Commit-local Cargo.lock; no changes to the repository's workspace dependency graph.
- Export templates must match **4.5.1** exactly when installed. Templates are not included.

## Desktop development

From this folder, run `./build.sh`, then `godot --path project` (or use the full path to your Godot executable). The first Cargo build needs `cargo generate-lockfile` only if the lockfile is intentionally regenerated. On Windows: `cargo build --locked -j 1`; create `project/bin`, copy `target/debug/pong_godot.dll` there, and run `godot --path project`.

Rust target and library architecture must match the running engine. The `.gdextension` maps common desktop targets; those mappings do not mean those targets have been built/tested. Keep the default per-experiment target directory to avoid clashing with other workers. Do not point Cargo output into the Godot project.

Controls: W/S or arrows to move; Space/Enter start or pause; P pause; R restart; Escape quit. Losing window focus pauses. Godot's fixed physics tick drives the shared simulation. Viewport stretching keeps the game coordinate system independent of window size.

## Mobile / web scope

Desktop first. Android requires the pinned Godot export templates, Android SDK/NDK/JDK and Rust libraries built per Android ABI. iOS requires matching templates, Apple Rust targets/framework packaging, Xcode and signing for devices. Neither has been built/exported/tested here; desktop mapping does not claim mobile support. Touch controls still need implementation before mobile qualification.

Godot Rust web export is experimental and requires its own Emscripten/nightly pipeline, not the repository's existing `wasm32-unknown-unknown` client. Use that existing raw-WASM client as the reliable browser baseline; do not silently equate it with this Godot experiment. Browser testing must use a localhost server, never file URLs.

## Verification

See `VERIFICATION.md` for actual checks and remaining work. A successful Rust compile does not prove the engine loads the extension, nor does a headless launch prove visual/interactive QA.

## Attribution / licenses

This adapter and `pong-core`: repository MIT license. [Godot Engine](https://godotengine.org/license/) is MIT. [godot-rust/gdext](https://github.com/godot-rust/gdext) bindings are **MPL-2.0**, not MIT. Their license/source obligations remain applicable to distribution; preserve notices and make any modifications to MPL-covered files available under MPL. No vendored binding source is modified here. A packaged export must include required engine and dependency notices; the repository MIT license does not replace them.

References: [compatibility](https://godot-rust.github.io/book/toolchain/compatibility.html), [Android](https://godot-rust.github.io/book/toolchain/export-android.html), [Mac/iOS](https://godot-rust.github.io/book/toolchain/export-mac-and-ios.html), [web](https://godot-rust.github.io/book/toolchain/export-web.html).
