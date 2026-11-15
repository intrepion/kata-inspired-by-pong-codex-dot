# Original browser movement-button synthesized activation

The on-screen movement buttons previously reacted to pointer holds and physical Space/Enter holds, but a synthesized `click` alone did not move the paddle. The mocked-handler regression failed before this change. During play, a zero-detail click now moves the corresponding paddle direction by 32 court units, clamped to the court. Pointer clicks with nonzero detail do not add an extra increment, and activation while paused is ignored. The 32-unit increment is a discrete control choice, not simulated hold duration.

The offline pinned TypeScript build and all 19 Node tests passed on Omarchy. Mocked handler tests cover a zero-detail click, a nonzero-detail click, repeated clicks at a boundary and paused activation, in addition to keyboard and pointer cases. This models event detail and dispatch but does not establish how a particular browser, assistive technology or touch device emits events. Real browser, screen-reader, physical input and cross-platform validation remain open; use localhost for browser tests.

Tested tree based on `3f50a3d805d0b734266679853fa8406a06d1c466`; `game.ts` Git blob `d98e8f36d10835e3dc5396afb093318b939cec9a`, generated `game.js` Git blob `d0dc3c5978f69b67ff079dd05d9ed3d619202cb7`; Node `v26.12.1`, TypeScript `5.9.3`. Earlier keyboard-hold evidence is in [the preceding fix](ORIGINAL-MOVEMENT-BUTTON-KEYS.md).

## Windows model recheck

At checkpoint `162ad16b968c179a8ce77c6aa2b29a4f20e62fec`, Node v24.20.0 passed all 19 checked-in JavaScript tests, including the mocked zero-detail activation assertions. The source and generated JavaScript blobs matched the hashes above; this run did not rebuild TypeScript. The supported browser check on the preceding revision exercised focused Space and Enter holds, not synthesized activation. A normal pointer click does not establish the zero-detail assistive-activation path. Browser delivery of a synthesized activation, screen-reader behavior and physical input remain unverified.
