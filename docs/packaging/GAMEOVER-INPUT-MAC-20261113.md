# Mac game-over input cross-check and Swift simulator build

2026-11-13. In the isolated source overlay described in [the earlier Mac input report](NONPLAYING-INPUT-MAC-20261113.md), the following changed source files were fetched from checkpoint `1101a127855b9d59078b1d1ab8ab8e13a01d3646`, including the game-over held-input fix `bdbae23`. Locked offline macOS ARM64 release-profile tests ran using separate cached or copy-on-write targets. Each suite passed, including its held-through-match-end-and-restart regression:

| Adapter/controller | Passing tests | Source SHA-256 |
| --- | ---: | --- |
| bracket-lib | 13 | `c682a18e20715d93caf171950754991c8fa0eff2ef0a00aa57e79ab1212b511b` |
| Fyrox | 6 | `8711ca93041a31e9b044795ebba250ce646e4e9919ae630b7f96960e50efa7e2` |
| ggez | 6 | `6309bacd2f5c8f08f9416b6861b53938f81b77b6266e3fc1a55c909f99e83cb6` |
| Miniquad | 12 | `d056a074bcf47d04603ad67271a661c227a1e049744e90851a0caae713d991fa` |
| Piston | 11 | `c8ab53a208280dc620ef12f747cdd527bf849bf4d6187010e245a37a0af6c302` |
| Pixels | 9 | `eb1f9cc2fbada80c7469a23274b458786f1ff7901b2d88098b401dd8d7ec4eff` |
| softbuffer | 8 | `392ac4b53f0676745bd78a2c377cc1e1463d3cf27c99be11f91eec2b04c6779d` |
| raw wgpu | 8 | `2220ffdc3b5758581f19bc417a7f8e1401c4fa82beda4649e73771c7bcb0476b` |
| ash | 10 | `cb1c3808567a8177632866435bccc619d956f240f5ba23f9cd5bbcbc6e63ac45` |
| shared web controller | 16 | `4be026a0eabca996c7456395e90448bb1c8700018db688c16b5a641681448eaf` |

These are adapter/model tests; no actual held keys, browser pointer, graphical replay, Vulkan runtime or physical input was checked. The first Pixels attempt began rebuilding GPU dependencies and was interrupted; the completed run used a cloned cached target. The tested source overlay is not a complete checkout at the checkpoint, and previous artifacts were preserved.

## Swift simulator attempt

The local isolated source contained Swift Game.swift SHA-256 `4fe688baef4341a6c00138c5f21c725de7baf6327f866c9ac18f249aefae9706`, corresponding to the [collision fix](SWIFT-COLLISION-MAC-20261113.md). XcodeGen successfully generated an isolated project. `xcrun simctl list devices booted` failed because CoreSimulatorService was unavailable and its log path was not writable. A generic iOS Simulator Debug `xcodebuild` attempt exited 74 before a build: it likewise could not connect to CoreSimulatorService, and package resolution could not write the user's Swift/Clang module and manifest caches (operation not permitted). No system security settings or sandbox protections were changed. No new simulator binary, app launch, interaction or render evidence is claimed. The isolated generated project and derived-data path are `/tmp/pong-mac-input-recheck-20261113-v1/experiments/native-swift` and `/tmp/pong-swift-collision-ios-derived-20261113-v1`; prior simulator data and app artifacts remain intact.
