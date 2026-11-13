# Blue Engine held movement across pause

On Omarchy ARM64, the Blue Engine adapter polled held movement after an explicit pause, so a key held through resume could immediately move the paddle. A focused movement-gate test failed before the fix. The windowed input loop now suppresses each held movement binding when Playing becomes Paused, until polling observes its release. It also applies the gate when focus loss pauses play. Fresh aliases and opposing bindings remain independent. A release and re-press entirely between polls is not observable by this polled gate.

Four offline locked release tests pass: two benchmark argument tests and two gate tests for all four bindings, release and fresh press, independent aliases and opposing keys. The windowed release build passed. Its separate host-display scripted replay exited 0 and all 13 telemetry records matched `evidence/rust-core-contact-timing-model-20261112.jsonl`. These are gate/model and scripted replay checks; they do not exercise native keyboard delivery, focus events, physical interaction or the headless capture build. Cross-platform and package requalification remain pending.

Tested against checkpoint `14132069cf07a204d8d7418e20e3520b903c3481`, with `experiments/blue-engine/src/main.rs` Git blob `613b25e8d280bed7956741850e547e10dfc2f4a5`; rebuilt binary SHA-256 `f3e8732484f0c3b005b25dfc9fc9a81aed4d4c9f9df4b623b10a88ab7f4278cd`. The previous executable is preserved at `/tmp/pong-blue-pre-held-20261113/pong-blue-engine`, SHA-256 `fb2183dc9f584b6eb4e4542e56d2933d025b8b2ab48072f9c1211600cc661fcd`. The new replay log is `/tmp/pong-blue-held-pause-run-20261113/replay.log`. Historical packages and evidence were retained.

## Mac recheck

macOS ARM64 source `f680d1075e6ee6575be107273a8ee8758711a7a3`. Offline locked release build reused a copy-on-write cache; all four Blue Engine frontend tests passed, including held-movement suppression. Cargo reported a future-incompatibility warning for `block` 0.1.6. Executable SHA-256 `0afe7fd41d98f0faba4e5e99a0e60ef2534e77d6716d6fcb0751c18efc0e5397`.

The packaged app opened through supported desktop automation. Sequential Enter, held W, Space, Enter, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused again. The app exited. W was released before pause: held-across-pause native delivery and physical input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac build.
