import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,existsSync,symlinkSync,rmSync,realpathSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {beforeFyroxRun,planFyroxCleanup} from './fyrox-state.mjs';
function fixture(t){const root=realpathSync(mkdtempSync(join(tmpdir(),'pong-state-')));t.after(()=>rmSync(root,{recursive:true,force:true}));return root;}
function registry(root,text='{}'){mkdirSync(join(root,'data'),{recursive:true});writeFileSync(join(root,'data/resources.registry'),text);}
test('new empty registry is review-only and never deleted',t=>{const r=fixture(t),b=beforeFyroxRun(r);registry(r);const p=planFyroxCleanup(r,b);assert.equal(p.action,'review-candidate');assert.equal(p.may_remove_data_directory,true);assert.equal(readFileSync(join(r,'data/resources.registry'),'utf8'),'{}');});
test('preexisting empty registry and missing receipt always preserve',t=>{const r=fixture(t);registry(r);assert.equal(planFyroxCleanup(r,beforeFyroxRun(r)).action,'preserve');assert.equal(planFyroxCleanup(r).action,'preserve');});
test('nonempty generated state and unrelated neighbors preserved',t=>{const r=fixture(t),b=beforeFyroxRun(r);registry(r,'["user-resource"]');assert.equal(planFyroxCleanup(r,b).action,'preserve');registry(r);writeFileSync(join(r,'data/user.txt'),'keep');assert.equal(planFyroxCleanup(r,b).may_remove_data_directory,false);assert.equal(readFileSync(join(r,'data/user.txt'),'utf8'),'keep');});
test('receipt cannot identify another directory',t=>{const a=fixture(t),b=fixture(t),receipt=beforeFyroxRun(a);registry(b);assert.equal(planFyroxCleanup(b,receipt).action,'preserve');});
test('no state needs no cleanup; existing data directory stays owned by caller',t=>{const r=fixture(t);mkdirSync(join(r,'data'));const b=beforeFyroxRun(r);assert.equal(planFyroxCleanup(r,b).action,'none');registry(r);assert.equal(planFyroxCleanup(r,b).may_remove_data_directory,false);assert.ok(existsSync(join(r,'data')));});
test('symlink data directory and registry refused without following targets',{skip:process.platform==='win32'},t=>{const r=fixture(t),outside=fixture(t),b=beforeFyroxRun(r);registry(outside);symlinkSync(join(outside,'data'),join(r,'data'));assert.equal(planFyroxCleanup(r,b).action,'preserve');assert.equal(readFileSync(join(outside,'data/resources.registry'),'utf8'),'{}');});
test('registry symlink and oversized state are preserved',{skip:process.platform==='win32'},t=>{const r=fixture(t),outside=fixture(t),b=beforeFyroxRun(r);mkdirSync(join(r,'data'));writeFileSync(join(outside,'owned.txt'),'{}');symlinkSync(join(outside,'owned.txt'),join(r,'data/resources.registry'));assert.equal(planFyroxCleanup(r,b).action,'preserve');assert.equal(readFileSync(join(outside,'owned.txt'),'utf8'),'{}');const s=fixture(t),before=beforeFyroxRun(s);registry(s,'x'.repeat(1024*1024+1));assert.equal(planFyroxCleanup(s,before).action,'preserve');});

test('symlink root and ancestor are rejected without weakening root identity',{skip:process.platform==='win32'},t=>{const r=fixture(t),outside=fixture(t);mkdirSync(join(outside,'child'));symlinkSync(outside,join(r,'alias'));assert.throws(()=>beforeFyroxRun(join(r,'alias')),/without symlink ancestors/);assert.throws(()=>beforeFyroxRun(join(r,'alias/child')),/without symlink ancestors/);});
