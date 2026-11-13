# Miniquad score-owner labels — macOS, 2026-11-13

Miniquad now labels the left score pips YOU and the right pips CPU inside the court, using the existing font-free glyph geometry. Input and physics are unchanged.

All 16 offline locked release-profile tests passed on Mac. The added geometry test checks both sides and verifies that the score labels and a maximum-pip winner scene remain within the existing 4096-vertex buffer. In the rebuilt native window, both labels were legible in Ready, with score pips during play, and in a completed CPU-win scene. Supported desktop automation sent Return to restart; the pips and winner message cleared while the labels remained. Escape closed the app. This is automated input and visual observation, not physical keyboard testing.

The executable SHA-256 is `257be3c379590faa49140d65f1e613b393ee8c12289d95c5bb372bfd5fcde1b0`; edited adapter SHA-256 is `608e6a538927b4cfb22a923f450a43720c2a4ef1d463739dfcc3bf4cea6a6d2d`. The isolated mixed tree used adapter base blob `c7dcb36d582cff739b26fa55233328c2ee920006` from checkpoint `c41deecfd6c9fc25f09e7de3c2d503c332870765`; other inputs were not all compared. Previous source and binary and the candidate app remain under `/tmp/pong-miniquad-score-labels-mac-20261113-v1`.

Player-win presentation, physical input, other sizes and platforms, and release qualification remain unverified. The [Miniquad Mac notice gap](MINIQUAD-MAC-NOTICE-GAP.md) and complete dependency and system-library review remain open.
