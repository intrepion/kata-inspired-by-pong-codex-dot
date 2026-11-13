import test from 'node:test';import assert from 'node:assert/strict';
import{mkdtempSync,readFileSync,readdirSync,writeFileSync,mkdirSync,rmSync,existsSync}from'node:fs';
import{tmpdir}from'node:os';import{join}from'node:path';import{spawnSync}from'node:child_process';
import{packagePortable}from'./linux-portable.mjs';
const linux=process.platform==='linux',rev='a'.repeat(40);
function sandbox(t){const p=mkdtempSync(join(tmpdir(),'pong-portable-unit-'));t.after(()=>rmSync(p,{recursive:true,force:true}));return p;}
test('twenty-two adapter fixtures package, verify and never overwrite',{skip:!linux},t=>{
 const root=sandbox(t);for(const adapter of ['dioxus-desktop','leptos-tauri','yew-tauri','tauri','miniquad','pixels','wgpu','raylib','piston','ggez','nannou','bracket-lib','fyrox','bevy','blue-engine','softbuffer','ash','sdl3','ratatui','gtk','slint','gpui']){
 const out=join(root,adapter),r=packagePortable(adapter,'/bin/true',out,rev);
 const initial=readFileSync(r.archive);assert.throws(()=>packagePortable(adapter,'/bin/true',out,rev),/already exists/);assert.deepEqual(readFileSync(r.archive),initial);
 const unpack=join(root,adapter+'-unpack');mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]);assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);assert.equal(readdirSync(stage).length,['slint','gpui'].includes(adapter)?6:5);
 const guide=readFileSync(join(stage,'README.txt'),'utf8');
 if(adapter==='fyrox') assert.match(guide,/data\/resources\.registry.*working directory/s);
 if(['yew-tauri','leptos-tauri','dioxus-desktop'].includes(adapter)){if(adapter==='dioxus-desktop')assert.match(guide,/libxdo/);assert.match(guide,/WebKitGTK 4\.1/);assert.match(guide,/--scripted-scenes/);assert.match(guide,/not screenshots/);assert.match(guide,/--scripted-captures is rejected/);}else if(adapter==='tauri'){assert.match(guide,/WebKitGTK 4\.1/);assert.match(guide,/excluding DOM scores/);assert.match(guide,/--scripted-captures/);}else if(adapter==='gpui'){assert.match(guide,/state-only/);assert.match(guide,/Pixel export is unsupported/);assert.match(readFileSync(join(stage,'GPUI-LICENSE-APACHE'),'utf8'),/Apache License/);}else if(adapter==='slint'){assert.match(guide,/SLINT-LICENSE/);assert.match(readFileSync(join(stage,'SLINT-LICENSE.md'),'utf8'),/Royalty-free/);}else if(adapter==='gtk'){assert.match(guide,/GTK4 runtime/);assert.match(guide,/Cairo render output/);}else if(adapter==='ratatui'){assert.match(guide,/--scripted-cells NEW_DIRECTORY/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else if(adapter==='blue-engine'){assert.match(guide,/Native scripted screenshots are not implemented/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else assert.match(guide,/--scripted-captures NEW_DIRECTORY/);
 }});
test('bad adapter/provenance/input refused before output creation',{skip:!linux},t=>{
 const root=sandbox(t),out=join(root,'output'),plain=join(root,'plain');writeFileSync(plain,'not executable',{mode:0o600});
 for(const args of [['no','/bin/true',out,rev],['wgpu','/bin/true',out,'bad'],['wgpu',root,out,rev],['wgpu',plain,out,rev]]){assert.throws(()=>packagePortable(...args));assert.equal(existsSync(out),false);}
});

test('optional notice bundle hash checked before output; texts and index packaged',{skip:!linux},async t=>{
 const {createHash}=await import('node:crypto');const root=sandbox(t),text='fixture license\n',index={bundle_sha256:createHash('sha256').update(text).digest('hex'),missing_notice_files:[]};
 const bad=join(root,'bad');assert.throws(()=>packagePortable('miniquad','/bin/true',bad,rev,{text,index:{...index,bundle_sha256:'bad'}}),/Invalid/);assert.equal(existsSync(bad),false);
 assert.throws(()=>packagePortable('miniquad','/bin/true',bad,rev,{text,index:{...index,missing_notice_files:['x']}}),/incomplete/);
 const out=join(root,'good'),r=packagePortable('miniquad','/bin/true',out,rev,{text,index}),unpack=join(root,'unpack');mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]);assert.equal(readdirSync(stage).length,7);assert.equal(readFileSync(join(stage,'THIRD-PARTY-NOTICES.txt'),'utf8'),text);assert.deepEqual(JSON.parse(readFileSync(join(stage,'NOTICE-INDEX.json'))),index);assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);
});

test('rejects an executable with a different ELF architecture before creating output',{skip:!linux},t=>{
 const root=sandbox(t),binary=join(root,'wrong-architecture'),out=join(root,'output');
 const bytes=readFileSync('/bin/true');assert.equal(bytes.subarray(0,4).toString('hex'),'7f454c46');
 const hostMachine=bytes.readUInt16LE(18);bytes.writeUInt16LE(hostMachine===62?183:62,18);
 writeFileSync(binary,bytes,{mode:0o755});
 assert.throws(()=>packagePortable('ratatui',binary,out,rev),/architecture/i);
 assert.equal(existsSync(out),false);
});

test('rejects truncated, non-ELF and wrong-format executables before output',{skip:!linux},t=>{
 const root=sandbox(t),original=readFileSync('/bin/true'),out=join(root,'output');
 for(const [name,bytes] of [['short',Buffer.from([0x7f,0x45,0x4c,0x46])],['script',Buffer.from('#!/bin/sh\nexit 0\n')],['class',Buffer.from(original)],['endian',Buffer.from(original)],['version',Buffer.from(original)]]){
  if(name==='class')bytes[4]=1;if(name==='endian')bytes[5]=2;if(name==='version')bytes[6]=0;
  const input=join(root,name);writeFileSync(input,bytes,{mode:0o755});
  assert.throws(()=>packagePortable('ratatui',input,out,rev),/architecture/i,name);
  assert.equal(existsSync(out),false);assert.deepEqual(readFileSync(input),bytes);
 }
});
