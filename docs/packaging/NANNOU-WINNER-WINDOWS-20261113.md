# Windows Nannou winner-message recheck

At checkpoint `9c0fd73ef711fb004c071d784dde08a2672bb2ef`, `experiments/nannou/src/main.rs` Git blob `04811fbce9c867f06092c1f6b6eeb1acdc3d8cb9`, Windows MSVC stable Rust 1.99.1 passed all 11 locked offline debug tests. The winner-message regression drives winning points for both sides, verifies the corresponding message and checks restart clears it. Cargo fell back to copying incremental cache files when hard-linking failed.

The prior test binary and debug symbols were copied to sibling workspace directory `nannou-winner-windows-20261113` before reusing `target-polling-active-windows-20261113`. This is adapter/model evidence; no Nannou executable was rebuilt or replayed for this change. Winner rendering, physical input, a full interactive match, clean-machine installation and release qualification remain open.
