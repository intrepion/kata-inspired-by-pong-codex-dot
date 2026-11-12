const { test } = require('node:test');
const assert = require('node:assert/strict');
const { createGame, H, PADDLE_H } = require('../game');
test('start, movement and bounds', () => { const g = createGame(() => .2); g.start(); assert.equal(g.state, 'playing'); for (let i = 0; i < 100; i++)
    g.step(.033, -1); assert.equal(g.player, PADDLE_H / 2); });
test('paddle reflects ball and steers angle', () => { const g = createGame(() => .2); g.start(); g.serve = 0; g.player = H / 2; g.ball = { x: 60, y: H / 2 + 25, vx: -350, vy: 0 }; g.step(.033); assert.ok(g.ball.vx > 0); assert.ok(g.ball.vy > 0); });
test('wall reflects and a miss scores, then match ends at seven', () => { const g = createGame(() => .2); g.start(); g.serve = 0; g.ball = { x: 450, y: 9, vx: 300, vy: -200 }; g.step(.033); assert.ok(g.ball.vy > 0); g.scores = [6, 0]; g.ball = { x: 905, y: 20, vx: 350, vy: 0 }; g.step(.033); assert.deepEqual(g.scores, [7, 0]); assert.equal(g.state, 'over'); g.start(); assert.deepEqual(g.scores, [0, 0]); });
test('paused game does not advance', () => { const g = createGame(() => .2); g.start(); g.state = 'paused'; const x = g.ball.x; g.step(.033, 1); assert.equal(g.ball.x, x); assert.equal(g.player, H / 2); });
test('computer reacts after a delay and moves at a finite speed', () => { const g = createGame(() => .5); g.start(); g.serve = 0; g.ball = { x: 500, y: 400, vx: 350, vy: 0 }; const initial = g.cpu; g.step(.033); assert.equal(g.cpu, initial); for (let i = 0; i < 10; i++)
    g.step(.033); assert.ok(g.cpu > initial); assert.ok(g.cpu - initial <= 190 * .363 + 1e-9); });
test('computer returns an accessible shot and misses a fast distant shot', () => { const easy = createGame(() => .5); easy.start(); easy.serve = 0; easy.ball = { x: 650, y: H / 2, vx: 350, vy: 0 }; for (let i = 0; i < 40 && easy.ball.vx > 0; i++)
    easy.step(.033); assert.ok(easy.ball.vx < 0, 'center shot should be returned'); const hard = createGame(() => .5); hard.start(); hard.serve = 0; hard.ball = { x: 750, y: 50, vx: 700, vy: 0 }; for (let i = 0; i < 20 && !hard.scores[0]; i++)
    hard.step(.033); assert.equal(hard.scores[0], 1, 'distant fast shot should score'); });
test('combined controls cannot exceed normal paddle speed', () => { for (const input of [-2, 2]) {
    const g = createGame(() => .5);
    g.start();
    g.step(.016, input);
    assert.ok(Math.abs(g.player - (250 + Math.sign(input) * 7.52)) < 1e-9);
} });
test('nonfinite time or input leaves the simulation unchanged', () => { for (const [dt, input] of [[NaN, 0], [Infinity, 0], [-Infinity, 0], [.016, NaN], [.016, Infinity], [.016, -Infinity]]) {
    const g = createGame(() => .5);
    g.start();
    const before = JSON.stringify(g);
    g.step(dt, input);
    assert.equal(JSON.stringify(g), before);
} });
test('frame loop preserves one second of gameplay at 20, 60 and 120 Hz', () => { const { createFrameLoop } = require('../game'); for (const hz of [20, 60, 120]) {
    const g = createGame(() => .5);
    g.start();
    g.serve = 0;
    const loop = createFrameLoop(g);
    for (let i = 0; i <= hz; i++)
        loop.advance(i * 1000 / hz);
    assert.ok(Math.abs(g.ball.x - 800) < 1e-8, `${hz} Hz: ${g.ball.x}`);
} });
test('long stalls catch up no more than 100 ms per frame', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(5000); assert.ok(Math.abs(g.ball.x - 485) < 1e-8); loop.advance(5000 + 1000 / 120); assert.ok(Math.abs(g.ball.x - 487.9166666667) < 1e-8); });
test('reset discards partial steps and elapsed pause time', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(4); loop.reset(); loop.advance(10000); loop.advance(10004); assert.equal(g.ball.x, 450); loop.advance(10000 + 1000 / 120); assert.ok(Math.abs(g.ball.x - 452.9166666667) < 1e-8); });
test('invalid frame timestamps and controls do not poison the clock', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(NaN); loop.advance(Infinity); loop.advance(-10); loop.advance(20, NaN); loop.advance(1000 / 120); assert.ok(Math.abs(g.ball.x - 452.9166666667) < 1e-8); });
