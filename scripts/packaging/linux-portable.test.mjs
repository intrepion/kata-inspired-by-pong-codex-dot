import test from 'node:test';import assert from 'node:assert/strict';
import{mkdtempSync,readFileSync,readdirSync,writeFileSync,mkdirSync,rmSync,existsSync}from'node:fs';
import{tmpdir}from'node:os';import{join}from'node:path';import{spawnSync}from'node:child_process';
import{packagePortable}from'./linux-portable.mjs';
const linux=process.platform==='linux',rev='a'.repeat(40);
function sandbox(t){const p=mkdtempSync(join(tmpdir(),'pong-portable-unit-'));t.after(()=>rmSync(p,{recursive:true,force:true}));return p;}
test('eighteen adapter fixtures package, verify and never overwrite',{skip:!linux},t=>{
 const root=sandbox(t);for(const adapter of ['miniquad','pixels','wgpu','raylib','piston','ggez','nannou','bracket-lib','fyrox','bevy','blue-engine','softbuffer','ash','sdl3','ratatui','gtk','slint','gpui']){
 const out=join(root,adapter),r=packagePortable(adapter,'/bin/true',out,rev);
 const initial=readFileSync(r.archive);assert.throws(()=>packagePortable(adapter,'/bin/true',out,rev),/already exists/);assert.deepEqual(readFileSync(r.archive),initial);
 const unpack=join(root,adapter+'-unpack');mkdirSync(unpack);assert.equal(spawnSync('tar',['-xzf',r.archive,'-C',unpack]).status,0);
 const stage=join(unpack,readdirSync(unpack)[0]);assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);assert.equal(readdirSync(stage).length,['slint','gpui'].includes(adapter)?6:5);
 const guide=readFileSync(join(stage,'README.txt'),'utf8');
 if(adapter==='gpui'){assert.match(guide,/state-only/);assert.match(guide,/Pixel export is unsupported/);assert.match(readFileSync(join(stage,'GPUI-LICENSE-APACHE'),'utf8'),/Apache License/);}else if(adapter==='slint'){assert.match(guide,/SLINT-LICENSE/);assert.match(readFileSync(join(stage,'SLINT-LICENSE.md'),'utf8'),/Royalty-free/);}else if(adapter==='gtk'){assert.match(guide,/GTK4 runtime/);assert.match(guide,/Cairo render output/);}else if(adapter==='ratatui'){assert.match(guide,/--scripted-cells NEW_DIRECTORY/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else if(adapter==='blue-engine'){assert.match(guide,/Native scripted screenshots are not implemented/);assert.doesNotMatch(guide,/--scripted-captures NEW_DIRECTORY/);}else assert.match(guide,/--scripted-captures NEW_DIRECTORY/);
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
