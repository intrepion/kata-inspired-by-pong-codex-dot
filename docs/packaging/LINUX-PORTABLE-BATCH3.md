# Linux portable lifecycle: Blue Engine and softbuffer

Local ARM64 extraction/run/removal pilot, 2026-11-12, from checkpoint 6619d2d. Both archive binaries match their earlier published timing hashes exactly. [Allowlisted manifest](linux-portable-batch3-20261112.json).

Both fresh extracted packages passed checksums, bounded native scripted replay with 13 states through tick 1440, finished=true, and normal exit 0. Softbuffer wrote six application-owned captures; tick 600 inspected with court, score, paddles and ball visible. Blue Engine native screenshots remain unsupported; the archive README explicitly says so. No headless evidence substituted for native rendering.

Exact five-file removal passed with adjacent sentinels preserved; original builds, archives and replay evidence retained. Shared packager now has twelve explicit adapters; 24 Node tests passed, including packaging fixtures for all twelve.

Together with Macroquad, SDL3 and ash, all fifteen native timing adapters now have a local portable lifecycle pilot on Omarchy. This does not qualify every Pong implementation or a public release. Clean-machine/cross-distro testing, full dependency notices, installers/upgrades/signing and OS-input/focus checks remain separate. No desktop control, system settings or driver changes were used.
