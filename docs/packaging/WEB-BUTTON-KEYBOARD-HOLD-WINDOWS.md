# Windows web movement button keyboard-hold recheck

At checkpoint `963fae5dec7a113e919d2a7bd8e5706aa9250136`, including `060f2ab`, Windows MSVC stable Rust 1.99.1 passed all 17 locked offline shared web controller tests, including keyboard hold independence, blur, pause and restart. The shared and Yew, Leptos and Dioxus source Git blobs match [the Omarchy report](WEB-BUTTON-KEYBOARD-HOLD-OMARCHY.md).

Locked offline debug WebAssembly builds succeeded for all three consumers (Dioxus with `--no-default-features --features web`). wasm-bindgen 0.2.130 generated web-target bindings. Browser-ready `_bg.wasm` SHA-256 values:

| Frontend | SHA-256 |
| --- | --- |
| Yew | `9ea14835d19beccc82b4f6d44b0090045c6208f8326d68369a3f073bb0990fe4` |
| Leptos | `8c5ec65def5cecd4854929a7759c91ca15419f5235b0a6447d7153fc33eedb3f` |
| Dioxus | `91463af1a870907dbb1988c77160488f646adeee8dda1a88fdc231b75802d8fc` |

Cargo fell back to copying incremental cache files when hard-linking failed. The isolated model and copied build targets remain under `target-web-keyhold-model-windows-20261113` and `target-web-keyhold-windows-20261113`; generated bindings remain in sibling workspace directory `web-keyhold-windows-20261113`. These are model and build results. The computer-use skill's required guidance resource remained unavailable, so Windows browser interaction, physical and assistive input, native wrappers and release qualification remain open.
