# Ratatui enhanced-key repeat after pause

Ratatui clears movement on pause, but in enhanced-keyboard mode a subsequent `Repeat` event previously set a cleared movement binding again. A focused test failed before the fix. Enhanced-mode repeats now leave the binding unchanged: a press starts it, release stops it, and repeat cannot re-arm a cleared binding. This also applies after reset, focus loss and resize clears. Fallback terminals without release support retain their latched controls; terminals that report repeats as indistinguishable presses cannot be covered by this change.

Ten offline locked release tests pass, including all four movement bindings after clear, release and fresh press, alias independence and existing fallback and renderer checks. A separate-target release build passed. Its TestBackend scripted replay produced 13 records exactly matching `evidence/rust-core-contact-timing-model-20261112.jsonl` and six text/JSON cell pairs; tick 480 contains the paused status. This is adapter/model and app-owned terminal-cell evidence, not a terminal screenshot, native keyboard delivery or physical input. Actual terminal protocols, cross-platform behavior and package qualification remain open.

Tested on Omarchy ARM64 against checkpoint `f680d10`, source Git blob `d93e737fab26b749f7f20dc6898b959a19b63f32`, binary SHA-256 `2e985f886e1480f7bf3bb3016ce10a1cf460ba30c1d36ea93db914d8d4684245`. The isolated build is `/tmp/pong-ratatui-held-build-20261113`, replay log `/tmp/pong-ratatui-held-final-replay-20261113.log`, and cell exports `/tmp/pong-ratatui-held-final-cells-20261113`. Existing binaries, packages and evidence were retained.

## Mac recheck

macOS ARM64 source `e0cd5d23092d37df59bfa4ab352b5236f03d0f22`. Offline locked debug build reused a copy-on-write cache; all ten Ratatui tests passed, including enhanced repeat suppression after pause and reset. Executable SHA-256 `97ca210aa2923ea22327d27686813b16f3f92a15d2c4e0ad7eec5816e417cdae`.

The TestBackend `--scripted-test --scripted-cells` run exited 0 with 13 telemetry records exactly matching the current contact-timing reference and six text/JSON cell pairs; tick 480 text shows Paused. An initial invocation using the unsupported `--scripted-captures` flag exited 101 with the intended instruction to use `--scripted-cells`; the corrected invocation is the passing run. Outputs remain under `/tmp/pong-ratatui-held-mac-*`. This is adapter/model and app-owned cell evidence, not a terminal screenshot, enhanced terminal event delivery or physical input.
