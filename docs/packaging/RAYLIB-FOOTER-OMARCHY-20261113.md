# raylib status footer — Omarchy, 2026-11-13

The status/instructions line was drawn inside the court and overlapped the player's paddle at its bottom position. The viewport now reserves a scaled footer below the aspect-preserving court; the line is drawn there. A geometry regression failed before the change and passed afterward, checking court separation and window bounds for standard, wide and tall dimensions. All 12 locked offline raylib adapter tests passed.

A rebuilt debug binary (SHA-256 `3af071346d559da94ac699290818509d2ceb088f1f2fa4e8bc85687234526862`; source Git blob `8f2df31ca9f569a53e854fa571b25b3411a73e66`) completed the local-display scripted replay: exit 0, empty stderr, six app-owned captures and 13 JSON checkpoints exactly matching the retained previous replay. Inspection of tick 480 showed the bottom-position paddle inside the court and the paused status below it, without overlap. Outputs are retained at `../pong-raylib-footer-check-20261113/` alongside earlier artifacts.

Only the displayed capture shape was visually checked; the other window shapes are geometry tests, not resized-window interaction. Winner rendering, physical input, clean-machine packaging and release qualification remain open.
