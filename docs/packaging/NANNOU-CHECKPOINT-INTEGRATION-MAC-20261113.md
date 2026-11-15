# Nannou checkpoint-input integration: Mac (2026-11-13)

At checkpoint `7c9cc3b702d0a1993554eb182aa0268aa93afa14`, the Nannou frontend source and manifest/lockfile, shared core source and manifest, scenario source and manifest, and benchmark source and manifest were checked against their Git blobs. The core CPU contact test module was also fetched from that checkpoint. Those checked files matched before this build; this does not assert every possible build input was independently compared.

With the cached Mac target, offline locked release tests passed: 20 core tests, including CPU target arrival at contact, and 13 Nannou tests. The release build succeeded and its executable SHA-256 was `1cf770f649282e213c02ae27151b74efb321487634f6c639aa22f3e2b859e88e`, byte-for-byte identical to the earlier mixed-source binary. The newer core source adds test-only module coverage. The earlier native window observations of Ready, Paused, a 3–7 CPU win and automated restart in `NANNOU-PROMPT-BACKING-MAC-20261113.md` therefore apply to these executable bytes; the GUI match was not repeated. The new app bundle and previous binary remain under `/tmp/pong-nannou-checkpoint-native-mac-20261113-v1/`.

This is bounded test and binary-identity evidence. Physical keyboard input, a player win, resizing, clean-machine operation, complete notices and release qualification remain open.
