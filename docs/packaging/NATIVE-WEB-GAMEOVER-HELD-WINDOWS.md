# Windows native and web game-over held-input recheck

At checkpoint `02fed664994da024a3bdd0f0a2643a0af2ad564b`, including `bdbae23`, Windows MSVC stable Rust 1.99.1 passed locked offline adapter/controller tests: Miniquad 12, Pixels 9, wgpu 8, Piston 11, ggez 6, ash 10, softbuffer 8, Fyrox 6, bracket-lib 13 and shared web controller 15. ggez used the release profile; the others used debug. Each suite included its match-end held-input regression. Source Git blobs match the corresponding entries in [the Omarchy report](NATIVE-WEB-GAMEOVER-HELD-OMARCHY.md). Cargo reported incremental-cache hard-link fallback to copies for several copied targets.

A fresh locked offline debug Miniquad build produced executable SHA-256 `6a909cd7929f7a061f38608210fc2b3c46f61ba8288d29b9b4cb6c26cf20bdbc`. Its scripted run exited 0, stderr was empty, and all 13 records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. This standard replay does not reach game over or exercise real keys. No captures were requested. Its target is `target-native-gameover-windows-20261113` and output is in sibling workspace directory `native-gameover-windows-20261113`. Separate copied ggez, Fyrox and bracket targets and the separate web test target remain in the checkout.

The regressions call adapter/controller logic rather than OS or browser events. Physical input, browser pointer delivery, both-winner integration, clean-machine and release qualification remain open; the standard replays do not exercise a game-over restart.

At the same checkpoint, fresh locked offline builds of the remaining eight adapters each completed a scripted replay with exit 0 and 13 parsed records matching the reference. ggez was built in release mode; the others were debug. No captures were requested. Ash stderr identified Intel UHD Graphics 630 Vulkan; Fyrox stdout included renderer logs and an OpenGL fragment-shader recompilation performance warning; bracket-lib stdout identified OpenGL 3.2. The other stderr streams were empty. These scripted runs do not test native key delivery or game-over restart. Each had an isolated working directory under sibling workspace directory `native-gameover-eight-windows-20261113`.

| Adapter | Executable SHA-256 |
| --- | --- |
| Pixels | `19d2b4809dcfb5b4d9e337d8aae75a6708cf226dc05cfaacaf70a2bef6e02e9f` |
| wgpu | `8567e1eaa3068716e9df441617c9605fd916732fdcb16d694de9631eb86ef1c6` |
| Piston | `ca3d162ccd87277f0f8fc11e9f8a7b64779e2a2040f151bb239404bd43ab9eed` |
| ash | `434d7f27108ce2ce46ce5cd9231ceac5d223f85e2136be74467460c1c8251754` |
| softbuffer | `5a1ab45adc69b0e7c6bba2ca959d73ed269fd44fdbc21d992c222725e15283f0` |
| ggez | `170d424a1e8272f7d216669ecba7b3b4386e687c64adc89b8ea140863f096003` |
| Fyrox | `c453107a36afe012cb2101d4fba30af84ace96d5a8aeed7837218a7cffe4e601` |
| bracket-lib | `8c77ed8280ff3f1cde0043e681b9a9dedbff0b3daf3ca509563e07942cedcf02` |