# Windows Bevy active-restart recheck

At checkpoint `9dfc79043046aea02702f14a906c2bbf6dd38c71`, including `698730e`, Windows MSVC stable Rust 1.99.1 passed all six locked offline debug adapter tests, including `held_key_must_not_survive_active_restart`. The tested `clients/bevy/src/main.rs` Git blob is `7494de30094aed31d01240f4f80d62f82e491de7`. Cargo fell back to copying incremental files when hard-linking failed. Previous `pong_bevy*` test and dependency binaries and debug symbols were copied to sibling workspace directory `bevy-matchend-test-binaries-preserved-20261113` before reusing `target-bevy-matchend-windows-20261113`.

This is an adapter/model check, not a native key delivery or physical keyboard test. No executable was rebuilt or replayed for this revision. The separate [Windows Vulkan replay crash](BEVY-WINDOWS-REPLAY-CRASH.md), accessibility, clean-machine and release qualification remain open.
