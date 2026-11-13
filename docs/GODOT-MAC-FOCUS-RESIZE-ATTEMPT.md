# Godot Mac focus and resize attempt

2026-11-13, macOS ARM64. The existing held-pause pilot used the project described in [the Mac recheck](GODOT-HELD-PAUSE-FIX.md), including Godot 4.5.1 and debug extension SHA-256 `cbd5fa8f5c7d848fa68187bf5bb77bc508c01c80b645affec8e1b80e9a3d82b6`. Current project `main.gd` SHA-256 was `ef64fcd8490f8eb3b2a76442ef6ee0f527ec176653231827739a10f524e0cccd`. This is not a new build or package qualification.

Selecting the local wrapper reported that its application could not be found, but process inspection showed that it had launched Godot with the intended project. Selecting the running engine through desktop automation exposed the game. Space showed Playing at 0–0. A drag at the lower-right corner left the screenshot at the same apparent 900×636 size; an actual resize was not established. Selecting Finder and returning to the game showed Playing, but the observations did not establish that Godot had actually lost focus. Neither attempt demonstrates a focus or resize defect or a pass. Escape exited the test process.

The wrapper's fixed `--log-file /tmp/pong-godot-held-pause-mac-native.log` reused the previous run's path. The launch replaced that prior local log; the resulting 150-byte log has SHA-256 `3731c27c164bac92bc9be4009dceae0a9b92f3d10b58ef4bb58179c4a51a275e`. No backup was found at that path. Prior published written evidence remains; the previous log contents were not recovered. Future runs should use a fresh log path and avoid this wrapper as-is.

A verified native focus transition, actual resizing, physical key delivery, held-across-pause input, a release package and distribution remain open.
