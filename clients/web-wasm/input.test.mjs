import test from 'node:test';
import assert from 'node:assert/strict';
import { MovementInput } from './input.mjs';

test('nonplaying movement and repeats cannot rearm on start', () => {
  for (const key of ['w', 'ArrowUp', 's', 'ArrowDown']) {
    const input = new MovementInput();
    input.key(key, true, false, false);
    input.key(key, true, true, true);
    assert.equal(input.axis(), 0, key);
    input.key(key, false, false, true);
    input.key(key, true, false, true);
    assert.notEqual(input.axis(), 0, key);
  }
  for (const direction of [-1, 1]) {
    const input = new MovementInput();
    input.pointer(1, direction, true, false);
    assert.equal(input.axis(), 0);
    input.pointer(1, direction, false, true);
    input.pointer(2, direction, true, true);
    assert.equal(input.axis(), direction);
  }
});
test('pointer ownership and independent sources survive unrelated release', () => {
  const input = new MovementInput();
  input.pointer(1, -1, true, true); input.pointer(2, -1, true, true);
  input.pointer(1, -1, false, true); assert.equal(input.axis(), -1);
  input.key('s', true, false, true); assert.equal(input.axis(), 0);
  input.pointer(99, -1, false, true); assert.equal(input.axis(), 0);
  input.pointer(2, -1, false, true); assert.equal(input.axis(), 1);
  input.clear(); assert.equal(input.axis(), 0);
  input.key('s', true, true, true); assert.equal(input.axis(), 0);
});
