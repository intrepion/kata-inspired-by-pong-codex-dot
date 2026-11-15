# Blue Engine action keys on focus return — Omarchy, 2026-11-13

Blue Engine already paused and suppressed movement while unfocused, but its first focused frame accepted action-key presses without accounting for keys held through focus loss. The windowed adapter now resamples Enter, R, Space and Escape independently after focus loss and suppresses any held or newly pressed on that first sample until release. A genuinely new press reported on that first sample is intentionally treated as potentially held; after a clear sample or release, fresh actions work normally. Movement, scripted and headless paths are unchanged.

The synthetic gate regression failed before suppression and passed after it for all four bindings, continued holds, release/repress, a clear first sample and independent Enter/R keys. All 11 offline locked debug tests passed. The windowed debug build SHA-256 is `dc823023d9bac4aab55964f507ae9eb2adf99bc7149d19797a35cad128d33745`; tested adapter blob `aa64770f5776007336bd0827f42715354d15be63` was based on checkpoint `c2f554d7b3efdc07819005d25c2fcfa68471a49b`. The previous target executable was copied to `/tmp/pong-blue-focus-held-actions-20261113-a/preexisting-debug-binary` before rebuilding.

With host-display access, the app-owned windowed scripted replay exited 0 and its 13 telemetry records byte-matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Vulkan/EGL informational and validation-layer warnings appeared on stderr; logs remain under `/tmp/pong-blue-focus-held-actions-20261113-a`.

These are synthetic gate tests and a scripted replay, not native focus delivery, physical input or proof of platform key-state ordering. Other platforms, installation and release qualification remain open. Prior evidence and artifacts are retained.
