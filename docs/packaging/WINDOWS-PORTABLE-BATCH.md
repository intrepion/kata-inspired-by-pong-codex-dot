# Windows portable ZIP batch: pixels, raw wgpu, softbuffer

2026-11-12. Local unsigned packaging only; not public distribution or an installer qualification.

| Adapter | Extracted replay seconds | States / PNGs | Manifest |
| --- | ---: | --- | --- |
| pixels | 13.078 | 13 / 6 | [Hashes](pixels-windows-manifest-20261112.json) |
| raw wgpu | 13.143 | 13 / 6 | [Hashes](wgpu-windows-manifest-20261112.json) |
| softbuffer | 12.310 | 13 / 6 | [Hashes](softbuffer-windows-manifest-20261112.json) |

For each existing timing-pilot release executable, created a new staging folder containing executable, project LICENSE and explicit incomplete-pilot notice; compressed with PowerShell Compress-Archive, extracted into a new path containing spaces with Expand-Archive, and verified extracted executable SHA-256 equals source. Both destinations had to be absent. Ran from the extracted working directory, all exits0 with finished tick1440. Source/build/staging files retained. No PATH, registry, security or system-install changes.

Inspected tick600 in each app-owned capture: both paddles, ball, divider and score indicators present. pixels and raw wgpu retain minimal tally-style scores rather than a text HUD; softbuffer displays numeric score. Captures do not qualify native keyboard/focus/full-match interaction or every visual state.

Pending: full third-party notices, clean-machine dependencies, signing/download security handling, installer/upgrade/uninstall, and actual native input gates. No security warnings bypassed; no archives publicly distributed. Timing reports retain their original provenance and these extracted runs do not add benchmark claims.
