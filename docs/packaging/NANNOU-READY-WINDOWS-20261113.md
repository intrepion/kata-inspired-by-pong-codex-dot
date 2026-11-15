# Windows Nannou Ready prompt recheck

At checkpoint `ab08b4f862562f5fddf4899c181aeed65f767c4e`, `experiments/nannou/src/main.rs` Git blob `bc6afa1b949c26ded4747e66fd8c12f7817efd26`, Windows MSVC stable Rust 1.99.1 passed all 13 locked offline debug tests, including `ready_prompt_clears_after_start_and_stays_hidden_on_pause_or_over`. Cargo fell back to copying incremental files where hard-linking failed. Prior test binary and symbols were copied to sibling workspace directory `nannou-ready-windows-20261113` before reusing the existing target.

This is prompt-selection model evidence. No new Nannou executable, rendered window, full match, physical input, license clearance or release qualification is claimed.
