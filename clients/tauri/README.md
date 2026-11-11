# Tauri comparison client

This native shell embeds the same Rust/WASM game and Canvas renderer as `clients/web-wasm`. It is deliberately a webview comparison, not a second game simulation. Macroquad and Bevy are native game renderers; Tauri uses the operating system's webview.

Build the WASM frontend first using the root instructions, then run from the repository root:

```
cargo run -p pong-tauri
```

The default `custom-protocol` feature embeds the frontend, so this command needs no server and opens a native application window. Browser testing of the frontend itself must use the local HTTP server, never `file://`.

Development with a separately running web server uses `cargo run -p pong-tauri --no-default-features`. The dev URL is `http://127.0.0.1:5188`. No Tauri CLI is needed for either command. Distributable signed installers are not configured.

Prerequisites: a compatible stable Rust toolchain, macOS Xcode command-line tools; on Windows MSVC C++ build tools and WebView2; on Linux the official Tauri 2 prerequisites, including WebKitGTK 4.1 and GTK 3 development packages. Package names differ by distribution. The app grants no Tauri command capabilities and uses a restrictive local-content CSP with WebAssembly compilation enabled.
