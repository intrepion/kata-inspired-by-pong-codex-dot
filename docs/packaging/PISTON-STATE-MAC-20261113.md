# Piston Ready and Paused indicators

2026-11-13. Piston previously exposed Ready and Paused primarily in its window title. It now draws a play symbol while Ready and pause bars while Paused, using rectangles inside the existing court; Playing and Over draw neither indicator. No font or dependency was added.

On macOS ARM64, offline locked release-profile tests passed 15/15, including the new state-geometry test and the concurrently landed Piston unfocused-action regression. The cached release build succeeded. Tested main.rs SHA-256 `0d2237f3500bbe477b3c7aaa6899fbe2e0143553d12f46a7792a2d543fe4d682`; executable SHA-256 `68fd22023a3dc8b57f6ca4ef26bfe65417d6e9c5aa6de3932945d6acee9a8609`. The source includes the Piston focus change from `34ed38e` as present at checkpoint `7e016204474c022f21145cfee53e13265fcb89df`.

These are geometry/model and build checks, not an actual GPU-rendered window, full match or physical-input verification. Prior artifacts were retained; isolated source and target are under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.

## Fyrox unfocused-action cross-check

The Fyrox source at checkpoint `7e016204474c022f21145cfee53e13265fcb89df` passed 8/8 offline locked release-profile tests, including `unfocused_key_actions_do_not_resume_or_restart`. Its main.rs SHA-256 is `f81578a1f6abc00deeaf5d8c650961e400c32e0ac3701f553708e7c199528a15`. These are adapter/model tests, not native focus transitions or physical input. Miniquad was not modified.
