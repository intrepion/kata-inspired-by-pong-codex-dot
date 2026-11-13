import {test} from 'node:test';
import assert from 'node:assert/strict';
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
import {packageDioxus} from './dioxus-local-xdo.mjs';
const usable=process.platform==='linux'&&process.arch==='arm64',source='a'.repeat(40),dep='/tmp/pong-xdotool-library.pkg.tar.xz';
test('rejects unrecognized dependency before writing output',{skip:!usable},t=>{
 const dir=mkdtempSync(join(tmpdir(),'pong-xdo-test-'));t.after(()=>rmSync(dir,{recursive:true,force:true}));
 const bad=join(dir,'bad'),out=join(dir,'output');writeFileSync(bad,'not the verified archive');
 assert.throws(()=>packageDioxus('/bin/true',bad,out,source),/Unrecognized xdotool archive/);assert.equal(existsSync(out),false);
});
test('verified dependency archive creates a manifest and launcher',{skip:!usable||!existsSync(dep)},t=>{
 const dir=mkdtempSync(join(tmpdir(),'pong-xdo-test-'));t.after(()=>rmSync(dir,{recursive:true,force:true}));const out=join(dir,'output');
 const result=packageDioxus('/bin/true',dep,out,source),stage=join(out,'pong-dioxus-local-xdo-linux-arm64');
 assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:stage}).status,0);
 assert.equal(spawnSync(join(stage,'run-pong-dioxus'),[],{cwd:dir,env:{...process.env,LD_LIBRARY_PATH:''}}).status,0);
 assert.match(readFileSync(join(stage,'XDO-COPYRIGHT'),'utf8'),/Copyright/);
 assert.throws(()=>packageDioxus('/bin/true',dep,out,source),/already exists/);assert.ok(result.archive_sha256);
});
