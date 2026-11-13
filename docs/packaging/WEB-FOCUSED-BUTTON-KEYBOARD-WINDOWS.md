# Windows focused web button keyboard recheck

At checkpoint `1101a127855b9d59078b1d1ab8ab8e13a01d3646`, Windows MSVC stable Rust 1.99.1 passed all 16 locked offline shared web controller host tests, including `native_activation_is_limited_to_enter_and_space`. The tested controller source blob `67e6d4bb34980a49c1a1d38effb6fb3b2df686cf` and Yew, Leptos and Dioxus source blobs match [the Omarchy report](WEB-FOCUSED-BUTTON-KEYBOARD-OMARCHY.md). Cargo fell back to copying incremental cache files where hard-linking was unavailable.

Locked offline debug `wasm32-unknown-unknown` builds succeeded for Yew, Leptos and Dioxus (the latter with `--no-default-features --features web`). Existing wasm-bindgen 0.2.130 generated web-target bindings successfully. Browser-ready `_bg.wasm` SHA-256 values:

| Frontend | SHA-256 |
| --- | --- |
| Yew | `d03c6f1d03765842046f6b214eb1bd26c32f1ddd025517399d354bc7b8e40547` |
| Leptos | `d2761796c82670bae110ea069844561c0461595e353a7404cb88f2073b733e3c` |
| Dioxus | `251a76522f5b86c1dd0681e11279166bbff86c52fd9095b145d9e91024ee92b5` |

The copied target remains at `target-web-focuskey-windows-20261113`; generated bindings remain in sibling workspace directory `web-focuskey-windows-20261113`. These are build and model results. The computer-use skill's required guidance resource could not be read in this environment, so no Windows browser interaction is claimed. Physical keyboard, pointer and assistive input, native wrappers, clean-machine operation and release qualification remain open.
