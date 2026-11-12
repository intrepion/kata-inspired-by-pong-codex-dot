import test from 'node:test';import assert from 'node:assert/strict';
import{mkdtempSync,readFileSync,readdirSync,writeFileSync,mkdirSync,rmSync,existsSync}from'node:fs';
import{tmpdir}from'node:os';import{join}from'node:path';import{spawnSync}from'node:child_process';
import{packagePortable}from'./linux-portable.mjs';
const linux=process.platform==='linux',rev='a'.repeat(40);
function sandbox(t){const p=mkdtempSync(join(tmpdir(),'pong-portable-unit-'));t.after(()=>rmSync(p,{recursive:true,force:true}));return p;}
test('four adapter fixtures package, verify and never overwrite',{skip:!linux},t=>{
 const root=sandbox(t);for(const adapter of ['miniquad','pixels','wgpu','raylib']){
 const out=join(root,adapter),r=packagePortable(adapter,'/bin/true',out,rev);
 const initial=readFileSync(r.archive);assert.throws(()=>packagePortable(adapter,'/bin/true',out,rev),/already exists/);assert.deepEqual(readFileSync(r.archive),initial);
 const unpack=join(root,adapter+'-unpack');mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]);assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);assert.equal(readdirSync(stage).length,5);
 }});
test('bad adapter/provenance/input refused before output creation',{skip:!linux},t=>{
 const root=sandbox(t),out=join(root,'output'),plain=join(root,'plain');writeFileSync(plain,'not executable',{mode:0o600});
 for(const args of [['no','/bin/true',out,rev],['wgpu','/bin/true',out,'bad'],['wgpu',root,out,rev],['wgpu',plain,out,rev]]){assert.throws(()=>packagePortable(...args));assert.equal(existsSync(out),false);}
});
