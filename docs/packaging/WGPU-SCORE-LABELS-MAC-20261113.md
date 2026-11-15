# Raw wgpu in-court score labels — macOS, 2026-11-13

The score marks previously lacked in-court side labels. The shader now draws YOU under the left score group and CPU under the right, using its existing pixel glyphs. This does not change scoring or input.

A cached offline locked release build and all 12 existing release-profile tests passed on macOS. The native wgpu window visibly displayed both labels at Ready 0–0 and at 2–2 while Paused, with the score marks and PAUSED prompt still visible. Return and Space were sent by supported desktop automation; this is not physical-input evidence. Escape closed the test window.

The candidate executable SHA-256 is `c4cd4a6674a6d386ddc15bde0d01cb85e251b63e7cfbc34bfe9ea6a6772d38d2`; shader SHA-256 is `3afe5efbefc45e09659f7e282eb739ff38aba550413bd1631cf99bc39e1fa538`. The mixed isolated local tree used main.rs blob `04c081e6cdb9521ffd82028de0e5be1b51945096`, renderer.rs blob `9ed876996cfc35b432b13a8ad367ebb8a171203c`, and core lib.rs blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3`, matching checkpoint `a27b35b3`; the shader was edited from blob `2314b072975edb87a60c6c2c474116a19f6a6dd7`. Other build inputs were not all compared. The candidate app and earlier artifacts remain under `/tmp/pong-wgpu-score-labels-mac-20261113-v1` and `/tmp/pong-wgpu-state-native-mac-20261113-v1`.

Other sizes, platforms, physical controls and release qualification were not checked. Existing unit tests do not validate shader pixels; the native rendering observation supplies this bounded visual check.
