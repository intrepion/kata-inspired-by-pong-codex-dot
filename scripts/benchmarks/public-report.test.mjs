import {test} from 'node:test';
import assert from 'node:assert/strict';
import {publicReport} from './public-report.mjs';
function fixture(){return {schema:1,protocol:'pong-frame-callback-v1',profile:'release',adapter:'raylib',git_revision:'a'.repeat(40),binary_sha256:'b'.repeat(64),host:{hostname:'PRIVATE'},notes:'PRIVATE',environment:{SECRET:'PRIVATE'},runs:[{iteration:1,exit:0,valid:true,stderr:'PRIVATE',load_before:['PRIVATE'],sample:{schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,nominal_measure_seconds:10,samples:2,sampled_seconds:10,mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,width:900,height:500,resized:false,intervals_ms:[5000,5000],unexpected:'PRIVATE'}}]};}
test('allowlist omits nested private/unknown fields and preserves exact timing',()=>{const raw=fixture(),out=publicReport(raw);assert.equal(JSON.stringify(out).includes('PRIVATE'),false);assert.deepEqual(out.runs[0].sample.intervals_ms,[5000,5000]);assert.equal(out.runs[0].sample.mean_ms,raw.runs[0].sample.mean_ms);assert.equal(raw.host.hostname,'PRIVATE');});
test('incomplete failed run remains failed without diagnostics',()=>{const raw=fixture();raw.runs[0]={iteration:1,exit:null,valid:false,sample:null,error:'PRIVATE'};const out=publicReport(raw);assert.deepEqual(out.runs[0],{iteration:1,exit:null,valid:false,sample:null});});
test('malformed raw output and parser diagnostics remain private',()=>{const raw=fixture();Object.assign(raw.runs[0],{valid:false,sample:null,raw_sample:'PRIVATE',raw_backend:'PRIVATE',parse_errors:{sample:'PRIVATE',backend:'PRIVATE'}});assert.equal(JSON.stringify(publicReport(raw)).includes('PRIVATE'),false);});
test('malformed provenance and numerical samples fail closed',()=>{for(const mutate of [r=>r.git_revision='PRIVATE',r=>r.adapter='../PRIVATE',r=>r.runs[0].sample.mean_ms=NaN,r=>r.runs[0].sample.intervals_ms=[Infinity,10],r=>r.runs[0].sample.samples=3]){const raw=fixture();mutate(raw);assert.throws(()=>publicReport(raw));}});

test('rejects runs falsely marked valid when execution or measurement is invalid',()=>{
 for(const mutate of [r=>r.runs[0].exit=1,r=>r.runs[0].signal='SIGTERM',r=>r.runs[0].error='failure',r=>r.runs[0].parse_errors={backend:'bad'},r=>r.runs[0].sample.sampled_seconds=.02,r=>r.runs[0].sample.resized=true,r=>r.runs[0].sample.p95_ms=1]){
  const raw=fixture();mutate(raw);assert.throws(()=>publicReport(raw),/valid/i);
 }
});
