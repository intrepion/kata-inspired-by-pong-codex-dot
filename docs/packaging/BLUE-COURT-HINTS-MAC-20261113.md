# Blue Engine court hints — macOS, 2026-11-13

Mac native cross-check of the published court hints at `b5ec27ff`. All 15 offline locked release-profile tests passed, including state-to-hint and glyph checks. The build emitted the existing future Rust compatibility warning for `block 0.1.6`.

In the rebuilt native window, Ready showed ENTER TO START and Paused showed SPACE TO RESUME. The hint was absent in a Playing frame. An unattended match ended CPU WINS at 3–7; a later settled frame showed ENTER TO RESTART. Supported desktop automation sent Return, which restarted at 0–0 and cleared the message; Command-Q closed the app.

One capture while the title had changed to CPU WINS showed partially updated winner glyphs, a prior SPACE TO RESUME hint and a 3–6 court tally; a later capture showed the correct 3–7 winner and restart hint. The capture and update timing are not isolated, so this observation does not establish whether a user-visible transient occurs or its cause. Native UI calls also took longer than in prior Blue checks, including a 60-second initial selection timeout. Inventory showed one candidate running; it was selected without another launch. No performance diagnosis is claimed.

The executable SHA-256 is `65412522d44a897d88bdf0c6fbddc15ac77e04db8afb71fe5f24b50aec797e3b`; adapter SHA-256 is `73898bf2ca63ee085ee96b0d3b89f14a67bec2e603d8470f50f9d32627fd8bc2`. The isolated mixed tree used adapter blob `5cc550a49b29d8c3c21cefc814480eb73660a773` matching checkpoint `478a16e22258002828799f00bb21c90bd67b17e3`; other inputs were not all compared. Previous source and executable and the candidate app remain under `/tmp/pong-blue-court-hints-mac-20261113-v1`.

These are visual observations and automated commands, not physical input. The transient, performance impact, player-win presentation, other sizes and platforms, complete system-library and notice review, and release qualification remain open. The [Blue Mac license screen](BLUE-MAC-LICENSE-SCREEN.md) is not distribution clearance.
