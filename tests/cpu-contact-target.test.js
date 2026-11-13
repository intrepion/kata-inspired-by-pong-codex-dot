const { test } = require('node:test');
const assert = require('node:assert/strict');
const { createGame } = require('../game');

test('CPU contact uses the target reached before the paddle-face crossing', () => {
    const game = createGame(() => 0.5);
    game.start();
    game.serve = 0;
    game.cpu = 250;
    game.cpuTarget = 251;
    game.cpuReaction = 1;
    game.ball = { x: 846, y: 304, vx: 350, vy: 0 };

    // The CPU reaches 251 after 1/190 seconds; the ball reaches x=848 later.
    // At contact this is exactly tangent (53 pixels), whereas interpolating
    // the end-of-step paddle displacement would put it too far away.
    game.step(1 / 120);
    assert.equal(game.cpu, 251);
    assert.ok(game.ball.vx < 0, 'the tangent shot should be returned');
});

test('CPU cannot use a target it reaches after the paddle-face crossing', () => {
    const game = createGame(() => 0.5);
    game.start();
    game.serve = 0;
    game.cpu = 250;
    game.cpuTarget = 252;
    game.cpuReaction = 1;
    game.ball = { x: 846, y: 304.2, vx: 350, vy: 0 };

    // At contact the CPU is near 251.086, short of the 251.2 needed.
    // Its later end-of-step position would incorrectly produce a hit.
    game.step(1 / 120);
    assert.ok(game.cpu > 251.2);
    assert.ok(game.ball.vx > 0, 'the shot should pass before the CPU arrives');
});