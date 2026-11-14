#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
name="${1:?Usage: build-web-framework.sh dioxus|leptos|yew}"
case "$name" in dioxus|leptos|yew);; *) echo 'Unknown frontend' >&2;exit 1;; esac
bindgen="${WASM_BINDGEN:-wasm-bindgen}"
target="${CARGO_TARGET_DIR:-$root/experiments/yew/target}"
args=()
if [[ "$name" == dioxus ]]; then args+=(--no-default-features --features web); fi
cargo build --locked --manifest-path "$root/experiments/$name/Cargo.toml" --target wasm32-unknown-unknown --target-dir "$target" -j 1 "${args[@]}"
mkdir -p "$root/experiments/$name/dist"
"$bindgen" "$target/wasm32-unknown-unknown/debug/pong-${name}.wasm" --target web --out-dir "$root/experiments/$name/dist" --out-name app
cp "$root/clients/web-wasm/favicon.svg" "$root/experiments/$name/dist/favicon.svg"
cat > "$root/experiments/$name/dist/index.html" <<HTML
<!doctype html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><link rel="icon" type="image/svg+xml" href="./favicon.svg"><title>Pong $name</title></head><body><div id="main"></div><script type="module" src="./boot.js"></script></body></html>
HTML
printf "import init from './app.js'; await init();\\n" > "$root/experiments/$name/dist/boot.js"
echo "Built $name. Serve with: python3 -m http.server 5191 --bind 127.0.0.1 --directory $root/experiments/$name/dist"
