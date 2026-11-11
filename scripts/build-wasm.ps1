$ErrorActionPreference = "Stop"
Push-Location "$PSScriptRoot/.."
try {
  cargo build -p pong-wasm --target wasm32-unknown-unknown --release
  if ($LASTEXITCODE -ne 0) { throw "WASM build failed" }
  $target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "target" }
  Copy-Item "$target/wasm32-unknown-unknown/release/pong_wasm.wasm" "clients/web-wasm/pong_wasm.wasm"
} finally { Pop-Location }
