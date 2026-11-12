// Shared bounded pilot runner. Node built-ins only. Run an already-built binary.
import {spawnSync} from 'node:child_process';
import {writeFileSync, readFileSync, existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import os from 'node:os';
import {parseArguments} from './adapters.mjs';
const {binary, output, backend, notes, adapter, presentation, renderer, pacing} = parseArguments(process.argv.slice(2));
if (existsSync(output)) throw Error('Refusing to overwrite results');
const command = (exe,args) => { const r=spawnSync(exe,args,{encoding:'utf8',timeout:5000}); return r.status===0?r.stdout.trim():null; };
const result = {schema:1, started_utc:new Date().toISOString(), protocol:'pong-frame-callback-v1', adapter, presentation, renderer, pacing, backend, notes,
  git_revision:command('git',['rev-parse','HEAD']), working_tree:command('git',['status','--porcelain']), rustc:command('rustc',['-Vv']),
  profile:'release', binary_sha256:createHash('sha256').update(readFileSync(binary)).digest('hex'),
  host:{hostname:os.hostname(),os:os.type(),release:os.release(),architecture:os.arch(),cpu:os.cpus()[0]?.model,cpu_count:os.cpus().length,memory_bytes:os.totalmem()},
  environment:{DISPLAY:process.env.DISPLAY??null,WAYLAND_DISPLAY:process.env.WAYLAND_DISPLAY??null},
  unmeasured:['GPU time','displayed FPS','CPU utilization','peak memory','startup latency','power','thermal state'],runs:[]};
for(let i=0;i<3;i++) {
  const before=os.loadavg();
  const r=spawnSync(binary,['--benchmark'],{encoding:'utf8',timeout:30000,maxBuffer:4*1024*1024});
  const line=r.stdout?.split('\n').find(x=>x.startsWith('PONG_BENCHMARK '));
  const sample=line?JSON.parse(line.slice('PONG_BENCHMARK '.length)):null;
  result.runs.push({iteration:i+1,exit:r.status,error:r.error?.message??null,load_before:before,load_after:os.loadavg(),stderr:r.stderr,sample,
    valid:r.status===0 && sample?.samples>0 && !sample.resized && sample.sampled_seconds>=9.5});
  writeFileSync(output,JSON.stringify(result,null,2)+'\n',{flag:i===0?'wx':'w'});
  if(r.status!==0) break;
}
console.log(JSON.stringify(result.runs.map(({iteration,valid,sample})=>({iteration,valid,samples:sample?.samples,p50:sample?.p50_ms,p95:sample?.p95_ms,p99:sample?.p99_ms})),null,2));
if(result.runs.length!==3 || result.runs.some(r=>!r.valid)) process.exitCode=1;
