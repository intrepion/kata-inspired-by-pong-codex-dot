# Pixels Mac resize and focus attempts (2026-11-13)

Using the release binary documented in `PIXELS-COURT-NATIVE-MAC-20261113.md` (SHA-256 `49a879c01da5be7abf8acc7d236a45d97032c17dc29e691621e0015c1e5cb0b2`), further desktop-automation drags at the right and lower window edges left the screenshot dimensions unchanged. This follows the unsuccessful corner drags recorded in `PIXELS-MOVEMENT-NATIVE-MAC-20261113.md`. The source creates a winit window without explicitly disabling resizing and handles nonzero `WindowEvent::Resized` via `resize_surface`; the unsuccessful gesture alone does not identify whether the app, OS, or automation caused the outcome.

During play, selecting Finder through the automation API and sending Command-Tab did not establish a confirmed loss of focus. The game was still observed playing afterward; that observation is not evidence of a focus-loss defect because the focus transition itself was not verified.

A confirmed native resize and confirmed focus-loss/pause test remain open. No code changes or physical-input or release qualification claims result from these attempts.
