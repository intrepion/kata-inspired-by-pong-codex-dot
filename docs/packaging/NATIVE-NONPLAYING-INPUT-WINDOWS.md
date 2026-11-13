# Windows native non-playing movement recheck

On Windows MSVC, checkpoint `8f6b63e067029259e76e1aa788615d536ebccecc` includes the six fixes in `e412077`. Locked offline debug tests using stable Rust 1.99.1 passed: Miniquad 11, Pixels 8, raw wgpu 7, Piston 10, ash 9 and softbuffer 7. Each suite includes the new non-playing movement regression. The tested source blob IDs match the six listed in [the Omarchy report](NATIVE-NONPLAYING-INPUT-OMARCHY.md). Tests used a separate `target-native-nonplaying-windows-20261113` target directory.

A fresh locked offline debug Miniquad build produced `pong-miniquad.exe` SHA-256 `9af8f1a1553f4e34239e3e3331242d283125f6e6584584e2c7a653801645ac9d`. Its scripted run exited 0 and all 13 parsed JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The replay output remains locally under the sibling workspace directory `native-nonplaying-windows-20261113`. No captures were requested for this run.

The tests call adapter logic and the replay uses scripted input; neither establishes native key delivery, physical keyboard behavior or accessibility. The other five rebuilt test targets were not replayed here. This is not clean-machine, release or license qualification.
