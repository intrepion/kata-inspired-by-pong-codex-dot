# Independent web-controller movement sources

## Defects and correction

Dioxus, Yew and Leptos share the same session controller. It previously kept one flag per direction, so releasing W could cancel held Up. Its on-screen buttons also pretended to be ArrowUp/ArrowDown keys: a pointer release/leave could cancel a real held arrow key, and keyboard release could cancel a held button.

The controller now tracks four keyboard bindings and two movement buttons independently. A typed `Direction`/`movement_button` API separates pointer buttons from keyboard identity in all three frontends. Upper/lowercase W/S map to the same physical-letter binding. Opposite directions still cancel; blur clears all sources; scripted mode ignores button input. Game physics and rendering are unchanged.

## Verification on Omarchy ARM64, 2026-11-12

- The new keyboard overlap regression fails against the original implementation and passes after correction.
- All eight controller tests pass, including keyboard/button release ordering, opposite sources, blur, case-changing release, scripted button lockout and existing deterministic six-SVG export checks.
- Locked offline WASM checks pass for Yew, Leptos and Dioxus (Dioxus web feature, desktop default disabled).
- Strict Clippy remains **failed** on four preexisting findings: two unnecessary unwraps in replay, manual multiple check, and items after the test module. Running the original unmodified controller at the same base reproduces those findings. They were not suppressed or presented as passes.

This is controller and compile evidence, not a new browser/webview, touch-device, OS-keyboard or package pass. Browser testing must use a server and localhost, never direct files. A useful next interaction check holds an arrow, presses/releases the corresponding on-screen button, and verifies movement continues until the arrow is released. Multiple simultaneous pointers on the *same* button remain one button state; per-pointer tracking is not added here.

Existing deployment, archives and benchmark evidence remain untouched.
