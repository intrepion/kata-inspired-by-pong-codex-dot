// Local ARM64 Dioxus pilot using the previously verified Arch Linux ARM xdotool archive.
import {chmodSync,copyFileSync,existsSync,mkdirSync,readFileSync,statSync,writeFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath,pathToFileURL} from 'node:url';
const root=resolve(fileURLToPath(new URL('../..',import.meta.url)));
const packageHash='0f8e61d3aa5c89e18b589fecfea2da1ff11defef8ca17b8ed237e5f5b8723268';
const libraryHash='cd0c434c67294dbe89a8793aee7b799400fc936a43416cc356afe10ff33642ad';
const hash=b=>createHash('sha256').update(b).digest('hex');
function extract(archive,name){const r=spawnSync('tar',['-xOf',archive,name],{maxBuffer:4*1024*1024});if(r.status!==0)throw Error('Cannot read verified dependency member');return r.stdout;}
export function packageDioxus(binary,dependencyArchive,output,source,profile='debug'){
 if(process.platform!=='linux'||process.arch!=='arm64')throw Error('ARM64 Linux pilot only');
 if(!['debug','release'].includes(profile))throw Error('Profile must be debug or release');
 if(!/^[a-f0-9]{40}$/.test(source??''))throw Error('Full source commit required');
 const b=readFileSync(binary),st=statSync(binary),out=resolve(output);
 if(!st.isFile()||!(st.mode&0o111)||b.length<20||!b.subarray(0,4).equals(Buffer.from([127,69,76,70]))||b[4]!==2||b[5]!==1||b.readUInt16LE(18)!==183)throw Error('ARM64 ELF executable required');
 if(hash(readFileSync(dependencyArchive))!==packageHash)throw Error('Unrecognized xdotool archive');
 const lib=extract(dependencyArchive,'usr/lib/libxdo.so.4'),copyright=extract(dependencyArchive,'usr/share/licenses/xdotool/COPYRIGHT');
 if(hash(lib)!==libraryHash||!copyright.length)throw Error('Invalid dependency members');
 if(existsSync(out))throw Error('Output already exists');
 mkdirSync(out);const name='pong-dioxus-local-xdo-linux-arm64',stage=join(out,name);mkdirSync(stage);
 copyFileSync(binary,join(stage,'pong-dioxus'));chmodSync(join(stage,'pong-dioxus'),0o755);
 copyFileSync(join(root,'LICENSE'),join(stage,'LICENSE'));writeFileSync(join(stage,'libxdo.so.4'),lib);writeFileSync(join(stage,'XDO-COPYRIGHT'),copyright);
 writeFileSync(join(stage,'run-pong-dioxus'),`#!/bin/sh\nset -eu\ndir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)\nLD_LIBRARY_PATH="$dir\${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" exec "$dir/pong-dioxus" "$@"\n`,{mode:0o755});
 writeFileSync(join(stage,'README.txt'),`Local ARM64 Linux ${profile} pilot; not a public release.\nRun ./run-pong-dioxus from this directory. The launcher selects the included\nlibxdo.so.4 without installing it. Other compatible WebKitGTK 4.1, GTK3, X11\nand graphics/runtime libraries are still required. --scripted-test is a\nbounded model replay; --scripted-scenes exports SVG source, not screenshots.\nWebKit may create application data and cache outside this folder. Inspect state\nseparately and preserve unrelated files. Full notice and distribution review,\nclean-machine testing, and real input remain incomplete.\n`);
 writeFileSync(join(stage,'BUILD-INFO.txt'),`source_commit=${source}\nbinary_sha256=${hash(b)}\nxdotool_package=4.20260303.1-1\nxdotool_package_sha256=${packageHash}\nlibxdo_sha256=${libraryHash}\nprofile=${profile}\nartifact_scope=local portable pilot; not public release\nprovenance_limit=supplied commit and profile are not proof for arbitrary binary input\n`);
 const files=['pong-dioxus','LICENSE','libxdo.so.4','XDO-COPYRIGHT','run-pong-dioxus','README.txt','BUILD-INFO.txt'];writeFileSync(join(stage,'SHA256SUMS'),files.map(f=>`${hash(readFileSync(join(stage,f)))}  ${f}\n`).join(''));
 const archive=join(out,name+'.tar.gz');const r=spawnSync('tar',['-C',out,'-czf',archive,name]);if(r.status!==0)throw Error('Archive creation failed');writeFileSync(archive+'.sha256',`${hash(readFileSync(archive))}  ${name}.tar.gz\n`);
 return {archive,archive_sha256:hash(readFileSync(archive)),size:statSync(archive).size,binary_sha256:hash(b),libxdo_sha256:hash(lib)};
}
if(process.argv[1]&&import.meta.url===pathToFileURL(process.argv[1]).href){const [binary,dep,out,source,profile,...extra]=process.argv.slice(2);if(!binary||!dep||!out||!source||extra.length)throw Error('Usage: dioxus-local-xdo.mjs BINARY VERIFIED_XDOTOOL_ARCHIVE NEW_OUTPUT SOURCE_COMMIT [debug|release]');console.log(JSON.stringify(packageDioxus(binary,dep,out,source,profile),null,2));}
