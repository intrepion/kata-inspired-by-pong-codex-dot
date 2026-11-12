# Shared Yew / Leptos / Dioxus web replay

Serve a built browser client via localhost and add `?scripted-test` to its URL.
The shared `Session` selects `pong-scenario::Scenario`, seed 42, fixed 120 Hz;
normal input handlers cannot interfere. At tick 1440 the court stops and links
appear for 13 telemetry records (JSONL) and six SVG scene exports.

These are app-owned SVG markup from the exact common court renderer, not
raster screenshots or evidence that the browser presented those frames.
They exclude surrounding framework DOM. They do not verify physical inputs,
continuous animation, compositor behavior, packaging or performance. No data
is uploaded; download links use local data URLs. Native Dioxus does not opt in
from a query string and is unchanged. The library exposes `enable_demo()` for
an explicit native adapter if one is implemented later.

The shared Rust unit test verifies all 13 reports, exact final reference state,
six checkpoint ticks, bounded completion, and immunity to gameplay handlers.
Browser integration/export links still require a localhost UI verification.
