# SDL3 focus-resume cross-check on macOS

2026-11-13. The SDL3 source at `7aa9d9ff45aabbf1316f8d3f35d8a1f22a741a01` was fetched into the existing isolated Mac overlay. Offline locked release-profile tests passed 10/10, including `focus_loss_then_immediate_resume_suppresses_held_direction`. Tested main.rs SHA-256 `ba4abb3a56921ae0e899e847aedeb3a40f27c05e0b6048c04b5e65df4c032731`.

This is an adapter/model test, not native focus delivery, physical key input, a graphical replay or release qualification. Existing cached targets and prior artifacts were retained.

## Unfocused actions follow-up

Fetched SDL3 source at `b123ba8d72b09ffe9a2164b6ed800cff1ee8d5e1`, SHA-256 `b8fc44e8361c4a0b9f991b0f6fa349f607add8f56cb4a4ee361e1f6c3b43d294`, into the isolated Mac overlay. Offline locked release-profile tests passed 11/11, including `unfocused_actions_cannot_resume_or_restart_game`. This is an adapter/model test, not a native focus event or physical key test. No graphical replay or release qualification is claimed.
