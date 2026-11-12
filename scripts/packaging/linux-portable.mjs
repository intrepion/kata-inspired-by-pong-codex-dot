// Local portable lifecycle pilots only: no system installation or driver bundling.
import {copyFileSync,chmodSync,existsSync,mkdirSync,readFileSync,realpathSync,statSync,writeFileSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
const adapters=Object.freeze({miniquad:'pong-miniquad',pixels:'pong-pixels',wgpu:'pong-wgpu',raylib:'pong-raylib-rs',piston:'pong-piston',ggez:'pong-ggez',nannou:'pong-nannou','bracket-lib':'pong-bracket-lib',fyrox:'pong-fyrox',bevy:'pong-bevy','blue-engine':'pong-blue-engine',softbuffer:'pong-softbuffer',ash:'pong-ash',sdl3:'pong-sdl3',ratatui:'pong-ratatui',gtk:'pong-gtk',slint:'pong-slint'});
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const digest=p=>createHash('sha256').update(readFileSync(p)).digest('hex');
export function packagePortable(adapter,input,output,source,notices=null) {
  if(process.platform!=='linux') throw Error('Linux only');
  if(!Object.hasOwn(adapters,adapter)) throw Error('Unsupported adapter');
  if(!/^[a-f0-9]{40}$/.test(source??'')) throw Error('Full source commit required');
  if(notices){
    if(typeof notices.text!=='string'||Buffer.byteLength(notices.text)>32*1024*1024||notices.index?.bundle_sha256!==createHash('sha256').update(notices.text).digest('hex')||!Array.isArray(notices.index?.missing_notice_files)||notices.index.missing_notice_files.length)throw Error('Invalid or incomplete notice bundle');
  }
  const binary=realpathSync(input), info=statSync(binary), out=resolve(output);
  if(!info.isFile() || !(info.mode&0o111)) throw Error('Executable regular file required');
  if(existsSync(out)) throw Error('Output already exists');
  const name=`${adapters[adapter]}-linux-${process.arch}`;
  mkdirSync(out); // No recursive overwrite of existing content.
  const stage=join(out,name);mkdirSync(stage);
  const executable=adapters[adapter];
  copyFileSync(binary,join(stage,executable));chmodSync(join(stage,executable),0o755);
  copyFileSync(join(root,'LICENSE'),join(stage,'LICENSE'));
  writeFileSync(join(stage,'README.txt'),`${executable}: local Linux portable pilot\n\nRun ./${executable} from this folder. --scripted-test runs a bounded seeded replay.\n${adapter==='slint'?'--scripted-captures writes Slint own-window component snapshots, not compositor screenshots. SLINT-LICENSE.md is included; dependency review remains required.':adapter==='gtk'?'Requires compatible GTK4 runtime libraries (not bundled). --scripted-captures exports app-owned Cairo render output, not desktop screenshots.':adapter==='ratatui'?'Run in an interactive terminal (minimum 50 columns x 18 rows). --scripted-cells NEW_DIRECTORY exports terminal cells, not screenshots.':adapter==='blue-engine'?'Native scripted screenshots are not implemented for this adapter.':'--scripted-captures NEW_DIRECTORY additionally writes application-owned captures.'}\nNormal controls are shown by the application.\n\nRequires compatible CPU architecture, Linux runtime, ${adapter==='ratatui'?'and an interactive terminal.':'graphics drivers and display.'}\nDrivers/runtime libraries are not bundled or installed. Not a public release or\ncross-distro guarantee. Root MIT included; ${notices?'resolved-graph notice texts included, review still required.':'full dependency notices remain pending.'}\n\nInstallation is extraction only; no settings, services or PATH changes. Verify\nSHA256SUMS before launch. Uninstall only the ${4+(notices?2:0)+(adapter==='slint'?1:0)} files listed there plus\nSHA256SUMS itself, then remove the empty dedicated folder; preserve other files.\n`);
  writeFileSync(join(stage,'BUILD-INFO.txt'),`source_commit=${source}\nadapter=${adapter}\narchitecture=${process.arch}\nbinary_sha256=${digest(binary)}\nartifact_scope=local portable pilot; not public release\nprovenance_limit=supplied commit is not proof for arbitrary binary input\n`);
  const files=[executable,'LICENSE','README.txt','BUILD-INFO.txt'];
  if(adapter==='slint'){copyFileSync(join(root,'experiments/gui-slint/LICENSE-SLINT.md'),join(stage,'SLINT-LICENSE.md'));files.push('SLINT-LICENSE.md');}
  if(notices){writeFileSync(join(stage,'THIRD-PARTY-NOTICES.txt'),notices.text);writeFileSync(join(stage,'NOTICE-INDEX.json'),JSON.stringify(notices.index,null,2)+'\n');files.push('THIRD-PARTY-NOTICES.txt','NOTICE-INDEX.json');}
  writeFileSync(join(stage,'SHA256SUMS'),files.map(f=>`${digest(join(stage,f))}  ${f}\n`).join(''));
  const archive=join(out,name+'.tar.gz');
  const result=spawnSync('tar',['-C',out,'-czf',archive,name],{encoding:'utf8'});
  if(result.status!==0) throw Error(`Archive creation failed: ${result.error?.message??result.stderr}`);
  writeFileSync(archive+'.sha256',`${digest(archive)}  ${name}.tar.gz\n`);
  return {archive,binary_sha256:digest(binary),archive_sha256:digest(archive),size:statSync(archive).size};
}
if(process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href){
  const [adapter,binary,out,source,...extra]=process.argv.slice(2);
  if(!adapter||!binary||!out||!source||extra.length) throw Error('Usage: linux-portable.mjs ADAPTER RELEASE_BINARY NEW_OUTPUT_DIRECTORY SOURCE_COMMIT');
  console.log(JSON.stringify(packagePortable(adapter,binary,out,source),null,2));
}
