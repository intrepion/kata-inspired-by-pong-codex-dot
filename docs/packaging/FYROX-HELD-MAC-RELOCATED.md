# Fyrox held-input Mac relocated package check

2026-11-13, Mac ARM64. This is a bounded local package check, not release qualification.

The existing Release pilot archive contains the held-pause corrected binary built at source commit `f680d1075e6ee6575be107273a8ee8758711a7a3`. The prior offline locked build and four frontend tests are documented in [the pause follow-up](../FYROX-HELD-PAUSE-FIX.md); they were not rerun here. At checkpoint `cd09f7691574d272f0fcae418ba10611015fdf5b`, the Fyrox main source, manifest and lockfile, core, scenario and benchmark library sources, and smoke scenario matched that historical commit. This is a comparison of those files, not a fresh build at the checkpoint.

- Archive `pong-fyrox-macos-arm64-pilot.tar.gz` SHA-256: `69166b623ba4f8781c8e12e544f80fee24c777e54ba3c43bda64d28c9b2fdf8c`.
- Extracted executable SHA-256: `c68c5c4f7039fca32da063008336af4978a0db04c18f3fe2f99e6ed92db67cc7`, matching the manifest. The extracted Info.plist passed `plutil -lint`.
- Extracted under `/tmp/pong-fyrox-held-relocated-20261113-v1`, the executable ran `--scripted-test --scripted-captures .../captures` from that dedicated working directory and exited 0. Filtering its 13 JSON telemetry lines from the mixed runtime log produced a byte-for-byte match with `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned PNGs were emitted but not visually inspected in this check. The process created `data/resources.registry` in the working directory; this state is retained. Stderr reported `sandbox_extension_issue_file_to_process failed ... Operation not permitted` despite completion.
- Separately, supported desktop automation opened the relocated app; Enter showed Playing at 0–0 and Space showed Paused at 1–1. R was pressed and the later screenshot showed Playing at 0–1, so a zero-score reset was not established. Command-Q closed the app. These are sequential automated actions, not physical input or a held-through-pause test.

The archive and extracted evidence remain local. No removal, clean-machine installation, signing/notarization, complete dependency notice review, accessibility, physical input, held-through-pause behavior or distribution qualification is claimed.
