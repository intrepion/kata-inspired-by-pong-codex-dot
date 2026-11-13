# Miniquad capture recheck — Omarchy, 2026-11-13

Correction: the initial assessment that the new tick-480 PNG was blank and tick-600 incomplete was wrong. Reinspection shows the court, center divider, paddles and ball. All six final-run PNGs are byte-for-byte identical to their corresponding retained pre-focus-change files under `/tmp/pong-miniquad-c77-host-captures-20261112/`; dimensions are 1474×1864 and each contains four colors. The two questioned files have SHA-256 `53a3a731e54eeba43e8752dfe66521bcdd12d874124b030897732ad81015ab1c` (480) and `29acb384ae3e8c17d59d442b725a6024d4cad99edea79c266b564aa4edc3853f` (600).

As an independent check, ImageMagick pixel reads in each file find the center line at (737,600) `srgb(46,64,89)`, player paddle at (60,1200) and CPU paddle at (1410,900) `srgb(89,242,217)`, and the ball white at (945,937) for tick 480 and (1250,940) for tick 600. A comparison and pixel-presence command passed. The final run's 13 telemetry checkpoints also matched the previous replay, as recorded in the [focus report](MINIQUAD-LINUX-FOCUS-OMARCHY.md). Files were preserved unchanged. The reason for the misleading initial visual interpretation is not established.

This corrects the prior capture report; it does not independently establish correct focus delivery, physical input, winner rendering, other platforms or release qualification.
