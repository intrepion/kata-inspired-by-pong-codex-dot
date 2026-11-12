#!/usr/bin/env bash
# Internal local lifecycle pilot, not a public distribution package.
set -euo pipefail
if [[ $# != 3 ]]; then echo 'Usage: linux-portable.sh RELEASE_BINARY NEW_OUTPUT_DIRECTORY SOURCE_COMMIT' >&2; exit 2; fi
binary=$(realpath "$1")
out=$2
revision=$3
[[ $(uname -s) == Linux && -x "$binary" && ! -e "$out" && $revision =~ ^[0-9a-f]{40}$ ]] || { echo 'Requires Linux, executable input, new output directory and source commit' >&2; exit 2; }
root=$(cd "$(dirname "$0")/../../.." && pwd)
name="pong-sdl3-linux-$(uname -m)"
# mkdir without -p deliberately refuses existing output rather than overwriting.
mkdir "$out"
out=$(realpath "$out")
mkdir "$out/$name"
install -m 755 "$binary" "$out/$name/pong-sdl3"
cp "$root/LICENSE" "$out/$name/LICENSE"
cp "$root/experiments/sdl3/packaging/PORTABLE-README.txt" "$out/$name/README.txt"
{
  printf 'source_commit=%s\n' "$revision"
  printf 'architecture=%s\n' "$(uname -m)"
  echo 'artifact_scope=local SDL3 packaging pilot, not a public release'
  echo 'dependencies=SDL3 compiled statically; platform graphics/audio/runtime libraries still required'
  echo 'licenses=root MIT included; complete dependency notices pending'
} > "$out/$name/BUILD-INFO.txt"
(cd "$out/$name" && sha256sum pong-sdl3 LICENSE README.txt BUILD-INFO.txt > SHA256SUMS)
tar -C "$out" -czf "$out/$name.tar.gz" "$name"
(cd "$out" && sha256sum "$name.tar.gz" > "$name.tar.gz.sha256")
printf '%s\n' "$out/$name.tar.gz"
