# Windows native non-playing movement recheck

On Windows MSVC, checkpoint `8f6b63e067029259e76e1aa788615d536ebccecc` includes the six fixes in `e412077`. Locked offline debug tests using stable Rust 1.99.1 passed: Miniquad 11, Pixels 8, raw wgpu 7, Piston 10, ash 9 and softbuffer 7. Each suite includes the new non-playing movement regression. The tested source blob IDs match the six listed in [the Omarchy report](NATIVE-NONPLAYING-INPUT-OMARCHY.md). Tests used a separate `target-native-nonplaying-windows-20261113` target directory.

A fresh locked offline debug Miniquad build produced `pong-miniquad.exe` SHA-256 `9af8f1a1553f4e34239e3e3331242d283125f6e6584584e2c7a653801645ac9d`. Its scripted run exited 0 and all 13 parsed JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The replay output remains locally under the sibling workspace directory `native-nonplaying-windows-20261113`. No captures were requested for this run.

The tests call adapter logic and the replay uses scripted input; neither establishes native key delivery, physical keyboard behavior or accessibility. This is not clean-machine, release or license qualification.

At checkpoint `ad6d274fd5af4a90c3b208fe80e6f4fa8fe9e6ba`, fresh locked offline debug builds of the remaining five adapters each completed `--scripted-test` with exit 0 and all 13 parsed telemetry records equal to the same reference. No captures were requested. Ash logged `Vulkan device: Intel(R) UHD Graphics 630` to stderr; the other four had empty stderr. The executable SHA-256 values were:

| Adapter | SHA-256 |
| --- | --- |
| Pixels | `f44a5517f588c603bbc5dafc34a003486bcd8307661c0e1d2463b0cd4d8dbaa6` |
| wgpu | `776b4a1bf4c5fea16ad9942527319ecc1d0e8bacf1536a6a75e5656feb58d9b1` |
| Piston | `0f2943eaaeb0054427c534ed94fdf9959523c8ff388e9e2786b7caa8fa23e796` |
| ash | `9134d1900cb4ac38547c4d5637659186a197b6d636fe82cf3d9689a34546757a` |
| softbuffer | `f916a3020e3d93e0087d947b3ab74bd5a279b496c43c5c9f1b493869d9339779` |

Executables remain in the separate target directory above; outputs and hashes remain in sibling workspace directory `native-nonplaying-five-windows-20261113`. These scripted replays do not test keyboard input or release readiness.