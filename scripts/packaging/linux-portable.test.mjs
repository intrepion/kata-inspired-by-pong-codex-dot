import test from 'node:test';import assert from 'node:assert/strict';
import{mkdtempSync,readFileSync,readdirSync,writeFileSync,mkdirSync,rmSync,existsSync}from'node:fs';
import{tmpdir}from'node:os';
import{createHash}from'node:crypto';import{join}from'node:path';import{spawnSync}from'node:child_process';
import{packagePortable}from'./linux-portable.mjs';
const linux=process.platform==='linux',rev='a'.repeat(40);
function sandbox(t){const p=mkdtempSync(join(tmpdir(),'pong-portable-unit-'));t.after(()=>rmSync(p,{recursive:true,force:true}));return p;}
test('twelve adapter fixtures package, verify and never overwrite',{skip:!linux},t=>{
 const root=sandbox(t);for(const adapter of ['miniquad','pixels','wgpu','raylib','piston','bracket-lib','bevy','blue-engine','softbuffer','ash','sdl3','ratatui']){
 const out=join(root,adapter),r=packagePortable(adapter,'/bin/true',out,rev);
 const initial=readFileSync(r.archive);assert.throws(()=>packagePortable(adapter,'/bin/true',out,rev),/already exists/);assert.deepEqual(readFileSync(r.archive),initial);
 const unpack=join(root,adapter+'-unpack');mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]);assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);assert.equal(readdirSync(stage).length,5);
 const guide=readFileSync(join(stage,'README.txt'),'utf8');
 if(adapter==='ratatui'){assert.match(guide,/--scripted-cells NEW_DIRECTORY/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else if(adapter==='blue-engine'){assert.match(guide,/Native scripted screenshots are not implemented/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else assert.match(guide,/--scripted-captures NEW_DIRECTORY/);
 }});
test('bad adapter/provenance/input refused before output creation',{skip:!linux},t=>{
 const root=sandbox(t),out=join(root,'output'),plain=join(root,'plain');writeFileSync(plain,'not executable',{mode:0o600});
 for(const args of [['ggez','/bin/true',out,rev],['nannou','/bin/true',out,rev],['no','/bin/true',out,rev],['wgpu','/bin/true',out,'bad'],['wgpu',root,out,rev],['wgpu',plain,out,rev]]){assert.throws(()=>packagePortable(...args));assert.equal(existsSync(out),false);}
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

test('excluded adapters are rejected before staging',{skip:!linux},t=>{const root=sandbox(t);for(const adapter of ['tauri','yew-tauri','leptos-tauri','fyrox','gpui','slint','yew','dioxus','dioxus-desktop','dioxus-mobile','gtk','gtk4']){const out=join(root,adapter);assert.throws(()=>packagePortable(adapter,'/bin/true',out,rev),/Unsupported adapter/);assert.equal(existsSync(out),false);}});

test('archived executable and all reported hashes use the verified staged bytes',{skip:!linux},t=>{
 const root=sandbox(t),input='/bin/true',out=join(root,'output'),unpack=join(root,'unpack');
 const r=packagePortable('ratatui',input,out,rev);
 mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]),archived=readFileSync(join(stage,'pong-ratatui'));
 const hash=createHash('sha256').update(archived).digest('hex');
 assert.deepEqual(archived,readFileSync(input));
 assert.equal(r.binary_sha256,hash);
 assert.match(readFileSync(join(stage,'BUILD-INFO.txt'),'utf8'),new RegExp('^binary_sha256='+hash+'$','m'));
 assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);
 assert.deepEqual(readdirSync(root).sort(),['output','unpack']);
});
test('rejects a truncated executable file and leaves no staged output',{skip:!linux},t=>{
 const root=sandbox(t),binary=join(root,'incomplete'),out=join(root,'output');
 const complete=readFileSync('/bin/true');
 const incomplete=complete.subarray(0,Math.floor(complete.length/2));
 writeFileSync(binary,incomplete,{mode:0o755});
 assert.throws(()=>packagePortable('ratatui',binary,out,rev),/ELF format or architecture verification failed/);
 assert.deepEqual(readdirSync(root),['incomplete']);
 assert.deepEqual(readFileSync(binary),incomplete);
});

for(const [kind,flags,valid] of [
 ['dynamic-pie',['-fPIE','-pie'],true],
 ['static-pie',['-static-pie'],true],
 ['shared-library',['-shared','-fPIC'],false],
]){
 test(kind+' ELF classification',{skip:!linux},t=>{
  const root=sandbox(t),source=join(root,'main.c'),binary=join(root,kind),out=join(root,'out');
  writeFileSync(source,'int main(void) { return 0; }\n');
  const result=spawnSync('cc',[...flags,source,'-o',binary],{encoding:'utf8'});
  if(result.error?.code==='ENOENT'||(kind==='static-pie'&&result.status!==0)){
   t.skip('native cc/linker support is unavailable for '+kind);return;
  }
  assert.equal(result.status,0,result.stderr);
  if(valid) assert.equal(existsSync(packagePortable('ratatui',binary,out,rev).archive),true);
  else {assert.throws(()=>packagePortable('ratatui',binary,out,rev),/not an ELF executable or PIE/);assert.equal(existsSync(out),false);}
 });
}
