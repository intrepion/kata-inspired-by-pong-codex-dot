#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
cargo build -p pong-wasm --target wasm32-unknown-unknown --release
cp "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/pong_wasm.wasm" clients/web-wasm/pong_wasm.wasm
