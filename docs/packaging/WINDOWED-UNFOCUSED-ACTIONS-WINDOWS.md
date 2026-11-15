# Windows windowed unfocused-action recheck

At checkpoint `84731994b80c217b538d2a2e6849fc5d9c93dc9e`, including `41f99b3`, Windows MSVC stable Rust 1.99.1 passed locked offline tests: Pixels 11, raw wgpu 10, ash 12, softbuffer 10 and ggez 9. ggez used release mode; the others used debug. The tested source Git blobs match [the Omarchy report](WINDOWED-UNFOCUSED-ACTIONS-OMARCHY.md). The suites include the unfocused-action boundary regression. Prior test binaries and selected ash executable artifacts were copied to sibling workspace directory `windowed-unfocused-windows-20261113` before reusing existing targets; Cargo fell back to copying incremental files when hard-linking failed.

A fresh locked offline ash debug executable had SHA-256 `16417da8e6aa77b93483b5d4d92e2f259eed02d95da71cef97ccb96afded3476`. Its isolated scripted run exited 0 and all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`; stderr identified the Intel UHD Graphics 630 Vulkan device. No captures were requested. Logs remain in the same workspace directory.

These tests call adapter boundaries and the scripted replay does not exercise real OS focus delivery or physical keys. No accessibility, license clearance, clean-machine operation or release qualification is claimed.
