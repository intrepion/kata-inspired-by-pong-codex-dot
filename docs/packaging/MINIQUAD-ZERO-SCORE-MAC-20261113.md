# Miniquad zero-score marks — macOS, 2026-11-13

Mac cross-check of the independently published Miniquad zero-score change at `992bf3e3`. The adapter source blob `bf01b8fea49d693d21c8bf3aaaf0fe63bf89f5ad` from checkpoint `01ee305f0f1e7da5410d077e2f4be204deca019e` was used in an isolated mixed source tree; other inputs were not all compared. All 17 offline locked release-profile tests passed.

The rebuilt native window showed two outlined zeros in Ready and solid pips after scoring. Supported desktop automation sent Return and Space; the paused restarted state showed two zeros again. Escape closed the app. These are visual observations with automated commands, not physical-input evidence.

The executable SHA-256 is `a06883a289bf870087acd74a8e07f15a315916e274d81587aca1699d3a394958`; adapter SHA-256 is `4cb7cb73e87e4c12aa016fe960add081139f070ddca2780027954050e9a77a9a`. An independently developed local candidate and its binary were preserved alongside the published-source candidate under `/tmp/pong-miniquad-zero-score-mac-20261113-v1`; only the published implementation is checked here.

Other score combinations, sizes and platforms, physical input and release qualification remain unverified. The [Miniquad Mac notice gap](MINIQUAD-MAC-NOTICE-GAP.md) and complete dependency and system-library review remain open.
