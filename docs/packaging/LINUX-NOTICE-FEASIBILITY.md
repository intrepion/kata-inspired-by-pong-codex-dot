# Linux dependency-notice preparation

2026-11-12, offline locked metadata scan for twelve additional native adapters after Miniquad. [Detailed gaps and digests](linux-notice-feasibility-20261112.json). All collection outputs remain local except the published Miniquad bundle.

| Adapter | Resolved packages | Notice files found | Packages without a discovered notice file |
| --- | ---: | ---: | ---: |
| ash | 95 | 173 | 0 |
| sdl3 | 34 | 113 | 0 |
| softbuffer | 105 | 192 | 1 |
| raylib-rs | 50 | 100 | 1 |
| pixels | 147 | 264 | 6 |
| wgpu | 143 | 259 | 5 |
| piston | 237 | 407 | 8 |
| ggez | 195 | 342 | 12 |
| nannou | 447 | 761 | 26 |
| bracket-lib | 126 | 207 | 7 |
| fyrox | 468 | 752 | 32 |
| blue-engine | 226 | 389 | 12 |

Ash and SDL3 have at least one discovered notice file for each resolved package, so their collected texts were used for the next local archive pilots. Missing-file lists on the other ten graphs are preserved, not filled using guessed generic license text or silently marked complete. The packager rejects those incomplete bundles.

The raylib-rs missing-file gap has [exact-version source-header evidence and an audited collection pilot](RAYLIB-RS-NOTICE-INVESTIGATION.md). Its filtered ARM64 graph now collects the verified headers; other-platform gaps and broader review remain.

The softbuffer gap has a focused [exact-version investigation](SOFTBUFFER-DRM-FOURCC-NOTICE-INVESTIGATION.md); its notice remains unresolved.

A missing file is not proof of a licensing violation. Some published crates omit a repository-root license, embed attribution in source comments, or use workspace-level notices. Exact-version upstream/source review is still needed. Conversely, finding a file does not prove all obligations are satisfied. Native bundled code, source-comment notices, platform libraries and any required source distribution remain release-review gates. No public release or legal clearance is claimed.

## Ash and SDL3 notice-bearing archive verification

Both new local archives use the exact previously qualified timing binaries. Ash includes collected texts for 95 packages / 173 files; SDL3 for 34 packages / 113 files. Full notice text/index bundles remain local; the detailed feasibility report includes bundle digests.

Both archives passed all six checksum entries after fresh extraction, relocated app-owned 13-state replay through tick 1440, normal exit 0, six captures and tick 600 inspection. Exact seven-file uninstall preserved neighbors/original builds. [Archive hashes and results](ash-sdl3-linux-notice-packages-20261112.json). The shared Linux packager now supports fourteen explicit adapters. All 28 shared Node tests pass.

These are verified local archive mechanics with collected notices, not completed legal/native-source review or released installers.
