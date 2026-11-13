# Low-level court instructions: Mac SDL3 (2026-11-13)

The shared renderer source blob `dcb11dadcc350bb43655a760ced8c42d0128b3dd` from checkpoint `499a29053ce0be58c16e332a333db733c6db66e6`, SHA-256 `6b53518048895478bcaa911f2299d346fda7cedf0011f7ab26c4c1f940e7694e`, includes the start/resume instructions and score-owner labels. The local SDL3 adapter source matched blob `d7c995ac1f91f28dae8e74c02a69f9a58ddc0109`. An offline locked cached release build produced executable SHA-256 `1b9b81bf5e78c4c1cca0b4ff668d01f2cdeafa173261678ed4613a2b9aad907d`.

All six shared unit tests, four shared integration tests and 12 SDL3 adapter tests passed. In the native Mac window, “SPACE TO START” and the YOU/CPU score labels were legible. Desktop automation sent Space twice; “SPACE TO RESUME” appeared under the pause bars. A further Space cleared the instruction during play; Escape closed the app. The app bundle and previous binary remain under `/tmp/pong-sdl-instructions-native-mac-20261113-v1/`.

This is a native visual check with automated keys plus model/raster tests, not physical input, a complete match, other shared frontends, or release qualification. Other window sizes and winner presentation were not checked in this run.
