#!/usr/bin/env bash
# Local validation artifact only; not a public release/distribution package.
set -euo pipefail
if [[ $# != 2 ]]; then echo 'Usage: linux-portable.sh RELEASE_BINARY NEW_OUTPUT_DIRECTORY' >&2; exit 2; fi
binary=$(realpath "$1")
out=$2
[[ $(uname -s) == Linux && -x "$binary" && ! -e "$out" ]] || { echo 'Requires Linux, executable input and a new output directory' >&2; exit 2; }
root=$(cd "$(dirname "$0")/../../.." && pwd)
name="pong-macroquad-linux-$(uname -m)"
mkdir -p "$out/$name"
out=$(realpath "$out")
install -m 755 "$binary" "$out/$name/pong-macroquad"
cp "$root/LICENSE" "$out/$name/LICENSE"
cp "$root/clients/macroquad/packaging/PORTABLE-README.txt" "$out/$name/README.txt"
{
  echo "source_revision=$(git -C "$root" rev-parse HEAD)"
  echo "packaged_utc=$(date -u +%FT%TZ)"
  echo "architecture=$(uname -m)"
  echo 'artifact_scope=local packaging pilot, not release'
  cat /etc/os-release
  echo; ldd "$binary"
} > "$out/$name/BUILD-INFO.txt"
(cd "$out/$name" && sha256sum pong-macroquad LICENSE README.txt BUILD-INFO.txt > SHA256SUMS)
tar -C "$out" -czf "$out/$name.tar.gz" "$name"
(cd "$out" && sha256sum "$name.tar.gz" > "$name.tar.gz.sha256")
printf '%s\n' "$out/$name.tar.gz"
