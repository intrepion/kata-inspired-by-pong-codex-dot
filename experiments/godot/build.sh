#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
cargo build --locked -j 1
mkdir -p project/bin
case "$(uname -s)" in
 Darwin) cp target/debug/libpong_godot.dylib project/bin/ ;;
 Linux) cp target/debug/libpong_godot.so project/bin/ ;;
 *) echo 'On Windows, copy target/debug/pong_godot.dll to project/bin';;
esac
