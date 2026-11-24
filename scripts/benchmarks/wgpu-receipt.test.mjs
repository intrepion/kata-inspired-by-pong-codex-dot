import {test} from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {runPilot,wgpuReceiptStatus,wgpuHostProfile,WGPU_HARDWARE_CANDIDATE} from './run.mjs';
import {parseArguments} from './adapters.mjs';
import {publicReport} from './public-report.mjs';

const adapter={schema:1,name:'test',driver:'PRIVATE_DRIVER',driver_info:'PRIVATE_INFO',vendor:1,device:2,
  backend:'Dx12',device_type:'IntegratedGpu',configured_present_mode:'AutoVsync'};
const backend = obj => 'PONG_BENCHMARK_BACKEND '+JSON.stringify(obj);
const sample='PONG_BENCHMARK '+JSON.stringify({schema:1,metric:'frame_callback_interval_ms',warmup_seconds:5,
  nominal_measure_seconds:10,resized:false,width:900,height:500,samples:2,sampled_seconds:10,
  mean_ms:5000,p50_ms:5000,p95_ms:5000,p99_ms:5000,max_ms:5000,intervals_ms:[5000,5000]});

test('strict qualification is explicit and raw-wgpu-only',()=>{
  for(const a of ['sdl3','miniquad']) {
    const args=['binary','out','backend','notes','--adapter',a];
    assert.equal(parseArguments(args).requireWgpuReceipt,undefined);
    assert.throws(()=>parseArguments([...args,'--require-wgpu-receipt']),/requires/);
  }
  assert.equal(parseArguments(['binary','out','backend','notes','--adapter','wgpu','--require-wgpu-receipt']).requireWgpuReceipt,true);
});
test('typed receipt rejects absent, nonobjects, wrong types, unknown enums, and late markers',()=>{
  for(const value of [null,[],{}, {...adapter,schema:2},{...adapter,vendor:'1'},
    {...adapter,device:-1},{...adapter,driver:null},{...adapter,backend:'OtherBackend'},
    {...adapter,device_type:'GPU'},{...adapter,configured_present_mode:['Fifo']}]){
    assert.equal(wgpuReceiptStatus(value,true),'missing_or_invalid');
  }
  assert.equal(wgpuReceiptStatus(adapter,false),'missing_or_invalid');
  assert.equal(wgpuReceiptStatus({...adapter,device_type:'Cpu',backend:'Vulkan'},true),'not_hardware_candidate');
  assert.equal(wgpuReceiptStatus({...adapter,device_type:'VirtualGpu'},true),'not_hardware_candidate');
  assert.equal(wgpuReceiptStatus(adapter,true),WGPU_HARDWARE_CANDIDATE);
  assert.equal(wgpuHostProfile('win32'),'windows_dx12');
  assert.equal(wgpuHostProfile('darwin'),'non_windows_native');
  for (const name of ['Vulkan','Metal']) {
    assert.equal(wgpuReceiptStatus({...adapter,backend:name},true,'windows_dx12'),'unexpected_backend');
    assert.equal(wgpuReceiptStatus({...adapter,backend:name},true,'non_windows_native'),WGPU_HARDWARE_CANDIDATE);
  }
});
test('same-process marker order, duplicate and malformed failures, legacy and privacy',t=>{
  const dir=mkdtempSync(join(tmpdir(),'pong-receipt-'));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  let i=0;
  const run=(lines,name='wgpu',strict=true)=>runPilot({binary:process.execPath,output:join(dir,`${i++}.json`),adapter:name,requireWgpuReceipt:strict},
    ['-e',`process.stdout.write(${JSON.stringify(lines.join('\n'))})`]);
  const good=run([backend(adapter),sample]);
  assert(good.runs.every(r=>r.valid&&r.wgpu_receipt_status==='hardware_candidate_unverified_host'));
  const sanitized=JSON.stringify(publicReport({...good,git_revision:'a'.repeat(40)}));
  assert.doesNotMatch(sanitized,/PRIVATE|wgpu_receipt_status|reported_backend/);
  assert(publicReport({...good,git_revision:'a'.repeat(40)}).runs.every(r=>r.valid));
  for(const [lines,status] of [
    [[sample],'missing_or_invalid'],
    [[sample,backend(adapter)],'missing_or_invalid'],
    [['PONG_BENCHMARK_BACKEND {not-json',sample],'missing_or_invalid'],
    [[backend(adapter),backend(adapter),sample],'missing_or_invalid'],
    [[backend({...adapter,device_type:'Cpu'}),sample],'not_hardware_candidate'],
    [[backend({...adapter,device_type:'VirtualGpu'}),sample],'not_hardware_candidate'],
  ]) {
    const rejected=run(lines);
    assert(rejected.runs.every(r=>r.wgpu_receipt_status===status&&!r.valid));
    assert(publicReport({...rejected,git_revision:'a'.repeat(40)}).runs.every(r=>!r.valid));
  }
  // Regression for an earlier private runner writing valid:true despite rejection.
  const legacy={...good,git_revision:'a'.repeat(40)};
  delete legacy.wgpu_receipt_profile;
  assert(publicReport(legacy).runs.every(r=>!r.valid));
  for (const receipt of [null,{...adapter,device_type:'Cpu'},{...adapter,backend:'Vulkan'}]) {
    const wrong={...good,git_revision:'a'.repeat(40),wgpu_receipt_profile:'windows_dx12',
      runs:good.runs.map(r=>({...r,reported_backend:receipt}))};
    assert(publicReport(wrong).runs.every(r=>!r.valid));
  }
  assert(run([sample],'wgpu',false).runs.every(r=>r.valid&&!Object.hasOwn(r,'wgpu_receipt_status')));
  assert(run([backend({video_driver:'private',renderer:'private'}),sample],'sdl3',false).runs.every(r=>r.valid&&!Object.hasOwn(r,'wgpu_receipt_status')));
  assert(run([backend({backend:'Gl'}),sample],'miniquad',false).runs.every(r=>r.valid&&!Object.hasOwn(r,'wgpu_receipt_status')));
});
