import test from 'node:test';import assert from 'node:assert/strict';
import{mkdtempSync,mkdirSync,writeFileSync,rmSync,symlinkSync}from'node:fs';
import{tmpdir}from'node:os';import{join}from'node:path';import{createHash}from'node:crypto';
import{buildNoticeBundle}from'./notice-bundle.mjs';
function fixture(t){const root=mkdtempSync(join(tmpdir(),'pong-notice-unit-'));t.after(()=>rmSync(root,{recursive:true,force:true}));writeFileSync(join(root,'LICENSE'),'MIT root\n');const pkg=join(root,'crate');mkdirSync(pkg);writeFileSync(join(pkg,'Cargo.toml'),'');return{root,pkg};}
function meta(pkg,extra={}){return{packages:[{id:'x',name:'fixture',version:'1.0.0',source:'registry+fixture',license:'MIT',manifest_path:join(pkg,'Cargo.toml'),...extra}],resolve:{nodes:[{id:'x'}]}};}
test('preserves exact texts, nested notices and hashes without host paths',t=>{const{root,pkg}=fixture(t);writeFileSync(join(pkg,'LICENSE'),'MIT text\r\n');mkdirSync(join(pkg,'vendor'));writeFileSync(join(pkg,'vendor','NOTICE'),'Attribution\n');const b=buildNoticeBundle(meta(pkg),root);assert.equal(b.index.packages[0].notices.length,2);assert.equal(b.index.packages[0].notices[0].sha256,createHash('sha256').update('MIT text\r\n').digest('hex'));assert(b.text.includes('MIT text\r\n'));assert(!JSON.stringify(b).includes(root));assert.equal(b.index.bundle_sha256,createHash('sha256').update(b.text).digest('hex'));});
test('first-party fallback is constrained; missing external notices remain explicit',t=>{const{root,pkg}=fixture(t);assert.deepEqual(buildNoticeBundle(meta(pkg),root).index.missing_notice_files,['fixture@1.0.0']);const b=buildNoticeBundle(meta(pkg,{name:'pong-fixture',source:null}),root);assert.equal(b.index.packages[0].root_mit_fallback,true);assert.equal(b.index.packages[0].notices[0].source,'LICENSE');assert.throws(()=>buildNoticeBundle(meta(pkg,{license_file:'../LICENSE'}),root),/escapes/);});
test('symlink notice sources rejected', {skip:process.platform==='win32'},t=>{const{root,pkg}=fixture(t);symlinkSync(join(root,'LICENSE'),join(pkg,'LICENSE'));assert.throws(()=>buildNoticeBundle(meta(pkg),root),/Symlink/);});

test('platform title is explicit while existing Linux output remains unchanged',t=>{const{root,pkg}=fixture(t);writeFileSync(join(pkg,'LICENSE'),'MIT text');const implicit=buildNoticeBundle(meta(pkg),root);const linux=buildNoticeBundle(meta(pkg),root,{platform:'Linux'});assert.deepEqual(implicit,linux);const windows=buildNoticeBundle(meta(pkg),root,{platform:'Windows'});assert(windows.text.startsWith('Pong Windows dependency notice preparation\n'));assert.deepEqual(windows.index.packages,linux.index.packages);assert.equal(windows.index.bundle_sha256,createHash('sha256').update(windows.text).digest('hex'));assert.throws(()=>buildNoticeBundle(meta(pkg),root,{platform:'Windows\nApproved'}),/platform/);});

test('first-party fallback rejects a sibling checkout on every platform',t=>{const a=fixture(t),b=fixture(t);assert.throws(()=>buildNoticeBundle(meta(b.pkg,{name:'pong-outside',source:null}),a.root),/outside repository/);});

test('audited headers preserve source bytes and reject changed or escaping files',async t=>{
 const {auditedSourceHeaders}=await import('./notice-bundle.mjs');const {root,pkg}=fixture(t),source='/* Original notice\r\nCopyright holder\r\n*/\ncode\n',file=join(pkg,'source.rs');writeFileSync(file,source);
 const hash=createHash('sha256').update(source).digest('hex'),[notice]=auditedSourceHeaders(pkg,[['source.rs',hash]]);
 assert.equal(notice.text,'/* Original notice\r\nCopyright holder\r\n*/');assert.equal(notice.sha256,createHash('sha256').update(notice.text).digest('hex'));assert.equal(notice.source,'source.rs (leading comment)');
 writeFileSync(file,source+'changed');assert.throws(()=>auditedSourceHeaders(pkg,[['source.rs',hash]]),/hash mismatch/);
 assert.throws(()=>auditedSourceHeaders(pkg,[['../LICENSE',hash]]),/escapes/);
 if(process.platform!=='win32'){symlinkSync(join(root,'LICENSE'),join(pkg,'link.rs'));assert.throws(()=>auditedSourceHeaders(pkg,[['link.rs',hash]]),/Symlink/);}
 const noHeader='code';writeFileSync(file,noHeader);assert.throws(()=>auditedSourceHeaders(pkg,[['source.rs',createHash('sha256').update(noHeader).digest('hex')]]),/Missing audited/);
});

