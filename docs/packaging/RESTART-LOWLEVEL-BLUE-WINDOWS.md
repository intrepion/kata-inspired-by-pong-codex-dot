# Windows ash, softbuffer and Blue Engine active-restart recheck

At checkpoint `0800a0b2b0502d2ee0695a66c70024ed8574ab6c`, Windows MSVC stable Rust 1.99.1 passed locked offline tests for ash (11), softbuffer (9) and Blue Engine (8), including their active-restart held-input regressions. Ash and softbuffer used debug mode; Blue Engine used release. Their source Git blobs match [the Omarchy report](RESTART-LOWLEVEL-BLUE-OMARCHY.md). Cargo fell back to copying incremental files where hard-linking failed. Prior test binaries and selected ash executable artifacts were copied to sibling workspace directory `lowlevel-blue-restart-windows-20261113/prior` before reusing existing targets.

A fresh ash debug executable built offline with SHA-256 `a85bd24ecdbd87a0f83b3f10323038b15d30bcdce719bcd972bca89887dc7658`. Its isolated scripted run exited 0 and all 13 parsed records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`; stderr identified the Intel UHD Graphics 630 Vulkan device. No captures were requested. Logs remain in the sibling workspace directory above.

The tests call adapter logic, while the standard replay does not exercise a held key through restart. SDL3 was not retried after its native build cache blocker. Physical input, accessibility, clean-machine operation and release qualification remain open.
