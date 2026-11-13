# Windows Piston Ready and Paused indicator recheck

At checkpoint `3ef7a49e0b9fa18a7c902eb5128209af109efa83`, `experiments/piston/src/main.rs` Git blob `87bbb5d484376e5f7c562649aa9f610adfb5a5ee`, Windows MSVC stable Rust 1.99.1 passed all 15 locked offline debug tests, including `ready_play_symbol_and_pause_bars_match_state`. Cargo fell back to copying incremental files where hard-linking failed. Prior test and executable artifacts were copied to sibling workspace directory `piston-state-windows-20261113` before reusing the existing target.

A fresh locked offline debug executable had SHA-256 `9af5183e50efb0ba82171485115858a85da3823131a5af83c7064eec31fb847a`. Its isolated scripted run exited 0 with empty stderr and all 13 records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were produced. The inspected tick 480 capture showed the court and pause bars. The standard replay does not capture Ready, so the Ready symbol is supported by the model test rather than an app-owned capture. Logs and captures remain in the workspace directory above.

No native key delivery, physical input, full interactive match, license clearance, clean-machine operation or release qualification is claimed.
