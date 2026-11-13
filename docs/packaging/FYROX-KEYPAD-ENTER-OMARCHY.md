# Fyrox keypad Enter: Omarchy adapter check

On checkpoint `a5ed05de5da4e44b16aa51e7d79d565c7a5a7a6b`, the Fyrox source blob was `91ac19cc66d3a3d183911a5252ffbc0923a34115`. On Omarchy, `cargo test --locked --offline` in `experiments/fyrox` passed all 10 tests using the existing target cache and a workspace `TMPDIR`. Tests call the adapter handler directly, covering keypad start and resume, ignored repeat and unfocused actions, and held movement through match-end restart.

An independently prepared regression on the preceding checkpoint failed before the equivalent fix because keypad Enter left the game in Ready; the locally fixed handler passed it. The concurrent Mac change was then tested as above and retained intact.

These are automated adapter tests, not native event delivery or physical keyboard testing. No fresh packaged build, clean-machine installation, release qualification, or dependency license clearance is established by this check. The nearly full `/tmp` was not cleared and existing caches and artifacts were preserved.
