# Mac non-playing movement recheck

On 2026-11-13, checkpoint e41207738080dd460efb4b2da318bc4fe9e7e87e was tested in an isolated checkout with copy-on-write target directories. Locked offline macOS ARM64 Release test suites passed:

| Adapter | Tests | Rebuilt Release executable SHA-256 |
| --- | ---: | --- |
| Miniquad | 11 | caa63b7659ca5334887b7fc35fd19304a4cf646d1944ae4121e35f0b007e095a |
| Pixels | 8 | 66e339df548eb404dfbf519765469c02fe2858b1fe96e8b8f3d602b189b56343 |
| raw wgpu | 7 | 614c2db2a269704855a7a375b3d07d4f27979ca97fb938e2094573aa19fd3a26 |
| Piston | 10 | 883b9d7d43802fcdb4a64a2a3a69acc0a4e201b15775e7639fedc035de18bd39 |
| softbuffer | 7 | 8508e7f43c459bca52a6331c69cf4f7687540c861d965181ca5e655cb6fe9d24 |
| ash | 9 | Test binary only; runtime executable not rebuilt in this check |

The new adapter-level regressions cover movement presses outside Playing, repeats, release and fresh press. These tests do not establish native event delivery or physical input. The ash test emitted the existing block 0.1.6 future-incompatibility warning.

The five rebuilt graphical executables each ran a separate bounded native --scripted-test process sequentially; all exited 0, each produced 13 JSON records byte-identical to docs/evidence/rust-core-contact-timing-model-20261112.jsonl, and stderr was empty. No captures or desktop interaction were requested in these runs. Scripted input bypasses the native key path and does not independently prove the fix. Ash's Vulkan runtime was not run; the previously recorded Mac loader/MoltenVK limitation remains.

Outputs remain at /tmp/pong-native-nonplaying-mac-replays-20261113-v1 and /tmp/pong-{adapter}-nonplaying-mac-target-20261113-v1. The isolated source checkout remains at /tmp/pong-ggez-option-mac-review-20261113-v1. Historical binaries, targets and prior evidence were retained. Physical keys, held-through-transition delivery, focus behavior, other platforms, full match and release qualification remain open. Checkpoint when recorded: 7b984a06b0e62303543015e64b17e9c9f6239cb2.
