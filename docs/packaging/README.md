# Packaging pilot: Macroquad / Linux ARM64

Verified 2026-11-12 06:22 UTC on Omarchy (`alarm`), Arch Linux ARM rolling,
`aarch64`. Application source: `1da8094a80559ad7ab78982955d8f9d3d196a9ad`.
This is **one local portable-archive pilot**, not an installer release or a
cross-platform/cross-distro certification. No public release was published.

| Check | Result |
|---|---|
| Offline locked release build | PASS, existing Rust toolchain/cache |
| Portable tar.gz creation | PASS, 651,890 bytes, five packaged files |
| Archive and extracted SHA-256 validation | PASS |
| Staged installation in a fresh task-owned directory | PASS, extraction only; no root/system changes |
| Launch outside checkout (working directory `/tmp`) | PASS with permitted display access |
| Installed deterministic gameplay replay | PASS, exit 0; 13 telemetry rows through tick 1440, `finished:true` |
| Isolated uninstall | PASS, exact package files removed; `rmdir` succeeded; neighboring sentinel and original build binary preserved |
| Existing-output overwrite protection | PASS, second packaging call rejected existing directory |
| Shell syntax | PASS, `bash -n` |
| Clean machine / different distro / x86_64 | NOT RUN |
| OS package manager integration, upgrade, signing | NOT RUN |
| Dependency-license notice inventory | PENDING; archive is not distribution-ready |

Archive SHA-256:
`a5445368b9d36db18a663b7d5f6904a8aa776b10ef2cf6aac79a4d714156716c`.
The archive remains local; binaries are deliberately not committed.

## Reproduce

From the repository root, build `cargo build --offline --locked --release -p
pong-macroquad`. Run
`clients/macroquad/packaging/linux-portable.sh target/release/pong-macroquad NEW_OUTPUT_DIRECTORY`.
The input binary must be the release build from the stated source revision;
the packager records the checkout revision but does not prove arbitrary input
binary provenance. Output directories must not already exist.

Verify the archive checksum, extract to a new staging directory, verify the
included `SHA256SUMS`, and launch its binary with `--scripted-test` from another
working directory. The deterministic scenario and normal assets are embedded.
After exit, remove only `pong-macroquad`, `LICENSE`, `README.txt`,
`BUILD-INFO.txt`, and `SHA256SUMS` from that isolated package directory, then
`rmdir` it. Preserve files outside it. Do not run recursive removal on a user
installation without inspecting its contents.

## Evidence and limitations

- [Installed replay](macroquad-linux-installed-replay.jsonl)
- [Build OS and direct dynamic links](macroquad-linux-build-info.txt)
- [Initial sandbox launch failure](macroquad-linux-sandbox-attempt.stderr)

The first launch failed `XOpenDisplay()` under the command sandbox. A separate,
permitted escalated invocation in the existing display session completed. This
is a display-access distinction, not evidence that installing a package fixes
it. We retained the failed attempt rather than silently dropping it.

This run verifies installation relocation and bounded scripted behavior, not
physical keyboard input, screenshot appearance, frame pacing or a full match.
It creates no desktop entry, service, global PATH entry, uninstall registry entry
or system dependency installation. The executable directly links glibc, libm
and libgcc_s; runtime-loaded graphics/display libraries also remain host
requirements. `ldd` is not a complete runtime-dependency inventory. No ABI
compatibility with Ubuntu, Debian, Fedora or other machines is established.

Root project MIT license is included, but complete transitive dependency
notices are a release gate. Timestamped build metadata means this pilot does not
claim byte-for-byte reproducible archives. Keep packaging alongside Macroquad;
shared game logic does not own installer lifecycle code.
