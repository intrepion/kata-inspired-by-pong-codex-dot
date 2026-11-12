import test from 'node:test';
import assert from 'node:assert/strict';
import {parseArguments} from './adapters.mjs';
const base=['/tmp/binary','/tmp/new.json','backend','conditions'];
test('legacy pilot remains Macroquad',()=>assert.equal(parseArguments(base).adapter,'macroquad'));
test('softbuffer metadata is explicit',()=>{const p=parseArguments([...base,'--adapter','softbuffer']);assert.equal(p.adapter,'softbuffer');assert.match(p.renderer,/CPU/);assert.match(p.pacing,/8ms/);});
test('reject unknown, missing or duplicate adapter',()=>{for(const a of [[...base,'--adapter','unknown'],[...base,'--adapter'],[...base,'--adapter','softbuffer','--adapter','macroquad'],base.slice(1)]) assert.throws(()=>parseArguments(a));});
import {mkdtempSync,writeFileSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {runPilot} from './run.mjs';
test('runner records selected adapter over three fixture processes, not a benchmark',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-fixture-'));
  try {
    const binary=join(dir,'fixture.cjs'), output=join(dir,'results.json');
    writeFileSync(binary,'#!/usr/bin/env node\nconsole.log(\'PONG_BENCHMARK_BACKEND {"video_driver":"fixture","renderer":"fixture"}\');\nconsole.log(\'PONG_BENCHMARK '+JSON.stringify({samples:2,resized:false,sampled_seconds:10,p50_ms:5,p95_ms:5,p99_ms:5})+'\');\n',{mode:0o755});
    const options=parseArguments([process.execPath,output,'fixture-only','synthetic runner test','--adapter','softbuffer']);
    runPilot(options,[binary]);
    const data=JSON.parse(readFileSync(output,'utf8'));
    assert.equal(data.adapter,'softbuffer'); assert.equal(data.runs.length,3); assert.ok(data.runs.every(x=>x.valid)); assert.ok(data.runs.every(x=>x.reported_backend?.renderer === "fixture"));
    assert.throws(()=>runPilot(options,[binary]), /Refusing to overwrite/);
  } finally {rmSync(dir,{recursive:true,force:true});}
});

test('SDL3 metadata names pacing and does not invent backend',()=>{const p=parseArguments([...base,'--adapter','sdl3']);assert.equal(p.adapter,'sdl3');assert.match(p.renderer,/reported by app/);assert.match(p.pacing,/8ms sleep/);});

test('raylib metadata retains target pacing without claiming a GPU driver', () => {
  const config = parseArguments(['binary', 'out.json', 'unqueried', 'pilot', '--adapter', 'raylib']);
  assert.match(config.pacing, /target120fps/);
  assert.match(config.pacing, /EndDrawing/);
  assert.match(config.renderer, /unqueried/);
});

test('pixels metadata separates CPU raster from GPU presentation', () => {
  const config = parseArguments(['binary', 'out.json', 'unqueried', 'pilot', '--adapter', 'pixels']);
  assert.match(config.renderer, /CPU raster plus wgpu/);
  assert.match(config.pacing, /successful pixels.render/);
});

test('Miniquad metadata names submission boundary without asserting driver', () => {
  const config = parseArguments(['binary', 'out.json', 'unqueried', 'pilot', '--adapter', 'miniquad']);
  assert.match(config.pacing, /commit_frame/);
  assert.match(config.renderer, /driver unqueried/);
});

test('Piston metadata distinguishes render callbacks from update rate', () => {
  const config = parseArguments(['binary', 'out.json', 'unqueried', 'pilot', '--adapter', 'piston']);
  assert.match(config.pacing, /render events only/);
  assert.match(config.pacing, /120UPS/);
});

test('ggez metadata does not confuse Canvas finish with end-frame presentation',()=> { const config=parseArguments([...base,'--adapter','ggez']); assert.match(config.pacing,/before framework end_frame/); assert.match(config.pacing,/120Hz remainder sleep/); });
