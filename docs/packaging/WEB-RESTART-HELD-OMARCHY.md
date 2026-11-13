# Web restart held-input regression — Omarchy, 2026-11-13

Yew, Leptos and Dioxus share `pong-web-ui-common::Session`. Starting a new game while already playing previously reset the court but retained movement held through keyboard bindings, keyboard-operated movement buttons, or pointer-owned movement buttons. The start operation now clears all three sources, as it already did when restarting after game over. A fresh press is required; repeated keydown cannot reacquire the cleared keyboard hold.

A regression failed before the change with W still held after restart, and passed afterward. The test checks all three held-source stores, verifies that repeat events do not move the new paddle, and confirms that a fresh W press does. All 18 shared-controller tests passed with `cargo test --offline --locked --manifest-path experiments/web-ui-common/Cargo.toml`; locked offline WASM builds passed for Yew, Leptos, and Dioxus web. The shared source Git blob is `1dadb21015592126e9188189c97f778aa4a3dac6` (including the regression). The final test-only addition does not alter the built production code.

This is shared-model and compilation evidence. Restart while a physical key or pointer remains held was not exercised in an actual browser or desktop wrapper; physical input and release qualification remain open. Historical browser evidence is retained separately.
