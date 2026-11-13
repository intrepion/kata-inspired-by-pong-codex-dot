# Blue Engine winner title — Omarchy, 2026-11-13

The windowed Blue Engine frontend previously showed only `Over` in its title at match end. It now names `YOU WIN` or `CPU WINS`, with a defensive `Match over` tied-state fallback; Ready, Playing and Paused retain their wording. A regression driving a winning point for both sides failed before the change and passes afterward, including restart. All 9 locked offline adapter tests passed.

A fresh locked windowed debug build (SHA-256 `03dea0353fa0bfceac35d73180db204c5a50bd1951464b5cf18ee8874c056b7a`; source Git blob `a71f3cc5f004bb61e7cd419f9f0e0b5903995bac`) completed a local-display `--scripted-test`: exit 0 and 13 checkpoints exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The saved stderr contains graphics loader information and warnings, including missing Vulkan validation layer and extensions; it is not empty. No capture was requested. Logs remain at `../pong-blue-winner-check-20261113/`; earlier artifacts were retained.

The winner label is in the window title, which may be hidden by a window manager; there is no new in-court winner message. The scenario does not reach match end and does not visually verify the winner title. Physical input, full-match rendering, other platforms, licensing review and release qualification remain open.
