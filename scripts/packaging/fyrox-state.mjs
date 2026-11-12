// Read-only cleanup planning. This module never removes generated state.
import {lstatSync,readFileSync,readdirSync,realpathSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {createHash} from 'node:crypto';
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
function stat(path){try{return lstatSync(path);}catch(e){if(e.code==='ENOENT')return null;throw e;}}
function rootIdentity(root){const path=resolve(root),s=lstatSync(path);if(!s.isDirectory()||s.isSymbolicLink()||realpathSync(path)!==path)throw Error('Run directory must be a real directory without symlink ancestors');return {path,dev:s.dev,ino:s.ino};}
function inspect(root){
 const data=join(root,'data'),s=stat(data);
 if(!s)return {kind:'absent'};
 if(s.isSymbolicLink()||!s.isDirectory())return {kind:'unsafe'};
 const file=join(data,'resources.registry'),f=stat(file);
 if(!f)return {kind:'absent',data_existed:true};
 if(f.isSymbolicLink()||!f.isFile()||f.nlink!==1||f.size>1024*1024)return {kind:'unsafe'};
 const bytes=readFileSync(file);
 return {kind:'file',bytes:bytes.length,sha256:hash(bytes),empty_registry:bytes.equals(Buffer.from('{}')),data_entries:readdirSync(data).sort()};
}
export function beforeFyroxRun(root){return {version:1,root:rootIdentity(root),before:inspect(resolve(root))};}
export function planFyroxCleanup(root,receipt=null){
 const id=rootIdentity(root),current=inspect(id.path);
 const result={action:'preserve',reason:'No pre-run ownership evidence',relative_path:'data/resources.registry',current};
 if(!receipt||receipt.version!==1||JSON.stringify(id)!==JSON.stringify(receipt.root))return result;
 if(receipt.before.kind!=='absent')return {...result,reason:'Registry or unsafe entry existed before this run'};
 if(current.kind==='absent')return {...result,action:'none',reason:'No registry file exists'};
 if(current.kind!=='file'||!current.empty_registry)return {...result,reason:'Unsafe or nonempty generated state must be preserved'};
 return {...result,action:'review-candidate',reason:'New exact empty registry observed after bounded run; not proof of exclusive ownership',expected_sha256:current.sha256,may_remove_data_directory:receipt.before.data_existed!==true&&current.data_entries.length===1};
}
