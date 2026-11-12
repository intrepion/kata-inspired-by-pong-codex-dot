# Linux portable lifecycle: six additional adapters

Local Omarchy ARM64 pilot, 2026-11-12. Each archive preserves the exact previously timed binary SHA-256; game source unchanged. Source checkpoint is recorded in the [public manifest](linux-portable-batch2-20261112.json). Archives and raw host diagnostics remain local.

| Adapter | Archive bytes | Relocated replay seconds | States / captures | Exit |
| --- | ---: | ---: | --- | ---: |
| piston | 4052384 | 12.115 | 13 / 6 | 0 |
| ggez | 4985933 | 12.096 | 13 / 6 | 0 |
| nannou | 32371778 | 12.493 | 13 / 6 | 0 |
| bracket-lib | 1749351 | 12.131 | 13 / 6 | 0 |
| fyrox | 19519902 | 12.27 | 13 / 6 | 0 |
| bevy | 29242678 | 12.32 | 13 / 6 | 0 |

All six extracted archives passed checksum validation, final tick 1440/finished checks, and exact five-file uninstall. Original binaries, archives, captures and adjacent sentinel files were preserved. All six app-owned tick-600 captures were inspected: paddles, ball and court visible; renderer-specific layout preserved. Piston has minimal score glyphs and Fyrox small header text in the tall window; no claim of pixel identity or polished UI.

The shared Linux packager now supports ten explicit adapters. Both packaging tests (including all ten fixture adapters) and 22 shared benchmark tests pass: 24/24. Fixture tests alone do not qualify game binaries.

This is local portable extraction/run/removal, not a clean-machine install, cross-distro package, public release, signing, complete dependency-notice bundle or OS-input/focus qualification. Root MIT license is included; third-party notices remain a release gate. No system package manager, driver, desktop permission or service was changed.
