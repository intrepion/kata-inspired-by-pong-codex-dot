# Active restart: Fyrox, Nannou and raylib-rs — Omarchy, 2026-11-13

Active restart retained a held movement direction in all three adapters. Fyrox now marks held inputs suppressed before R starts a new match. Nannou and raylib-rs now mark their polling movement gate as not previously playing on restart, so the next sample suppresses keys already down. Release followed by a fresh press restores movement; existing pause and game-over rules remain covered by the suites.

The new regression in each adapter failed before its fix and passed afterward. Full `cargo test --offline --locked` suites passed: Fyrox 7, Nannou 10, raylib-rs 10. Tests exercise adapter handlers and polling gates on Omarchy, not physical input, actual rendered windows, rebuilt release packages or other platforms. Existing capture and release evidence remains historical.

Source Git blobs: Fyrox `6532f6e693f4f53165611dbe993dbf310b17f6af`, Nannou `ee0bc8d5bbc8dd661de9354d147867dbed843207`, raylib-rs `5b8c2852266abaeebb375521e33fea7a02405aee`. Other adapters remain outside this batch.
