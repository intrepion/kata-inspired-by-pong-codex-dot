# Experimental comparison source checkpoint

This branch checkpoints the Rust Pong comparisons, native Swift/Kotlin experiments, and their documented platform checks. It is not a completed release or a statement that every implementation works on every platform.

The original browser game remains unchanged. See desktop-comparison.md, web-framework-comparison.md, mobile-comparison.md and experiments/* comparison/readme files for the specific build, launch, interaction, and known-failure records. Build success is not gameplay verification. Generated builds, simulator projects, local SDK paths and web dist outputs are intentionally excluded.

Current gaps include Windows web-framework builds/gameplay, Dioxus desktop validation, incomplete platform coverage across engines and GUI toolkits, and visual gameplay for the Tauri iOS development build. The Windows validation checkout has additional local notes/fixes which must be reconciled without overwriting its existing work.

No downloadable release matrix is published by this checkpoint. Main and the hosted original game should not be replaced by this experimental branch. Run web tests through a local HTTP server, never file URLs. Prefer Windows for subsequent web testing to conserve Mac disk space.
