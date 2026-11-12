const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const os=require('node:os');
const path=require('node:path');
const {spawnSync}=require('node:child_process');
const script=path.join(__dirname,'linux-portable.sh');
const revision='a'.repeat(40);
const run=(...args)=>spawnSync('bash',[script,...args],{encoding:'utf8'});
function sandbox(t){const p=fs.mkdtempSync(path.join(os.tmpdir(),'pong-package-unit-'));t.after(()=>fs.rmSync(p,{recursive:true,force:true}));return p;}
const linux=process.platform==='linux';
test('Linux fixture archive checksums/extraction; existing output never overwritten',{skip:!linux},t=>{
  const root=sandbox(t),out=path.join(root,'package'),sentinel=path.join(root,'neighbor');fs.writeFileSync(sentinel,'keep');
  // /bin/true is only a packaging fixture, not a Pong runtime qualification.
  assert.equal(run('/bin/true',out,revision).status,0);
  const archive=fs.readdirSync(out).find(n=>n.endsWith('.tar.gz'));
  assert.ok(archive);const before=fs.readFileSync(path.join(out,archive));
  assert.notEqual(run('/bin/true',out,revision).status,0);
  assert.deepEqual(fs.readFileSync(path.join(out,archive)),before);assert.equal(fs.readFileSync(sentinel,'utf8'),'keep');
  assert.equal(spawnSync('sha256sum',['-c',archive+'.sha256'],{cwd:out}).status,0);
  const unpack=path.join(root,'unpack');fs.mkdirSync(unpack);
  assert.equal(spawnSync('tar',['-xzf',path.join(out,archive),'-C',unpack]).status,0);
  const contents=path.join(unpack,fs.readdirSync(unpack)[0]);
  assert.equal(spawnSync('sha256sum',['-c','SHA256SUMS'],{cwd:contents}).status,0);
});
test('invalid provenance or directory input refused before output creation',{skip:!linux},t=>{
  const root=sandbox(t);
  for(const [input,rev,label] of [['/bin/true','not-a-commit','bad-revision'],[root,revision,'directory']]){
    const out=path.join(root,label);assert.notEqual(run(input,out,rev).status,0);assert.equal(fs.existsSync(out),false);
  }
});
test('nonexecutable input refused before output creation',{skip:!linux},t=>{
  const root=sandbox(t),input=path.join(root,'plain'),out=path.join(root,'package');fs.writeFileSync(input,'not executable',{mode:0o600});
  assert.notEqual(run(input,out,revision).status,0);assert.equal(fs.existsSync(out),false);
});
