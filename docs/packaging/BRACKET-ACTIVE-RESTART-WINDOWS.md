# Windows bracket-lib active-restart recheck

At checkpoint `6ba4b5f22b0b1406b2d265afefa8678db26489e6`, including `88dd987`, `experiments/bracket-lib/src/main.rs` Git blob `807249733a9c4c5bbeb6c6c3e67a9ba44e3c0e15`, Windows MSVC stable Rust 1.99.1 passed all 14 locked offline debug tests. The suite includes `active_restart_suppresses_held_movement_until_release`. Cargo fell back to copying incremental cache files where hard-linking failed.

A fresh locked offline debug executable had SHA-256 `92e6111f7e40339555baa931d73b4d93e16ac648e2b3d5612031d34f5aa933dc`. Its isolated scripted run exited 0, stderr was empty and all 13 records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stdout additionally identified OpenGL 3.2.0, build 31.0.101.2145, and shader language 1.50. No capture was requested. Logs and preserved prior binaries remain in sibling workspace directory `bracket-active-restart-windows-20261113`; the existing target was reused.

The regression calls adapter logic and the standard replay does not exercise held native keys during restart. Physical input, accessibility, clean-machine operation and release qualification remain open.
