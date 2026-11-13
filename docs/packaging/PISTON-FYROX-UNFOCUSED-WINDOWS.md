# Windows Piston and Fyrox unfocused-action recheck

At checkpoint `7e016204474c022f21145cfee53e13265fcb89df`, including `34ed38e`, Windows MSVC stable Rust 1.99.1 passed locked offline debug tests for Piston (14) and Fyrox (8). Each suite includes `unfocused_key_actions_do_not_resume_or_restart`. Tested source Git blobs are Piston `5a7a9560e1c9c226f19723b7697a29666849fb15` and Fyrox `a686e308536bbe3e30d1c3a1193162f84fb8367d`. Cargo fell back to copying incremental files when hard-linking failed. Previous test and selected Piston executable artifacts were copied to sibling workspace directory `unfocused-piston-fyrox-windows-20261113` before reusing existing targets.

A fresh locked offline Piston debug executable had SHA-256 `2887628641f389d489b9fdab34a4af584698e5f553c1c56f34a598739c3d1d70`. Its isolated scripted run exited 0 with empty stderr and all 13 records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. No capture was requested; logs remain in that workspace directory.

These tests call event-action boundaries, and the standard replay does not exercise OS focus transitions or queued physical key events. Miniquad remains unchanged because its platform callbacks do not provide a safe universal focus signal. No accessibility, license clearance, clean-machine or release qualification is claimed.
