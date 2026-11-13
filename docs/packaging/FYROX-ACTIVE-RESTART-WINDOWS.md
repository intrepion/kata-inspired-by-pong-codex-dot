# Windows Fyrox active-restart recheck

At checkpoint `a9066764adf1a283b9d9d206d27fed962f17536c`, including `39d3037`, `experiments/fyrox/src/main.rs` Git blob `6532f6e693f4f53165611dbe993dbf310b17f6af`, Windows MSVC stable Rust 1.99.1 passed all seven locked offline debug tests, including `active_restart_blocks_held_direction_until_release`. Cargo fell back to copying incremental files when hard-linking failed. Previous test and executable artifacts were copied to sibling workspace directory `fyrox-active-restart-windows-20261113` before reusing the existing target.

A fresh offline debug executable had SHA-256 `14f647d12d87188bae037caa29a828364ae1e30a088d0036fa2c2802e021acc1`. Its isolated scripted run exited 0, stderr was empty and all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stdout contained an OpenGL performance warning, with full renderer logs retained in the same workspace directory. No capture was requested.

The regression calls adapter logic and the standard replay does not exercise a held physical key through restart. No license clearance, accessibility, clean-machine or release qualification is claimed; see existing Fyrox dependency audits for the separate licensing constraints.
