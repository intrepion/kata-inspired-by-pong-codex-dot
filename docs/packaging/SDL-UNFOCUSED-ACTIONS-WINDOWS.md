# Windows SDL3 unfocused actions recheck

At checkpoint `d0537c157d813e6f1c22f05a0b4d49b18574a75a`, including `b123ba8`, SDL3 source Git blob `42c616548220022e4ba0b183c06ad75ce61ea160`, Windows MSVC stable Rust 1.99.1 passed all 11 locked offline debug tests. The suite includes `unfocused_actions_cannot_resume_or_restart_game`. The existing SDL3 debug target was reused after copying the previous test and executable artifacts to sibling workspace directory `sdl-unfocused-actions-windows-20261113`.

A fresh locked offline debug executable had SHA-256 `c25685771bd9191c16c5c32b265872f951d1f1c52cb9891e058f61ad070b6c45`. Its isolated scripted run exited 0 with empty stderr and all 13 parsed records matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. No capture was requested. The ordinary debug test and build continued to report `LNK4098` for conflicting `MSVCRTD` default libraries and incremental hard-link fallback warnings; the runtime-linkage limitation remains unresolved.

The regression invokes the event-handling boundary directly, and the standard replay does not exercise real focus changes or queued physical keys. No accessibility, license clearance, clean-machine or release qualification is claimed.
