# Softbuffer held-input Mac relocated package check

2026-11-13, Mac ARM64. This is a bounded local package check, not release qualification.

The existing Release pilot archive contains the held-pause corrected binary built at source commit `caca70820acbb49046878fae07eb69509dc5eca6`. The prior offline locked build and six frontend tests are documented in [the pause follow-up](../SOFTBUFFER-ASH-PAUSE-HELD-INPUT-FIX.md); they were not rerun here. At checkpoint `cdd534168d8b436a482938376cd5f4d1709914c5`, the softbuffer source, manifest and lockfile, low-level-common, core, scenario and benchmark library sources, and smoke scenario matched that historical commit. This comparison does not claim a fresh build at the checkpoint.

- Archive `pong-softbuffer-macos-arm64-pilot.tar.gz` SHA-256: `fc82c6e743503836ffcef64ac3e8aa9591f3ba567374a33d0ecd995286798afc`.
- Extracted executable SHA-256: `51ebf96596fc6e92d1c1499eb03604599db19b4d8308e86f4d79a4b2139a5694`, matching its manifest. The extracted Info.plist passed `plutil -lint`.
- Extracted under `/tmp/pong-softbuffer-pause-relocated-20261113-v1`, the executable ran `--scripted-test --scripted-captures .../captures` and exited 0. Its 13 JSONL records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` byte for byte, and six app-owned PNGs were emitted; these captures were not visually inspected here. The runtime printed `sandbox_extension_issue_file_to_process failed ... Operation not permitted` during this otherwise completed run.
- Separately, supported desktop automation opened the relocated app and observed its window title transition from Ready to Playing after Space, Paused after Space, and Playing at 0–0 after R. Escape closed the app. These are sequential automated actions, not physical keyboard input or held-through-pause testing.

Archive and extracted artifacts remain local. No removal, clean-machine installation, signing/notarization, complete dependency notice review, accessibility, physical input, held-through-pause interaction or distribution qualification is claimed.
