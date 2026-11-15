# Windows raw WASM contact-timing recheck

On 2026-11-13, raw WASM was built in release mode with locked offline dependencies from `fe812c35e11ec386ac353954923f2179b181943b`. The WASM SHA-256 was `1324cd2935611280582fa5ca294f5dde03b1661dfa82d10d2b965b324d590d58`. Both Node ABI tests passed; they test compiled-WASM state behavior, not browser or physical input.

Served over localhost in Chrome, a separate normal session reached Playing, Paused, then Playing using the visible Start and Pause/Resume buttons. This is a bounded browser interaction check, not sustained movement, touch, full match or native-wrapper qualification.

The separate scripted browser session advanced through pause and restart and displayed its completion download link. The final court was visually inspected; the generated bundle and its telemetry and Canvas2D images were not downloaded or compared. Scripted execution and app-owned captures do not establish physical input, compositor behavior, packaging or release qualification.
