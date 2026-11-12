# Original TypeScript portable scripted replay

```sh
npm ci
npm test
PORT=5249 npm run serve
# Open http://127.0.0.1:5249/?scripted-test through a supported browser.
```

The adapter fetches [the portable manifest](../scenarios/smoke-v1.txt), applies its actions through the original TypeScript game, and renders through the ordinary Canvas2D draw function. Real gameplay handlers are ignored during demo mode. At each requested tick the source exports its own canvas via `toDataURL`, then at completion offers a JSON download containing state, actions and six PNG data URLs. The canvas excludes DOM score/overlay/buttons; it is not a full browser or OS screenshot. `game.js` remains generated from `game.ts` and is checked in. Normal play is unchanged.

## Verified on Omarchy, 2026-11-12

- TypeScript build and 15 Node tests pass (12 existing gameplay tests, 3 new portable replay/parser/pacing tests).
- 16 Rust core/scenario/WASM tests pass, including a new exact comparison of all thirteen old smoke-v1 baseline lines after moving the timeline into the manifest. Strict scenario Clippy passes.
- Original TypeScript replay produces thirteen deterministic records with the manifest's four actions and six checkpoint ticks; player boundaries, frozen pause, resume, reset and terminal no-op are asserted.
- The local HTTP server returns the manifest with HTTP200 and exact bytes. The capture-site workflow copies the scenario directory too.
- Browser UI/canvas export verification is **pending**: the supported in-app browser was unavailable and browser inventory was empty. No Playwright/OS workaround, file URL, fabricated PNG or screenshot-pass claim is made. Node telemetry does not establish visual correctness or that download links work in a real browser.

TS trajectories intentionally differ from Rust: the demo uses a stable TS-only 32-bit RNG and the original physics/f64 arithmetic. This is an action/timing comparison, not Rust-state equivalence.

[TS Node state evidence](evidence/original-ts-portable-replay-20261112.jsonl) · [Manifest/actions and verification summary](evidence/original-ts-portable-replay-20261112.json) · [Portable format](../scenarios/FORMAT.md)
