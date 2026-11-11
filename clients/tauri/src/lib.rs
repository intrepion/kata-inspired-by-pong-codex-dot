#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Same Rust/WASM simulation on desktop and mobile, no privileged IPC.
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to run the Pong Tauri client");
}
