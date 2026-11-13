// Preparation aid, not legal clearance or proof of linked-code attribution.
import {readFileSync,readdirSync,lstatSync,realpathSync} from 'node:fs';
import {dirname,relative,resolve,join,isAbsolute} from 'node:path';
import {createHash} from 'node:crypto';
const digest=b=>createHash('sha256').update(b).digest('hex');
function safeFile(root,file) {
 const rel=relative(root,file);
 if(isAbsolute(rel)||rel==='..'||rel.startsWith('../')||rel.startsWith('..\\')) throw Error('License file escapes package root');
 if(lstatSync(file).isSymbolicLink() || realpathSync(file)!==resolve(file)) throw Error('Symlink notice source refused');
 if(!lstatSync(file).isFile() || lstatSync(file).size>1024*1024) throw Error('Invalid or oversized notice source');
 const bytes=readFileSync(file),text=bytes.toString('utf8');
 if(!Buffer.from(text,'utf8').equals(bytes)||text.includes('\0')) throw Error('Notice must be UTF-8 text');
 return {source:rel.replaceAll('\\','/'),sha256:digest(bytes),bytes:bytes.length,text};
}
function candidates(root) {
 const found=[];
 function visit(dir) {
  for(const e of readdirSync(dir,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name))){
   if(e.name==='.git'||e.name==='target')continue;
   const file=join(dir,e.name);
   if(e.isDirectory()) visit(file);
   else if(/^(?:licen[sc]e|copying|notice|copyright)(?:[._-]|$)/i.test(e.name)) found.push(file);
  }
 }
 visit(root); return found;
}
// Audited against the published raylib 6.0.0 archive and its recorded upstream revision.
// Hash the complete source before taking only its original leading comment.
const raylibHeaders=Object.freeze([
 ['src/lib.rs','ede8d8a5446ea278d1a91171c0a09a23623b627c3dd1188a072e4d0fd02799b4'],
 ['src/ease.rs','38b8a54d114f412698eba7b159a639aa62cbe6b6315714010e7dd447fb21b46e'],
 ['src/prelude.rs','b3e3d48997507630dbfe1948f9b8fbda079647435da8edf94f1f6ea8a572e511'],
 ['src/core/math.rs','0183cdc9933b947bd8a77a230c54acd7f741bba183a315a585657699a45c4b1d'],
]);
export function auditedSourceHeaders(root,entries) {
 return entries.map(([path,expected])=>{
  const f=safeFile(root,resolve(root,path));
  if(f.sha256!==expected)throw Error('Audited source hash mismatch: '+path);
  const end=f.text.indexOf('*/');
  if(!f.text.startsWith('/*')||end<2)throw Error('Missing audited source header: '+path);
  const text=f.text.slice(0,end+2);
  return {source:f.source+' (leading comment)',sha256:digest(Buffer.from(text)),bytes:Buffer.byteLength(text),text};
 });
}
// Exact upstream or source-referenced license texts; each rule is bound to the published crate's
// recorded VCS revision and a source file independently matched to upstream.
const upstreamRules=Object.freeze({
 'selectors@0.38.0':{rev:'572ecba2d1600e7c3d490586692a209faf703baa',path:'selectors',sourcePath:'lib.rs',sourceHash:'d54c6e13e9e952dac17d209171df8657e3cae93beaddace4906150cdec8d02e9',files:[['mozilla-MPL-2.0.txt','fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85','https://github.com/mozilla/openbadges-bakery/blob/3057b06f75997c22ad4f039960dd2688578c3d32/LICENSE-MPL-2.0']]},
 'alloc-stdlib@0.3.0':{rev:'0a81fd6928ea3b33c8cd484aa4575d50ffb98012',path:'alloc-stdlib',sourceHash:'3b1358fd99790a1063f0f2b4358d3771a75fcc3ee8e4560984326f86a8c56b0a',files:[['alloc-stdlib-LICENSE.txt','c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae','https://github.com/dropbox/rust-alloc-no-stdlib/blob/0a81fd6928ea3b33c8cd484aa4575d50ffb98012/LICENSE']]},
 'dlopen2@0.8.2':{rev:'cc80e4a0a90d499b677fdf7743699b4b3a43a989',path:'dlopen2',sourceHash:'e99da1fe29c4c9391b6e0f2de4220da7b5dafa62649a9889a9ab4a9f3722cbb5',files:[['dlopen2-LICENSE.txt','39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2','https://github.com/OpenByteDev/dlopen2/blob/cc80e4a0a90d499b677fdf7743699b4b3a43a989/LICENSE']]},
 'dlopen2_derive@0.4.3':{rev:'cc80e4a0a90d499b677fdf7743699b4b3a43a989',path:'dlopen2-derive',sourceHash:'f172e1b7acca5edabefc867c1f7dd8f6deaf3eeb0f53097740961fe47b770a72',files:[['dlopen2-LICENSE.txt','39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2','https://github.com/OpenByteDev/dlopen2/blob/cc80e4a0a90d499b677fdf7743699b4b3a43a989/LICENSE']]},
 'libappindicator-sys@0.9.0':{rev:'eafd1e3682a1247f595410266091e9684021cb6f',path:'sys',sourceHash:'742a25cd2a4c09762f57c3dfaa8958e6c62c771e28916a58b45a85fae84aea6d',files:[['libappindicator-LICENSE-MIT.txt','eb227437252b2a7a9c1fc342c93ade1f3d7ce38cc6dd754f613db07d53ceff0b','https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-MIT'],['libappindicator-LICENSE-APACHE.txt','a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2','https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-APACHE']]},
});
export function auditedUpstreamNotices(packageRoot,noticeRoot,rule) {
 const vcs=JSON.parse(safeFile(packageRoot,resolve(packageRoot,'.cargo_vcs_info.json')).text);
 if(vcs.git?.sha1!==rule.rev||vcs.path_in_vcs!==rule.path)throw Error('Audited crate revision mismatch');
 if(safeFile(packageRoot,resolve(packageRoot,rule.sourcePath??'src/lib.rs')).sha256!==rule.sourceHash)throw Error('Audited crate source hash mismatch');
 return rule.files.map(([path,expected,url])=>{
  const f=safeFile(noticeRoot,resolve(noticeRoot,path));
  if(f.sha256!==expected)throw Error('Audited upstream notice hash mismatch');
  return {...f,source:url};
 });
}
export function buildNoticeBundle(metadata,repoRoot,{platform='Linux'}={}) {
 if(!['Linux','Windows','macOS'].includes(platform)) throw Error('Unsupported platform label');
 if(!Array.isArray(metadata.packages)||!Array.isArray(metadata.resolve?.nodes)) throw Error('Resolved Cargo metadata required');
 const wanted=new Set(metadata.resolve.nodes.map(x=>x.id)),root=realpathSync(repoRoot),packages=[];
 for(const pkg of metadata.packages.filter(p=>wanted.has(p.id)).sort((a,b)=>a.name.localeCompare(b.name)||a.version.localeCompare(b.version))) {
  if(!/^[a-zA-Z0-9_-]+$/.test(pkg.name)||!pkg.version||!pkg.manifest_path) throw Error('Invalid package metadata');
  const dir=realpathSync(dirname(pkg.manifest_path));let files=candidates(dir);
  if(pkg.license_file)files.push(resolve(dir,pkg.license_file));
  files=[...new Set(files)].sort();
  let fallback=false;
  if(!files.length&&!pkg.source&&pkg.name.startsWith('pong-')&&pkg.license==='MIT'){
   const rel=relative(root,dir);if(rel==='..'||rel.startsWith('../')||rel.startsWith('..\\')||isAbsolute(rel))throw Error('First-party fallback outside repository');
   files=[join(root,'LICENSE')];fallback=true;
  }
  const notices=files.map(f=>safeFile(fallback?root:dir,f));
  const upstream=upstreamRules[pkg.name+'@'+pkg.version];
  if(upstream&&pkg.source==='registry+https://github.com/rust-lang/crates.io-index')notices.push(...auditedUpstreamNotices(dir,resolve(root,'scripts/packaging/audited-notices'),upstream));
  if(pkg.name==='raylib'&&pkg.version==='6.0.0'&&pkg.source==='registry+https://github.com/rust-lang/crates.io-index')notices.push(...auditedSourceHeaders(dir,raylibHeaders));
  packages.push({name:pkg.name,version:pkg.version,license:pkg.license??null,origin:pkg.source?.startsWith('registry+')?'registry':pkg.source?'external':'local',root_mit_fallback:fallback,notices});
 }
 const missing=packages.filter(p=>!p.notices.length).map(p=>p.name+'@'+p.version);
 let text='Pong '+platform+' dependency notice preparation\n\nResolved Cargo graph; may include build/optional packages. All discovered license choices are retained without choosing one. This is not legal clearance or a system-library inventory. Nested source comments and unlabelled bundled code require separate review.\n';
 for(const p of packages)for(const f of p.notices)text+='\n===== '+p.name+' '+p.version+' | '+f.source+' | declared '+p.license+' =====\n'+f.text+'\n';
 if(Buffer.byteLength(text)>32*1024*1024)throw Error('Notice bundle too large');
 const index={schema:1,scope:'Resolved Cargo metadata, not linked-binary attribution',missing_notice_files:missing,bundle_sha256:digest(Buffer.from(text)),packages:packages.map(p=>({...p,notices:p.notices.map(({text,...f})=>f)}))};
 return {text,index};
}
