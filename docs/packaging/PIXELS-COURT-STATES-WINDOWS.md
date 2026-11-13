# Windows Pixels court-state recheck

At checkpoint `ce015a506bbaab4851fbaec995803f61ff01e612`, Pixels source Git blob `773a27d6350fbd5c708aa96ddfe05e886431f5b8`, Windows MSVC stable Rust 1.99.1 passed all 14 locked offline debug tests, including `court_shows_state_and_distinguishes_winner` and `message_backdrop_clears_ball_only_outside_play`. A separate worktree was used to validate the shared implementation; prior executable artifacts and a concurrent local draft patch were preserved in sibling workspace directories.

A fresh locked offline debug executable had SHA-256 `423b52d732f4c7bc952871395dcaa8d6d87a615343b6057321f79fdfb9d5a920`. Its isolated scripted run exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. The inspected tick-480 capture shows `PAUSED`, the 0-1 score and court; its SHA-256 is `70a07dcc3beabcdf24451b0e27e564b3a609a1f6cb9d5d7442b887ca00d79890`. The standard replay does not reach a winner, so actual complete-match winner rendering remains unverified. Logs and captures remain in sibling workspace directory `pixels-shared-state-evidence-windows-20261113`.

This is framebuffer-test and app-owned scripted-capture evidence, not physical input, actual OS focus delivery, accessibility, license clearance or release qualification.
