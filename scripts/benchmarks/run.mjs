// Shared bounded pilot runner. Node built-ins only. Run an already-built binary.
import {spawnSync} from 'node:child_process';
import {writeFileSync, readFileSync, existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import os from 'node:os';
import {parseArguments} from './adapters.mjs';
import {pathToFileURL} from 'node:url';
export function validSample(s) {
  const timings=['mean_ms','p50_ms','p95_ms','p99_ms','max_ms'];
  return s?.schema===1 && s.metric==='frame_callback_interval_ms' &&
    s.warmup_seconds===5 && s.nominal_measure_seconds===10 && s.resized===false &&
    Number.isInteger(s.samples) && s.samples>0 && Number.isFinite(s.sampled_seconds) && s.sampled_seconds>=9.5 &&
    Number.isInteger(s.width) && s.width>0 && Number.isInteger(s.height) && s.height>0 &&
    timings.every(k=>Number.isFinite(s[k]) && s[k]>=0) && Array.isArray(s.intervals_ms) &&
    s.intervals_ms.length===s.samples && s.intervals_ms.every(n=>Number.isFinite(n) && n>=0);
}
export function runPilot(options, binaryArguments = ['--benchmark']) {
  const {binary, output, backend, notes, adapter, presentation, renderer, pacing} = options;
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
    const r=spawnSync(binary,binaryArguments,{encoding:'utf8',timeout:30000,maxBuffer:4*1024*1024});
    const line=r.stdout?.split('\n').find(x=>x.startsWith('PONG_BENCHMARK '));
    const backendLine=r.stdout?.split('\n').find(x=>x.startsWith('PONG_BENCHMARK_BACKEND '));
    const raw_sample=line?.slice('PONG_BENCHMARK '.length)??null;
    const raw_backend=backendLine?.slice('PONG_BENCHMARK_BACKEND '.length)??null;
    const parse_errors={sample:null,backend:null};
    let sample=null,reported_backend=null;
    try { if(raw_sample!==null) sample=JSON.parse(raw_sample); } catch(e) { parse_errors.sample=e.message; }
    try { if(raw_backend!==null) reported_backend=JSON.parse(raw_backend); } catch(e) { parse_errors.backend=e.message; }
    result.runs.push({iteration:i+1,exit:r.status,signal:r.signal??null,error:r.error?.message??null,load_before:before,load_after:os.loadavg(),stderr:r.stderr,reported_backend,sample,parse_errors,
      raw_sample:parse_errors.sample?raw_sample:null,raw_backend:parse_errors.backend?raw_backend:null,
      valid:r.status===0 && !parse_errors.sample && !parse_errors.backend && validSample(sample)});
    writeFileSync(output,JSON.stringify(result,null,2)+'\n',{flag:i===0?'wx':'w'});
    if(r.status!==0) break;
  }
  return result;
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
const result = runPilot(parseArguments(process.argv.slice(2)));
console.log(JSON.stringify(result.runs.map(({iteration,valid,sample})=>({iteration,valid,samples:sample?.samples,p50:sample?.p50_ms,p95:sample?.p95_ms,p99:sample?.p99_ms})),null,2));
if(result.runs.length!==3 || result.runs.some(r=>!r.valid)) process.exitCode=1;

}