test('raylib source audit applies only to the exact registry version',t=>{
 const{root,pkg}=fixture(t);const base={name:'raylib',version:'6.0.0',source:'registry+https://github.com/rust-lang/crates.io-index'};
 assert.throws(()=>buildNoticeBundle(meta(pkg,base),root),/ENOENT/);
 assert.deepEqual(buildNoticeBundle(meta(pkg,{...base,version:'6.0.1'}),root).index.missing_notice_files,['raylib@6.0.1']);
 assert.deepEqual(buildNoticeBundle(meta(pkg,{...base,source:'git+https://example.invalid/raylib'}),root).index.missing_notice_files,['raylib@6.0.0']);
});

test('audited upstream notices enforce revision, source and notice hashes and containment',async t=>{
 const {auditedUpstreamNotices}=await import('./notice-bundle.mjs');const{root,pkg}=fixture(t),n=join(root,'notices');mkdirSync(n);mkdirSync(join(pkg,'src'));const hash=s=>createHash('sha256').update(s).digest('hex');
 const code='source\n',notice='original notice\n';writeFileSync(join(pkg,'src','lib.rs'),code);writeFileSync(join(pkg,'.cargo_vcs_info.json'),JSON.stringify({git:{sha1:'revision'},path_in_vcs:'crate'}));writeFileSync(join(n,'LICENSE'),notice);
 const rule={rev:'revision',path:'crate',sourceHash:hash(code),files:[['LICENSE',hash(notice),'https://example.org/exact/LICENSE']]};const [got]=auditedUpstreamNotices(pkg,n,rule);assert.equal(got.text,notice);assert.equal(got.source,rule.files[0][2]);
 assert.throws(()=>auditedUpstreamNotices(pkg,n,{...rule,rev:'wrong'}),/revision mismatch/);writeFileSync(join(pkg,'src','lib.rs'),'changed');assert.throws(()=>auditedUpstreamNotices(pkg,n,rule),/source hash mismatch/);writeFileSync(join(pkg,'src','lib.rs'),code);
 writeFileSync(join(n,'LICENSE'),'changed');assert.throws(()=>auditedUpstreamNotices(pkg,n,rule),/notice hash mismatch/);assert.throws(()=>auditedUpstreamNotices(pkg,n,{...rule,files:[['../LICENSE',hash(notice),'url']]}),/escapes/);
 if(process.platform!=='win32'){symlinkSync(join(root,'LICENSE'),join(n,'link'));assert.throws(()=>auditedUpstreamNotices(pkg,n,{...rule,files:[['link',hash(notice),'url']]}),/Symlink/);}
});

test('upstream rules require exact registry package and version',t=>{
 const{root,pkg}=fixture(t),registry='registry+https://github.com/rust-lang/crates.io-index';
 for(const[name,version]of [['alloc-stdlib','0.3.0'],['dlopen2','0.8.2'],['dlopen2_derive','0.4.3'],['libappindicator-sys','0.9.0'],['selectors','0.38.0']]){
  assert.throws(()=>buildNoticeBundle(meta(pkg,{name,version,source:registry}),root),/ENOENT/);
  assert.deepEqual(buildNoticeBundle(meta(pkg,{name,version,source:'git+https://example.invalid/repo'}),root).index.missing_notice_files,[name+'@'+version]);
  assert.deepEqual(buildNoticeBundle(meta(pkg,{name,version:'999.0.0',source:registry}),root).index.missing_notice_files,[name+'@999.0.0']);
 }
});

test('audited source override is contained and hash checked',async t=>{
 const {auditedUpstreamNotices}=await import('./notice-bundle.mjs');const{root,pkg}=fixture(t),n=join(root,'notices');mkdirSync(n);const hash=s=>createHash('sha256').update(s).digest('hex');writeFileSync(join(pkg,'.cargo_vcs_info.json'),JSON.stringify({git:{sha1:'r'},path_in_vcs:'p'}));writeFileSync(join(pkg,'lib.rs'),'source');writeFileSync(join(n,'LICENSE'),'notice');const rule={rev:'r',path:'p',sourcePath:'lib.rs',sourceHash:hash('source'),files:[['LICENSE',hash('notice'),'url']]};assert.equal(auditedUpstreamNotices(pkg,n,rule)[0].text,'notice');assert.throws(()=>auditedUpstreamNotices(pkg,n,{...rule,sourcePath:'../LICENSE'}),/escapes/);
});
