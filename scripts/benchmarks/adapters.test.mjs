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
import {spawnSync} from 'node:child_process';
test('runner records selected adapter over three fixture processes, not a benchmark',()=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-runner-fixture-'));
  try {
    const binary=join(dir,'fixture.cjs'), output=join(dir,'results.json');
    writeFileSync(binary,'#!/usr/bin/env node\nconsole.log(\'PONG_BENCHMARK '+JSON.stringify({samples:2,resized:false,sampled_seconds:10,p50_ms:5,p95_ms:5,p99_ms:5})+'\');\n',{mode:0o755});
    const r=spawnSync(process.execPath,['scripts/benchmarks/run.mjs',binary,output,'fixture-only','synthetic runner test','--adapter','softbuffer'],{encoding:'utf8'});
    assert.equal(r.status,0,r.stderr); const data=JSON.parse(readFileSync(output,'utf8'));
    assert.equal(data.adapter,'softbuffer'); assert.equal(data.runs.length,3); assert.ok(data.runs.every(x=>x.valid));
    const duplicate=spawnSync(process.execPath,['scripts/benchmarks/run.mjs',binary,output,'fixture-only','synthetic runner test','--adapter','softbuffer']);
    assert.notEqual(duplicate.status,0);
  } finally {rmSync(dir,{recursive:true,force:true});}
});
