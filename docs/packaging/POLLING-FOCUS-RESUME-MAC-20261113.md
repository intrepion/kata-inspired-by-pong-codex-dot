# Nannou and raylib focus-resume cross-check on macOS

2026-11-13. The landed focus-resume changes from `ad437fd1d5297e593b98532a6126f49b01ea5167` were fetched at checkpoint `437ae1ceb501b5b7c7a478bbf5043eedce75863a` into the isolated Mac overlay. Offline locked release-profile tests passed: Nannou 12/12 and raylib-rs 13/13. Each includes `focus_loss_then_immediate_resume_suppresses_held_key`, along with its active-restart and winning-point regressions. Tested source SHA-256: Nannou `3999b0c1ed101f16b7815a6d3a9594f829e1a09541245c8f50c618660d501d26`; raylib-rs `8b54ee4a33721dc172f314b0ad5b175a24e25bed870d9d81ee6a11d67a2ca574`.

These are adapter/model tests. No native focus change, operating-system key delivery, held physical key, graphical replay or release qualification was checked. Existing isolated cached targets were reused, and prior artifacts were preserved.
