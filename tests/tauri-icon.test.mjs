import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

test('Windows Tauri icon contains the tracked PNG asset',()=>{
  const png=readFileSync(new URL('../clients/tauri/icons/icon.png',import.meta.url));
  const ico=readFileSync(new URL('../clients/tauri/icons/icon.ico',import.meta.url));
  assert.equal(ico.readUInt16LE(0),0);
  assert.equal(ico.readUInt16LE(2),1);
  assert.equal(ico.readUInt16LE(4),1);
  assert.equal(ico[6],png.readUInt32BE(16));
  assert.equal(ico[7],png.readUInt32BE(20));
  assert.equal(ico.readUInt32LE(14),png.length);
  assert.equal(ico.readUInt32LE(18),22);
  assert.deepEqual(ico.subarray(22),png);
});
