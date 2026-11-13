# Wgpu and Pixels keypad Enter — macOS, 2026-11-13

Both winit adapters now accept `NumpadEnter` for the same start/restart action as Enter and R. Repeats, releases and unfocused keys remain ignored, and restarting still clears held movement.

Offline locked release-profile tests passed on Mac: 13 wgpu and 15 Pixels tests. The added model regression checks keypad start, unfocused/repeat/release rejection and fresh restart; existing restart and match-end movement regressions now include the keypad binding. These tests inject winit key codes and do not prove OS keyboard delivery.

A rebuilt native wgpu candidate received the desktop automation tool's `KP_Enter` command: Ready changed to Playing, and a later command restarted before Space paused the 0–0 match. The tool's mapping to a physical keypad event was not independently observed; physical keypad input and native Pixels input remain unverified. The wgpu executable SHA-256 was `a5ca692b0e6689371e3fe5f7e65155787fe3a435b8b29314c1b157f4ed0eb2c3`. Candidate source SHA-256 values: wgpu `e382648bdab9dc0a6c1f04cd48bd7ee1b47842022c8d6b49f179b8f27a8d74cb`, Pixels `850c82925840e6a8764706c1af29d02243ea9bc274b88ff5c3bd9fd09b8e0a29`.

The mixed isolated build used adapter bases wgpu blob `04c081e6cdb9521ffd82028de0e5be1b51945096` and Pixels blob `773a27d6350fbd5c708aa96ddfe05e886431f5b8` from checkpoint `61dd9917`; other inputs were not all compared. Earlier source and wgpu binary are retained in `/tmp/pong-winit-keypad-mac-20261113-v1`. Other platforms, complete gameplay, dependency notices and release qualification remain open.
