# Nannou score labels: macOS check (2026-11-13)

Nannou's in-court score now identifies the left player and right CPU. Its adapter was based on Git blob `c6469cb6e5a8665af9eead36f221f4e8a9421829` at checkpoint `818e14b763e835f6c7cdd4a705e3b306d60d1bb4`; the isolated build tree was not established to match the whole checkpoint.

The offline locked release build succeeded and all 16 existing frontend unit tests passed. The tested adapter SHA-256 was `8133cb031604fe254b4a7ec58a42bbf06b56ba1bd60ccf69813a50c60db95a66`, and the candidate executable SHA-256 was `4c55171b92a6a18a2a5e2c57422c63719cb991cab74deb446088751717bb1b5a`. A native macOS Ready window visibly displayed “YOU 0 : 0 CPU” centered on court; the test window was closed.

This is a default-size Ready-screen visual check and existing automated tests, not a physical input or played-match check. Nonzero scores, other sizes and platforms, and release qualification remain unverified. The [Mac metadata license screen](NANNOU-MAC-LICENSE-SCREEN.md) did not identify restricted declarations, but native/system review and complete notices remain open.
