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
import {runPilot,validSample} from './run.mjs';
test('runner records selected adapter over three fixture processes, not a benchmark',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-fixture-'));
  try {
    const binary=join(dir,'fixture.cjs'), output=join(dir,'results.json');
    writeFileSync(binary,'#!/usr/bin/env node\nconsole.log(\'PONG_BENCHMARK_BACKEND {"video_driver":"fixture","renderer":"fixture"}\');\nconsole.log(\'PONG_BENCHMARK '+JSON.stringify({schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,nominal_measure_seconds:10,samples:2,resized:false,sampled_seconds:10,mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,width:900,height:500,intervals_ms:[5000,5000]})+'\');\n',{mode:0o755});
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

test('nannou metadata states recording rather than presentation boundary',()=>{const config=parseArguments([...base,'--adapter','nannou']);assert.match(config.pacing,/before rendering/);assert.match(config.renderer,/Bevy/);});

test('raw wgpu metadata excludes skipped surface frames',()=>{const config=parseArguments([...base,'--adapter','wgpu']);assert.match(config.pacing,/successful surface present request only/);assert.match(config.pacing,/default surface config/);});

test('bracket-lib distinguishes graphical tiles from terminal and retained cap',()=>{const config=parseArguments([...base,'--adapter','bracket-lib']);assert.match(config.presentation,/not terminal/);assert.match(config.pacing,/60fps cap/);});

test('Bevy metadata labels main-world frame callback scope',()=>{const config=parseArguments([...base,'--adapter','bevy']);assert.match(config.pacing,/main-world Last/);assert.match(config.pacing,/not GPU timing/);});

test('Blue Engine timing excludes headless mode and identifies upload boundary',()=>{const config=parseArguments([...base,'--adapter','blue-engine']);assert.match(config.presentation,/headless excluded/);assert.match(config.pacing,/object upload precedes/);});

test("Fyrox callback metadata excludes completed GPU timing",()=>{const r=parseArguments(["binary","new.json","backend","notes","--adapter","fyrox"]); assert.match(r.pacing,/before_rendering/);});

test("ash timing names serialized FIFO and excludes skipped frames",()=>{const r=parseArguments(["binary","new.json","backend","notes","--adapter","ash"]); assert.match(r.pacing,/FIFO/);assert.match(r.pacing,/successful present-request/);});

for(const [label,body,exit,signal] of [
  ['nonzero exit','process.exit(7)',7,null],
  ['signal termination','process.kill(process.pid,"SIGTERM")',null,'SIGTERM'],
]) test(`runner retains ${label} and rejects emitted samples`,{skip:signal!==null && process.platform==='win32'},()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-failure-'));
  try {
    const fixture=join(dir,'fixture.cjs'),output=join(dir,'results.json');
    const sample={schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,nominal_measure_seconds:10,samples:2,resized:false,sampled_seconds:10,mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,width:900,height:500,intervals_ms:[5000,5000]};
    writeFileSync(fixture,`require('node:fs').writeSync(1,${JSON.stringify('PONG_BENCHMARK '+JSON.stringify(sample)+'\n')});${body};`);
    const data=runPilot(parseArguments([process.execPath,output,'fixture','not a benchmark']),[fixture]);
    assert.equal(data.runs.length,1);
    assert.equal(data.runs[0].exit,exit);
    assert.equal(data.runs[0].signal,signal);
    assert.equal(data.runs[0].sample.samples,2);
    assert.equal(data.runs[0].valid,false);
  } finally {rmSync(dir,{recursive:true,force:true});}
});

test('runner rejects incomplete or inconsistent samples instead of marking them valid',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-invalid-'));
  try {
    const fixture=join(dir,'fixture.cjs'),output=join(dir,'results.json');
    writeFileSync(fixture,`console.log('PONG_BENCHMARK '+JSON.stringify({samples:2,sampled_seconds:10}));`);
    const data=runPilot(parseArguments([process.execPath,output,'fixture','not a benchmark']),[fixture]);
    assert.equal(data.runs.length,3);
    assert.ok(data.runs.every(r=>r.valid===false));
    assert.equal(data.runs[0].sample.samples,2);
  } finally {rmSync(dir,{recursive:true,force:true});}
});

test('sample validation rejects resized, mismatched, or non-finite measurements',()=>{
  const sample={schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,nominal_measure_seconds:10,samples:2,resized:false,sampled_seconds:10,mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,width:900,height:500,intervals_ms:[5000,5000]};
  assert.equal(validSample(sample),true);
  for(const change of [{resized:true},{samples:3},{width:0},{p95_ms:NaN},{intervals_ms:[5000,-1]},{metric:'other'}]) {
    assert.equal(validSample({...sample,...change}),false);
  }
});

test('runner retains malformed sample and backend output as invalid evidence',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-malformed-'));
  try {
    const fixture=join(dir,'fixture.cjs'),output=join(dir,'results.json');
    writeFileSync(fixture,`console.log('PONG_BENCHMARK {bad'); console.log('PONG_BENCHMARK_BACKEND {bad');`);
    const data=runPilot(parseArguments([process.execPath,output,'fixture','not a benchmark']),[fixture]);
    assert.equal(data.runs.length,3);
    assert.ok(data.runs.every(r=>!r.valid && r.parse_errors.sample && r.parse_errors.backend));
    assert.equal(JSON.parse(readFileSync(output,'utf8')).runs[0].raw_sample,'{bad');
  } finally {rmSync(dir,{recursive:true,force:true});}
});

test('malformed backend invalidates an otherwise complete sample',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-backend-'));
  try {
    const fixture=join(dir,'fixture.cjs'),output=join(dir,'results.json');
    const sample={schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,nominal_measure_seconds:10,samples:2,resized:false,sampled_seconds:10,mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,width:900,height:500,intervals_ms:[5000,5000]};
    writeFileSync(fixture,`console.log('PONG_BENCHMARK '+${JSON.stringify(JSON.stringify(sample))}); console.log('PONG_BENCHMARK_BACKEND {bad');`);
    const data=runPilot(parseArguments([process.execPath,output,'fixture','not a benchmark']),[fixture]);
    assert.ok(data.runs.every(r=>!r.valid && r.parse_errors.backend && r.sample.samples===2));
  } finally {rmSync(dir,{recursive:true,force:true});}
});
