# Original browser model: CPU contact timing

At checkpoint `75e7a9c`, two Node regressions cover a CPU paddle reaching its target just before or after the ball crosses the paddle face. The first is a tangent return once the target is reached; the second must miss because the paddle arrives too late even though its end-of-step position would overlap. The tests exercise the checked-in `game.js` model and are included in the npm test command.

On Windows Node v24.20.0, `node --test tests/cpu-contact-target.test.js tests/game.test.js` passed all 26 tests. `npm test` could not complete its preceding TypeScript build because `tsc` is not installed in this checkout; no TypeScript rebuild or browser interaction is claimed. This is model-level evidence, not physical input, real browser rendering, clean-machine operation or release qualification.
