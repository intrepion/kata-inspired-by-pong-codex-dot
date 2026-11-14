import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,writeFileSync,copyFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawnSync} from 'node:child_process';

function git(cwd,...args) {
  const result=spawnSync('git',['-C',cwd,...args],{encoding:'utf8'});
  assert.equal(result.status,0,result.stderr);
  return result.stdout.trim();
}
function fixture(t) {
  const root=mkdtempSync(join(tmpdir(),'pong-checkout-test-'));
  t.after(()=>rmSync(root,{recursive:true,force:true}));
  return root;
}
function repository(dir,label) {
  mkdirSync(dir); git(dir,'init','-q'); writeFileSync(join(dir,'fixture.txt'),label);
  git(dir,'add','fixture.txt');
  git(dir,'-c','user.name=Fixture','-c','user.email=fixture@example.invalid','-c','commit.gpgsign=false','commit','-qm',label);
  return git(dir,'rev-parse','HEAD');
}
async function runner(root) {
  const dir=join(root,'scripts','benchmarks'); mkdirSync(dir,{recursive:true});
  for(const name of ['run.mjs','adapters.mjs']) copyFileSync(join(import.meta.dirname,name),join(dir,name));
  return import(pathToFileURL(join(dir,'run.mjs')).href);
}
function run(module,output) {
  return module.runPilot({binary:process.execPath,output,adapter:'macroquad'},['-e','process.exit(1)']);
}
test('runner outside a checkout leaves attribution unknown',async t=>{
  const root=fixture(t), isolated=join(root,'isolated'); mkdirSync(isolated);
  const result=run(await runner(isolated),join(root,'result.json'));
  assert.equal(result.git_revision,null); assert.equal(result.working_tree,null);
});
test('runner records its checkout instead of the caller repository',async t=>{
  const root=fixture(t), source=join(root,'source'),caller=join(root,'caller');
  const expected=repository(source,'source'),other=repository(caller,'caller'); assert.notEqual(expected,other);
  const module=await runner(source); writeFileSync(join(caller,'caller-only.txt'),'private fixture');
  const prior=process.cwd(); try {
    process.chdir(caller); const result=run(module,join(root,'result.json'));
    assert.equal(result.git_revision,expected); assert.doesNotMatch(result.working_tree,/caller-only/);
  } finally { process.chdir(prior); }
});
test('runner recognizes its linked Git worktree',async t=>{
  const root=fixture(t),main=join(root,'main'),linked=join(root,'linked'),caller=join(root,'caller');
  repository(main,'main'); repository(caller,'caller'); git(main,'worktree','add','-q','-b','fixture-linked',linked);
  writeFileSync(join(linked,'fixture.txt'),'linked'); git(linked,'add','fixture.txt');
  git(linked,'-c','user.name=Fixture','-c','user.email=fixture@example.invalid','-c','commit.gpgsign=false','commit','-qm','linked');
  const expected=git(linked,'rev-parse','HEAD'),module=await runner(linked),prior=process.cwd();
  try { process.chdir(caller); const result=run(module,join(root,'result.json')); assert.equal(result.git_revision,expected); assert.equal(typeof result.working_tree,'string'); }
  finally { process.chdir(prior); }
});
