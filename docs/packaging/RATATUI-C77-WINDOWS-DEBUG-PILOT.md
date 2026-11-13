# Windows Ratatui current-physics portable debug pilot

On 2026-11-13, the Windows Ratatui debug executable built from `c8caa9b8c17b296b1401da88f91786db364b3d56` and tested in the [contact-timing recheck](../WINDOWS-RATATUI-CONTACT-TIMING.md) was staged with the project LICENSE and an incomplete-pilot warning. A new local unsigned ZIP was created and extracted to a new directory containing a space. The three extracted files each matched their staged SHA-256 hashes. No installer or system changes were made.

ZIP size: 395,268 bytes; SHA-256: `36845ce3cb1fab671bd6ad52532544fd925ec9bb4f8243586704e0acaf66454c`. The staged and extracted executable SHA-256: `c2a6a19a4df58e49914b715c6061ff42fbac2f83d5f0532c82f1d3d2c01f527b`. This was a debug build, not a release build or reproducible-archive claim.

The extracted executable ran from its own directory with `--scripted-test --scripted-cells`, exiting 0 with empty stderr. All 13 telemetry records matched the [reference](../evidence/rust-core-contact-timing-model-20261112.jsonl) through finished tick 1440. Six app-owned TestBackend JSON/text cell pairs were generated; the tick 480 text contained the paused status, score 0:1 and latched-input instructions. These cells are not terminal screenshots or real input. ZIP, staging, extraction and output remain in the local `c77-ratatui-package-windows-20261113` directory; previous artifacts remain untouched.

This is a local relocation check only. Real terminal rendering and interaction, physical input, focus/resize and terminal restoration, clean-machine dependencies, full third-party notices, signing, installation, upgrades, uninstall and distribution remain unverified. The earlier Windows Terminal access limitation was not retested.
