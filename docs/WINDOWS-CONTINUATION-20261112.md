# Windows continuation checkpoint, 2026-11-12 06:42 UTC

Source checkout: `734f175`; clean before checks. Machine: DESKTOP-HEL7V0H.

## Passed locally

- Existing pinned TypeScript compiler: `node node_modules/typescript/bin/tsc --project tsconfig.json`, exit 0.
- Generated `game.js` unchanged (`git diff --exit-code -- game.js`, exit 0).
- `node --test tests/game.test.js`: 15/15 passed.
- `node --test tests/wasm.test.mjs`: 2/2 compiled-WASM ABI tests passed against the existing local artifact, not a fresh WASM build.

## New access blockers, not game failures

- Fetching the checkpoint branch failed: network policy blocked `https://github.com:443`. Latest remote source was not obtained; no push attempted.
- `npm test` execution was interrupted by a policy block to `https://registry.npmjs.org:443`. The existing local compiler and Node tests above ran directly without network and passed.
- Existing raw-wgpu native executable opened its Ready window. The subsequent exec session failed with a policy block to `https://dc.services.visualstudio.com:443`; the window disappeared before input could be verified. No new native gameplay pass is claimed and the endpoint's source has not been established.
- No policy settings, credentials, browser settings, or system settings changed. No alternate network route was attempted.

No Pong process was left running after this bounded attempt. Existing Android emulator remains available. Continuing native checks and latest-source benchmarks requires resolving the execution/network interruption; earlier runtime evidence remains separately recorded, not invalidated by this attempt.

## Access restored and fresh checks, 2026-11-12 07:18 UTC

GitHub ls-remote/fetch and npm registry HEAD succeeded after the user reported updating Chrome extension access. This correlation does not establish which setting restored command networking. Localhost Chrome Start/Pause worked; test tab/server closed.

Merged remote source40c40f2 without discarding prior local evidence. Fresh npm test passed15/15; generated JS unchanged. Shared Rust core9/9, benchmark3/3 and scenario replay5/5 passed. The portable scenario snapshot test FAILED: Windows GNU floating-point velocity at tick600 was -368.4489/-25.659891 versus Omarchy -368.44888/-25.65989. Byte-identical cross-platform floating-point replay is not established; this assertion remains failing, not hidden or weakened.

Benchmark runner fixture initially failed on Windows because Windows cannot directly execute its Unix-shebang .cjs fixture. Extracted the callable runner and invoked the fixture with the actual Node executable, without shell dispatch; all5 adapter/runner tests now pass. Production CLI still executes the real binary with --benchmark.

Fresh Macroquad release build passed. Three bounded Windows benchmark repeats completed with1440 samples each, stable800x544 logical dimensions, p50 approximately6.94ms. This is callback cadence, not displayed FPS, GPU timing, input validation, or a framework/OS ranking. See [pilot report](benchmarks/WINDOWS-MACROQUAD-PILOT.md). All benchmark processes exited.
