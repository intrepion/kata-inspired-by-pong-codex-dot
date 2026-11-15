# Windows SDL3 focus/resume recheck

At checkpoint `0a71fd25e6bd7e476070cd9d3c282f8b45cd0d39`, including `7aa9d9f`, SDL3 source Git blob `3a9965738175a3c4b1d7a5e2b02c8cf29a1b15e6`, Windows MSVC stable Rust 1.99.1 passed all 10 locked offline debug tests. The suite includes `focus_loss_then_immediate_resume_suppresses_held_direction`. Unlike the nonrelocatable copied release cache in the earlier attempt, the original `target-sdl3-c77` debug target was reusable. Previous test and executable artifacts were copied to sibling workspace directory `sdl-focus-resume-windows-20261113` before reuse; the earlier copied target remains intact.

A fresh locked offline debug executable had SHA-256 `b760b438fbe466a43234a36f8c71548a91b57bd2a2b051488c67f383d6c1e52e`. Its isolated scripted run exited 0 with empty stderr and all 13 parsed records matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. No capture was requested. The ordinary debug test and build still emitted `LNK4098` for conflicting `MSVCRTD` default libraries, as well as incremental hard-link fallback warnings; the previously documented runtime-linkage limitation remains unresolved.

The regression calls the handler and polling gate directly; the standard replay does not exercise actual window focus or held physical keys. No license clearance, accessibility, clean-machine or release qualification is claimed.
