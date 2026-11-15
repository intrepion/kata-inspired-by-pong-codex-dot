# Four-adapter Linux ARM64 portable lifecycle

Verified 2026-11-12 on Omarchy ARM64, from checkout `59e9b1562940c4897cc25ec5d039c41a677d75c5`.
Each packaged executable's SHA-256 exactly matches its previously published
Omarchy timing report. Archives remain local, not committed or released.

| Adapter | Archive bytes | Relocated replay | App-owned captures | Isolated uninstall |
| --- | ---: | --- | --- | --- |
| Miniquad | 444,408 | Exit0, 12.105s, 13states/tick1440 | Six; tick600 inspected | Pass |
| pixels | 4,005,811 | Exit0, 12.081s, 13states/tick1440 | Six; tick600 inspected | Pass |
| raw wgpu | 3,978,179 | Exit0, 12.079s, 13states/tick1440 | Six; tick600 inspected | Pass |
| raylib-rs | 409,975 | Exit0, 12.862s, 13states/tick1440 | Six; tick600 inspected | Pass |

[Allowlisted results and hashes](linux-portable-batch-20261112.json) omit local
paths and raw diagnostics. All final states have finished=true. Captures show
the respective renderer's court/paddles/ball, not a desktop screenshot or proof
of native keyboard handling. pixels captures its 900×500 CPU source buffer,
not the final window presentation; visual appearance is not identical across
adapters and is not claimed to be.

All archives and five-file extracted stages had SHA-256 checks validated.
Binaries were launched from /tmp rather than their checkout. Removal checked
an exact expected filename allowlist, removed only those files and the empty
stage, and verified neighboring sentinels and original binaries remained.
The first Miniquad post-removal sentinel check used an unnormalized path through
the removed folder; that check was corrected to the parent directory and all
preservation assertions passed. No application failure occurred.

## Reproduce

`node scripts/packaging/linux-portable.mjs ADAPTER RELEASE_BINARY NEW_OUTPUT_DIRECTORY SOURCE_COMMIT`

The shared utility is outside pong-core, explicitly allows only miniquad,
pixels, wgpu and raylib, rejects existing output and invalid metadata/input,
and creates checksums with an embedded provenance warning. The declared commit
alone is not proof for arbitrary input bytes. The two packaging tests exercise
all four adapters with /bin/true fixtures, extraction/hash verification,
overwrite protection and invalid input; these are not gameplay results.
Combined with shared benchmark checks, all24Node tests pass on Linux.

This is same-machine local extraction/run/removal, not an OS installer, public
release or cross-distro guarantee. Graphics/display/runtime libraries are not
bundled or installed. Full third-party notices, upgrade/rollback, signing,
clean-machine compatibility and real-input/focus checks remain separate gates.
No root access, Docker permission change, service, desktop entry or PATH change
was made. Existing Macroquad/SDL3/ash package evidence remains separate.
