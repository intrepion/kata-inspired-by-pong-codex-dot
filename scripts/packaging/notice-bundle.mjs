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
export function buildNoticeBundle(metadata,repoRoot) {
 if(!Array.isArray(metadata.packages)||!Array.isArray(metadata.resolve?.nodes)) throw Error('Resolved Cargo metadata required');
 const wanted=new Set(metadata.resolve.nodes.map(x=>x.id)),root=realpathSync(repoRoot),packages=[];
 for(const pkg of metadata.packages.filter(p=>wanted.has(p.id)).sort((a,b)=>a.name.localeCompare(b.name)||a.version.localeCompare(b.version))) {
  if(!/^[a-zA-Z0-9_-]+$/.test(pkg.name)||!pkg.version||!pkg.manifest_path) throw Error('Invalid package metadata');
  const dir=realpathSync(dirname(pkg.manifest_path));let files=candidates(dir);
  if(pkg.license_file)files.push(resolve(dir,pkg.license_file));
  files=[...new Set(files)].sort();
  let fallback=false;
  if(!files.length&&!pkg.source&&pkg.name.startsWith('pong-')&&pkg.license==='MIT'){
   const rel=relative(root,dir);if(rel==='..'||rel.startsWith('../')||isAbsolute(rel))throw Error('First-party fallback outside repository');
   files=[join(root,'LICENSE')];fallback=true;
  }
  const notices=files.map(f=>safeFile(fallback?root:dir,f));
  packages.push({name:pkg.name,version:pkg.version,license:pkg.license??null,origin:pkg.source?.startsWith('registry+')?'registry':pkg.source?'external':'local',root_mit_fallback:fallback,notices});
 }
 const missing=packages.filter(p=>!p.notices.length).map(p=>p.name+'@'+p.version);
 let text='Pong Linux dependency notice preparation\n\nResolved Cargo graph; may include build/optional packages. All discovered license choices are retained without choosing one. This is not legal clearance or a system-library inventory. Nested source comments and unlabelled bundled code require separate review.\n';
 for(const p of packages)for(const f of p.notices)text+='\n===== '+p.name+' '+p.version+' | '+f.source+' | declared '+p.license+' =====\n'+f.text+'\n';
 if(Buffer.byteLength(text)>32*1024*1024)throw Error('Notice bundle too large');
 const index={schema:1,scope:'Resolved Cargo metadata, not linked-binary attribution',missing_notice_files:missing,bundle_sha256:digest(Buffer.from(text)),packages:packages.map(p=>({...p,notices:p.notices.map(({text,...f})=>f)}))};
 return {text,index};
}
