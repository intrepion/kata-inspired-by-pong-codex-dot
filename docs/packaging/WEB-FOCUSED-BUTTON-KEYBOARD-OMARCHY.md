# Focused web button keyboard activation

2026-11-13, Omarchy ARM64. In the Yew localhost build before the fix, Space on the focused Start button left the game in Ready, and Enter on the focused Pause button left it Playing: the main element intercepted Enter/Space, prevented native activation and ran its game shortcut. Source inspection showed the same bubbling structure in Leptos and Dioxus. The pre-fix Yew browser-ready WASM SHA-256 was `ae7c03e2c29b762c2ebef7cbbe2b7d60b0ca395d362a6b393db5e24031dc1722`.

Yew, Leptos and Dioxus now stop Enter/Space keydown propagation at focused buttons and the link without preventing their native default activation. The court/root shortcut remains available. A shared helper restricts this behavior to Enter and Space; all 16 shared-controller host tests pass. The movement Hold buttons remain pointer-driven: this change prevents their Enter/Space keys from accidentally triggering the global shortcuts, but does not implement keyboard-held movement on those buttons. W/S and arrow controls remain separately available at the game root.

Fresh locked offline debug WASM builds and wasm-bindgen 0.2.130 outputs were served from `127.0.0.1` in the in-app browser. For each of Yew, Leptos and Dioxus, supported automated Space on Start entered play, Enter on Pause paused, and Space on Pause resumed; Yew and Leptos visibly retained the 1–0 score across pause/resume, as did Dioxus. In the Dioxus check, Space on the focused Hold button did not pause, and Space on the focused game root did pause. These are browser automation checks, not physical keyboard, touchscreen, or native-wrapper verification. Dioxus desktop passed a locked offline `cargo check`; no desktop binary or interaction is claimed.

| Component | Tested source Git blob | Browser-ready WASM SHA-256 |
| --- | --- | --- |
| Yew | `a258aa6e42848e7f2e9fdee5a1e70bb6b4eec949` | `a29d41a7a06fd377215dbab65e42e1e3d22ba37858c98a8f13c21948d708eaef` |
| Leptos | `58403a8337da40c9bc75afd7f04586f548238881` | `8650d28054f198b6db131e2b614c6610c6dddd2a90d1077d5e0f9bc455058d1b` |
| Dioxus | `b7b5a1b7995fa8af84f38395d94a834039ff0196` | `1a30f3dcc1587f3a8315a837825dd9c3cc02bed09131d030f14512c54ab2c38a` |

The shared controller source Git blob is `67e6d4bb34980a49c1a1d38effb6fb3b2df686cf`; the wasm-bindgen executable SHA-256 is `cb06762193f62126cf95b6ca811b8976ed2c5250ca2e06e45e33ccab9759de4b`. Isolated before/after artifacts and copied targets remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-web-button-key-20261113-OQhynV`. The localhost server was stopped. Other browsers, native shells, physical input, keyboard-accessible Hold behavior, dependency/notice review and release qualification remain open; prior artifacts were retained.
