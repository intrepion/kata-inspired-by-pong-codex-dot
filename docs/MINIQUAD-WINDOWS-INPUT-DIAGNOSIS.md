# Miniquad Windows native-input diagnosis

## What was observed

On 2026-11-12 the fresh GNU release was opened as a normal native window. Supported Computer Use selected the exact window, reported it focused, and successfully clicked the court and closed the window. Return did not start play. Clicking the court and sending Space also left the ball centered in Ready. This reproduces the earlier symptom rather than treating scripted replay as native input acceptance.

An opt-in --trace-input flag now records engine key-down/up enum values, repeat status, game state and mouse-button-down identity to stderr. It does not inject events, change key mapping, reinterpret Unknown, enable input outside the game or capture text from other applications. Without the flag it emits no input traces.

The instrumented window received each requested Return, Space, R and Up as key_down Unknown (repeat=false) followed by key_up Unknown, with state Ready throughout. A court click produced mouse_down Left. Both diagnostic windows were closed through their normal Close control. No process remains.

## Interpretation and limits

The game's Enter/Space start branches never received their expected key enum in this supported automation path. Inspection of pinned Miniquad 0.4.11 Windows backend shows its WM_KEYDOWN/WM_KEYUP path translating scan-code bits from lParam rather than the virtual key. This narrows the failure to key delivery/translation before app action handling; it does not prove which component supplied the unexpected scan code, nor that a physical keyboard fails. Real keyboard/full-match acceptance remains open. No synthetic-input workaround or Unknown-as-Enter behavior was added.

Five release tests and strict all-target release Clippy pass. A fresh native scripted replay remains separately checked, not a replacement for real input. Existing portable and benchmark hashes are historical because the opt-in diagnostic adds a new binary. The traced events are limited to the intentionally requested game test keys; logs remain local.

Fresh scripted run: exit 0, thirteen state reports through finished tick 1440, six app captures and no input-trace output without the flag. [Sanitized event summary and binary hash](evidence/miniquad-windows-input-diagnosis-20261112.json).

## Action boundary verification

The existing key-action match was extracted without changing its behavior so it can be exercised without a GPU window. Two additional tests verify mapped Enter/R/Space start correctly; Space pauses/resumes; R clears scores; repeated action keys do not trigger actions; Unknown never starts; Escape requests quit. All seven release tests and strict Clippy pass. The fresh rebuilt replay is byte-identical across all thirteen state records to the pre-extraction replay and has six PNGs. This validates app behavior for correctly mapped keys, not the unresolved supported-input translation. [New binary/result record](evidence/windows-action-boundary-followup-20261112.json).
