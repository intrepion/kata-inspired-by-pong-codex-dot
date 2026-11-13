# Original browser canvas drag ownership

The original browser game tracked all active canvas-drag pointers and accepted movement from any of them. A second finger could therefore redirect the paddle while the first finger was dragging. The controller regression failed before the change because the second pointer was also accepted.

A canvas drag now belongs to the first pointer until its up, cancellation, lost capture, pause or blur. Other pointers cannot move the paddle, release the owner or silently inherit it; a fresh pointer-down after release can acquire control. Only the owner receives canvas pointer capture. This is independent of the on-screen movement-button pointers, which remain separately tracked.

On Omarchy ARM64 the offline pinned TypeScript build and all 24 Node tests passed. Controller tests cover unrelated release, no inherited ownership, fresh acquisition and pause clearing; mocked DOM handlers check that a secondary touch cannot move the paddle while the first can. This is simulated event evidence, not a real browser or physical multi-touch test. Browser pointer capture/cancel ordering and device interaction remain pending; use localhost for browser testing.

Tested tree based on `2c414fb68f47b777c961bc1b79eb990148cfa309`; `game.ts` Git blob `a3460b06ee3b9a0ae160c0912327e91e003ff04d`, generated `game.js` Git blob `f42e0b0540b15cea9b3fa53ff87e8050154e2a5a`; Node `v26.12.1`, TypeScript `5.9.3`. Existing input and collision evidence is retained.
