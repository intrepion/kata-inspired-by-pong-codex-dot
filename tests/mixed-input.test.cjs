const test = require('node:test');
const assert = require('node:assert/strict');
const {createControls, createGame} = require('../game.js');

test('opposite mixed input sources cancel until all of a direction are released', () => {
  const controls = createControls();
  controls.buttonDown(1, -1); controls.buttonDown(2, 1); controls.keyDown('w', false);
  assert.equal(controls.axis(), 0);
  controls.buttonUp(1); assert.equal(controls.axis(), 0);
  controls.keyUp('w'); assert.equal(controls.axis(), 1);
  controls.buttonUp(2); assert.equal(controls.axis(), 0);
});
test('same-direction inputs remain bounded and suspension clears every source', () => {
  const controls = createControls();
  controls.buttonDown(1, 1); controls.keyDown('s', false); assert.equal(controls.axis(), 1);
  controls.keyDown('ArrowUp', false); assert.equal(controls.axis(), 0);
  controls.suspend(); assert.equal(controls.axis(), 0);
});
test('opposite sources cancel in gameplay and movement resumes after release', () => {
  const controls = createControls(), game = createGame(() => .5);
  game.start(); controls.buttonDown(1, -1); controls.buttonDown(2, 1); controls.keyDown('w', false);
  const before = game.player; game.step(.01, controls.axis()); assert.equal(game.player, before);
  controls.buttonUp(2); game.step(.01, controls.axis()); assert.ok(game.player < before);
});
