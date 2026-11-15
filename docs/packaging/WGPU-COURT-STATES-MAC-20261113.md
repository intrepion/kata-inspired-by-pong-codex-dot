# Raw wgpu court-state rendering on macOS — 2026-11-13

A bounded native check of the in-court state messages, not a release qualification.

## Build provenance

The isolated source tree was based on older local material with these files replaced by their exact blobs from checkpoint `056712e72aefa2a1a8e3238dc20728746b1f741b`:

| File | Git blob | SHA-256 |
| --- | --- | --- |
| `experiments/wgpu/src/main.rs` | `04c081e6cdb9521ffd82028de0e5be1b51945096` | `7b3060f670656cb32c8a0e115765ec78db0ca0d907430cdcb902feb989da107e` |
| `experiments/wgpu/src/renderer.rs` | `9ed876996cfc35b432b13a8ad367ebb8a171203c` | `99079003e098332d4fe4266918cca3a6c9758741d17c67a7ab8193cde7a5f590` |
| `experiments/wgpu/src/court.wgsl` | `2314b072975edb87a60c6c2c474116a19f6a6dd7` | `043faa23dd938fd196c4024c2939409d33653f4a9db3adcc9ea76f5bee73792e` |

The local `crates/pong-core/src/lib.rs` blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3` also matched that checkpoint. Other build inputs were not all compared; do not interpret this as a complete build of that revision. Existing earlier files and binary were retained.

Using the cached target, `cargo test --offline --locked --release --manifest-path experiments/wgpu/Cargo.toml` passed 12 tests, including renderer state and title tests. The resulting release executable SHA-256 was `a74958af454f7a51bf521088366e374d0ee78998fa4ea67d546cf855a3f9bda8`; the local app bundle was `/tmp/pong-wgpu-state-native-mac-20261113-v1/Pong wgpu State.app`.

## Native observation

In the native app, the centered READY message was visible. Supported desktop automation sent Return to start and Space to pause: PAUSED appeared on court; another Space resumed play and removed that message. An unattended match reached YOU 3 : 7 CPU: the title and on-court display both showed CPU WINS. Automated Return then showed Playing at 0 : 0 in the title. Escape closed the test app.

These are automated desktop actions and a visual observation, not physical keyboard testing. Player-winner presentation, other window sizes, full manual gameplay and release packaging remain unverified by this check.
