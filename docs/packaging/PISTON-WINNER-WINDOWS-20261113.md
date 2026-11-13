# Windows Piston winner-title recheck

At checkpoint `b18d45f2ba88c1668a68cfcbc6e89cbd6a0939c4`, `experiments/piston/src/main.rs` Git blob `c810cfe0de50eaa37f3cb7566e11e7059e116c60`, Windows MSVC stable Rust 1.99.1 passed all 13 locked offline debug tests. The winner-title test drives a winning score for each side, verifies the corresponding title and checks restart clears it. Cargo fell back to copying incremental files when hard-linking failed. This is adapter/model evidence, not a visible game-over title check.

A fresh locked offline debug executable had SHA-256 `730c7f0eb9d789f309f3f06dadf24acc92a56f78cac14a3868d98615fc85fd67`. Its scripted run exited 0 with empty stderr and all 13 parsed telemetry records matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The standard replay does not reach game over, so the actual winner title and its fullscreen visibility remain unverified. Logs and selected prior binaries remain in sibling workspace directory `piston-winner-windows-20261113`; the existing target was reused.

No native key delivery, physical input, full interactive match, clean-machine installation or release qualification is claimed.
