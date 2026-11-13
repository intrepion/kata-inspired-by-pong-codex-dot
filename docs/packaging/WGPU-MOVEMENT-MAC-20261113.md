# Raw wgpu automated movement on macOS — 2026-11-13

A bounded native-window check used the candidate previously described in [the action-prompt evidence](WGPU-ACTION-PROMPTS-MAC-20261113.md), executable SHA-256 `5daf6bbdb9cf51b7b56e12a2e2acaae863664b82bb3c86d54731f7a08287fbae`. That evidence records the mixed isolated source provenance and the 12 passing release-profile tests; this is not a claim that every build input matched a checkpoint.

Supported desktop automation sent Return, held W briefly, and paused: the player paddle moved from center to the top. S was pressed and released while paused, then Space resumed; the paddle remained at the top. A fresh S press during play moved the paddle down. After another pause, Return followed by Space produced a paused 0–0 state with the paddle centered. Escape closed the app. Score and state were observed in the native window title, and paddle positions were observed visually.

The paused S press was released before resume; this does not prove behavior for a key physically held through pause or focus change. These actions were automated, not physical input. Other keys, complete gameplay, other window sizes and platforms, and release qualification remain open.
