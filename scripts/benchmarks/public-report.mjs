// Explicit publication allowlist. Original runner output stays local.
import {readFileSync, writeFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {validSample} from './run.mjs';
const sampleNumbers = ['schema','warmup_seconds','nominal_measure_seconds','samples','sampled_seconds','mean_ms','p50_ms','p95_ms','p99_ms','max_ms','width','height'];
export function publicReport(raw) {
  if(raw?.schema!==1 || raw.protocol!=='pong-frame-callback-v1' || !Array.isArray(raw.runs)) throw Error('Unsupported benchmark report');
  if(typeof raw.adapter!=='string' || !/^[a-z0-9-]+$/.test(raw.adapter)) throw Error('Invalid adapter identifier');
  if(typeof raw.git_revision!=='string' || typeof raw.binary_sha256!=='string' || !/^[a-f0-9]{40}$/.test(raw.git_revision) || !/^[a-f0-9]{64}$/.test(raw.binary_sha256)) throw Error('Source and binary hashes required');
  if(!['release','debug','unverified'].includes(raw.profile)) throw Error('Known build profile required');
  if(raw.runs.length<1 || raw.runs.length>3 || raw.runs.some((run,index)=>run?.iteration!==index+1)) throw Error('Invalid benchmark run sequence');
  const result={profile:raw.profile,schema:1,protocol:'pong-frame-callback-v1',adapter:raw.adapter,git_revision:raw.git_revision,binary_sha256:raw.binary_sha256,
    provenance_limit:'Profile is caller supplied or unverified; checkout revision does not prove binary source.',
    privacy:'Publication copy: allowlisted timing/provenance fields only. Host identity, environment, toolchain, notes and process diagnostics omitted. Original retained locally.',
    unmeasured:['GPU time','displayed FPS','CPU utilization','peak memory','startup latency','power','thermal state'],runs:[]};
  for(const run of raw.runs) {
    if(!Number.isInteger(run.iteration) || typeof run.valid!=='boolean') throw Error('Malformed run');
    const output={iteration:run.iteration,exit:Number.isInteger(run.exit)?run.exit:null,valid:run.valid,sample:null};
    if(run.valid && (run.exit!==0 || run.signal!=null || run.error!=null || run.parse_errors?.sample!=null || run.parse_errors?.backend!=null || !validSample(run.sample))) throw Error('Run marked valid failed execution or measurement validation');
    if(run.sample) {
      const s=run.sample;
      if(s.metric!=='frame_callback_interval_ms' || typeof s.resized!=='boolean' || !Array.isArray(s.intervals_ms)) throw Error('Malformed sample');
      const sample={metric:s.metric,resized:s.resized};
      for(const key of sampleNumbers) {if(!Number.isFinite(s[key])) throw Error(`Invalid ${key}`);sample[key]=s[key];}
      if(!s.intervals_ms.every(n=>Number.isFinite(n)&&n>=0)) throw Error('Invalid intervals');
      if(s.samples!==s.intervals_ms.length) throw Error('Sample count mismatch');
      sample.intervals_ms=[...s.intervals_ms];output.sample=sample;
    }
    result.runs.push(output);
  }
  return result;
}
if(process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const [input,output,...extra]=process.argv.slice(2);
  if(!input||!output||extra.length) throw Error('Usage: public-report.mjs PRIVATE_INPUT.json NEW_PUBLIC_OUTPUT.json');
  const result=publicReport(JSON.parse(readFileSync(input,'utf8')));
  writeFileSync(output,JSON.stringify(result,null,2)+'\n',{flag:'wx'});
}
