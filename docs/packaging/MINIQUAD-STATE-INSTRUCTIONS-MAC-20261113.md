# Miniquad state instructions: Mac native check (2026-11-13)

The Miniquad source blob `6e5487d7a7a1d58d44ce4d7b5c8e59a4311f7a89`, source SHA-256 `6cdafaae82e69aa47b2f2dc8af7ebbe950899f47ea1e1903eb32d23771b0a624`, was built against local core, scenario, and benchmark source files matching checkpoint `f417dbe1967065a05e82899bd891c94547a50020`. The Miniquad manifest and lockfile and core test module also matched that checkpoint. This does not assert all build inputs were independently compared.

An offline locked cached release build succeeded (executable SHA-256 `91dc24ee329269edbe57caf76477e509610bbfb1f63c1bebedf077b4a08654b0`); all 15 Miniquad release tests passed. In the native Mac window, the Ready play symbol and “ENTER TO START” were legible, with the ball below the instruction. Desktop automation sent Return then Space; the pause bars and “SPACE TO RESUME” were legible. A second Space cleared the panel during play. The app bundle and prior binary remain under `/tmp/pong-miniquad-prompts-native-mac-20261113-v1/`.

This is one native-window visual check with automated keys, not physical keyboard testing, a full match, or release qualification. Winner state, focus transitions and other window sizes remain unverified in this run.
