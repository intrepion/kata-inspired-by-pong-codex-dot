# Blue Engine focus-return action gate: Windows cross-check

At checkpoint `fd3653f7aa6c5061bb227de4dfb25c481a6cbe11`, Blue Engine adapter blob `aa64770f5776007336bd0827f42715354d15be63` passed all 11 locked offline Windows stable MSVC debug tests, including the synthetic focus-return held-action regression. The test covers Enter, R, Space and Escape suppression, release and fresh presses. Cargo reported incremental hard-link failures and copied files instead.

The rebuilt windowed debug executable SHA-256 was `ae530bd35f68624e49ed2c7b6e91cb0bb1e28fb5afa753732154d593891de3dc`. Its app-owned scripted run exited 0 and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stderr contained graphics-loader diagnostics including a missing Vulkan validation-layer warning. Prior executables, tests and local logs remain in sibling workspace directory `blue-focus-return-windows-20261113`.

The gate deliberately suppresses keys pressed on the first focused sample until released, including a genuinely new first-sample press. The regression supplies synthetic polling values and the replay does not exercise OS focus or keyboard delivery. Physical input, native key-state ordering, complete interactive matches, clean-machine operation, license clearance and release qualification remain open.
