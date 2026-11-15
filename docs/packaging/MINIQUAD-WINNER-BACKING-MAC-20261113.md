# Miniquad winner-message backing — macOS, 2026-11-13

Miniquad now draws a court-colored backing before its match-end winner and ENTER TO RESTART text. This keeps the net and ball behind the letters, matching the existing Ready and Paused backing. Input and physics are unchanged.

All 15 offline locked release-profile tests passed on Mac. The winner vertex test verifies that backing vertices precede the text vertices and that the message is absent after restart. In the rebuilt native window, an unattended match ended CPU WINS with the winner and restart instruction legible against a clear background. Supported desktop automation sent Return: the match restarted and the backing and message cleared. Escape closed the app. This is automated input and visual observation, not physical keyboard testing.

The executable SHA-256 is `599450b373dced1c67f31982a87e9bb33840ec5afd18f233dace41319e269231`; edited adapter SHA-256 is `71e49df9f5d3f3bf9929d45b05fbeac5f3a057fb6c04b8bef4c891a92e3274a3`. The isolated mixed tree used adapter base blob `6e5487d7a7a1d58d44ce4d7b5c8e59a4311f7a89` from checkpoint `abaa8200bd5aabffc8ea99ae03677affa04986d4`; other inputs were not all compared. Previous source and binary and the candidate app remain under `/tmp/pong-miniquad-winner-backing-mac-20261113-v1`.

Player-win presentation, other window sizes and platforms, physical input and release qualification remain unverified. The [Miniquad Mac notice gap](MINIQUAD-MAC-NOTICE-GAP.md) and complete dependency and system-library review remain open.
