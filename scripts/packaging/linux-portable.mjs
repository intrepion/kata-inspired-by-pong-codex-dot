// Local portable lifecycle pilots only: no system installation or driver bundling.
import {copyFileSync,chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,statSync,writeFileSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
const adapters=Object.freeze({miniquad:'pong-miniquad',pixels:'pong-pixels',wgpu:'pong-wgpu',raylib:'pong-raylib-rs',piston:'pong-piston','bracket-lib':'pong-bracket-lib',bevy:'pong-bevy','blue-engine':'pong-blue-engine',softbuffer:'pong-softbuffer',ash:'pong-ash',sdl3:'pong-sdl3',ratatui:'pong-ratatui'});
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const digest=p=>createHash('sha256').update(readFileSync(p)).digest('hex');
// Inspect the private staged copy, not a path that is copied again afterward.
function verifyStagedElf(binary,source) {
  const hostMachine={x64:'Advanced Micro Devices X86-64',arm64:'AArch64'}[process.arch];
  if(!hostMachine) throw Error('Unsupported Linux pilot architecture');
  const helper=fileURLToPath(new URL('./elf_compat.py',import.meta.url));
  const probe=spawnSync('python3',['-I','-B',helper,binary,'--source-revision',source],
    {encoding:'utf8',timeout:35000,maxBuffer:1024*1024});
  if(probe.error||probe.status!==0||probe.stderr?.trim())
    throw Error('ELF format or architecture verification failed: '+(probe.error?.message??probe.stderr?.trim()??'no valid report'));
  let report;
  try { report=JSON.parse(probe.stdout); } catch { throw Error('Invalid ELF compatibility report'); }
  if(report?.schema!==1 || report.machine!==hostMachine || report.class!=='ELF64'
     || report.data!=="2's complement, little endian")
    throw Error('Executable ELF format or architecture does not match this Linux pilot');
  if(!/^(?:EXEC\b|DYN\s+\(Position-Independent Executable file\)$)/.test(report.type??''))
    throw Error('Input is not an ELF executable or PIE');
  const bytes=readFileSync(binary);
  if(bytes[6]!==1) throw Error('Unsupported ELF format version or architecture');
  verifyElfEntryPoint(bytes);
  const hash=createHash('sha256').update(bytes).digest('hex');
  if(report.sha256!==hash) throw Error('Staged ELF changed during inspection');
  return hash;
}
// ELF64: require a complete program header table and an entry point in a
// file-backed executable PT_LOAD. GNU readelf classifies ET_DYN PIE separately
// from ordinary shared libraries, including static PIE without an interpreter.
function verifyElfEntryPoint(bytes) {
  if(bytes.length<64) throw Error('Incomplete ELF64 header');
  const limit=BigInt(bytes.length), start=bytes.readBigUInt64LE(32);
  const size=bytes.readUInt16LE(54), count=bytes.readUInt16LE(56);
  if(start<64n||size!==56||count===0||count===0xffff||start+BigInt(size*count)>limit)
    throw Error('Invalid ELF64 program header table');
  const entry=bytes.readBigUInt64LE(24);
  let hasEntry=false;
  for(let i=0;i<count;i++){
    const at=Number(start)+i*size;
    if(bytes.readUInt32LE(at)!==1) continue; // PT_LOAD
    const flags=bytes.readUInt32LE(at+4), offset=bytes.readBigUInt64LE(at+8);
    const address=bytes.readBigUInt64LE(at+16), fileSize=bytes.readBigUInt64LE(at+32);
    const memorySize=bytes.readBigUInt64LE(at+40);
    if(offset+fileSize>limit||fileSize>memorySize) throw Error('Invalid ELF64 load segment');
    if((flags&1)!==0&&entry>=address&&entry<address+fileSize) hasEntry=true; // PF_X
  }
  if(!hasEntry) throw Error('ELF entry point is not in an executable load segment');
}
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
  const executable=adapters[adapter], archiveName=name+'.tar.gz';
  const staging=mkdtempSync(join(dirname(out),'.pong-linux-portable-'));
  try {
    const payload=join(staging,'output'),stage=join(payload,name);
    mkdirSync(payload);mkdirSync(stage);
    const stagedBinary=join(stage,executable);
    copyFileSync(binary,stagedBinary);chmodSync(stagedBinary,0o755);
    const binaryHash=verifyStagedElf(stagedBinary,source);
    copyFileSync(join(root,'LICENSE'),join(stage,'LICENSE'));
    writeFileSync(join(stage,'README.txt'),`${executable}: local Linux portable pilot\n\nRun ./${executable} from this folder. --scripted-test runs a bounded seeded replay.\n${adapter==='ratatui'?'Run in an interactive terminal (minimum 50 columns x 18 rows). --scripted-cells NEW_DIRECTORY exports terminal cells, not screenshots.':adapter==='blue-engine'?'Native scripted screenshots are not implemented for this adapter.':'--scripted-captures NEW_DIRECTORY additionally writes application-owned captures.'}\nNormal controls are shown by the application.\n\nRequires compatible CPU architecture, Linux runtime, ${adapter==='ratatui'?'and an interactive terminal.':'graphics drivers and display.'}\nDrivers/runtime libraries are not bundled or installed. Not a public release or\ncross-distro guarantee. Root MIT included; ${notices?'resolved-graph notice texts included, review still required.':'full dependency notices remain pending.'}\n\nInstallation is extraction only; no settings, services or PATH changes. Verify\nSHA256SUMS before launch. Uninstall only the ${4+(notices?2:0)} files listed there plus\nSHA256SUMS itself, then remove the empty dedicated folder; preserve other files.\n`);
    writeFileSync(join(stage,'BUILD-INFO.txt'),`source_commit=${source}\nadapter=${adapter}\narchitecture=${process.arch}\nbinary_sha256=${binaryHash}\nartifact_scope=local portable pilot; not public release\nprovenance_limit=supplied commit is not proof for arbitrary binary input\n`);
    const files=[executable,'LICENSE','README.txt','BUILD-INFO.txt'];
    if(notices){writeFileSync(join(stage,'THIRD-PARTY-NOTICES.txt'),notices.text);writeFileSync(join(stage,'NOTICE-INDEX.json'),JSON.stringify(notices.index,null,2)+'\n');files.push('THIRD-PARTY-NOTICES.txt','NOTICE-INDEX.json');}
    writeFileSync(join(stage,'SHA256SUMS'),files.map(f=>`${f===executable?binaryHash:digest(join(stage,f))}  ${f}\n`).join(''));
    const archive=join(payload,archiveName);
    const result=spawnSync('tar',['-C',payload,'-czf',archive,name],{encoding:'utf8'});
    if(result.status!==0) throw Error(`Archive creation failed: ${result.error?.message??result.stderr}`);
    if(digest(stagedBinary)!==binaryHash) throw Error('Staged ELF changed while archiving');
    const archiveHash=digest(archive),size=statSync(archive).size;
    writeFileSync(archive+'.sha256',`${archiveHash}  ${archiveName}\n`);
    mkdirSync(out); // Reserve atomically; never replace an existing user directory.
    renameSync(payload,out); // Publish the complete, previously private staged output.
    return {archive:join(out,archiveName),binary_sha256:binaryHash,archive_sha256:archiveHash,size};
  } finally {
    rmSync(staging,{recursive:true,force:true});
  }
}
if(process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href){
  const [adapter,binary,out,source,...extra]=process.argv.slice(2);
  if(!adapter||!binary||!out||!source||extra.length) throw Error('Usage: linux-portable.mjs ADAPTER RELEASE_BINARY NEW_OUTPUT_DIRECTORY SOURCE_COMMIT');
  console.log(JSON.stringify(packagePortable(adapter,binary,out,source),null,2));
}
