# SDL3 focus-resume cross-check on macOS

2026-11-13. The SDL3 source at `7aa9d9ff45aabbf1316f8d3f35d8a1f22a741a01` was fetched into the existing isolated Mac overlay. Offline locked release-profile tests passed 10/10, including `focus_loss_then_immediate_resume_suppresses_held_direction`. Tested main.rs SHA-256 `ba4abb3a56921ae0e899e847aedeb3a40f27c05e0b6048c04b5e65df4c032731`.

This is an adapter/model test, not native focus delivery, physical key input, a graphical replay or release qualification. Existing cached targets and prior artifacts were retained.
