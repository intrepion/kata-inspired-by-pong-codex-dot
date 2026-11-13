// Preparation aid, not legal clearance or proof of linked-code attribution.
import {readFileSync,readdirSync,lstatSync,realpathSync} from 'node:fs';
import {dirname,relative,resolve,join,isAbsolute,extname} from 'node:path';
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
// Conventional names with source-code or structured-data extensions are not notice texts.
const nonNoticeExtensions=new Set(['.c','.cc','.cpp','.h','.hpp','.rs','.js','.cjs','.mjs','.ts','.tsx','.jsx','.py','.sh','.ps1','.json','.toml','.yaml','.yml','.xml','.lock','.wasm','.exe','.dll']);
function candidates(root) {
 const found=[];
 function visit(dir) {
  for(const e of readdirSync(dir,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name))){
   if(e.name==='.git'||e.name==='target')continue;
   const file=join(dir,e.name);
   if(e.isDirectory()) visit(file);
   else if(/^(?:licen[sc]e|copying|notice|copyright)(?:[._-]|$)/i.test(e.name) && !nonNoticeExtensions.has(extname(e.name).toLowerCase())) found.push(file);
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
const gpuDescriptorFiles=Object.freeze([['gpu-descriptor-COPYING.txt','f72143f7e9d9fb8bd336b139fa22f825a299591c29f2f7338c5566347b137613','COPYING'],['gpu-descriptor-APACHE.txt','6bec8c944a6d4251be50ac86bb4ca360bcce5aea472505dee2800e160bf1cdbf','license/APACHE'],['gpu-descriptor-MIT.txt','5226e27d5fbc683db56c52853cdb9d15b69abadb525a60f95c862c2dbd77cdc8','license/MIT']]);
const accesskitFiles=(rev,dual)=>[
 ['accesskit-LICENSE-APACHE.txt','62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a',`https://github.com/AccessKit/accesskit/blob/${rev}/LICENSE-APACHE`],
 ...(dual?[
  ['accesskit-LICENSE-MIT.txt','23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3',`https://github.com/AccessKit/accesskit/blob/${rev}/LICENSE-MIT`],
  ['accesskit-LICENSE.chromium.txt','845022e0c1db1abb41a6ba4cd3c4b674ec290f3359d9d3c78ae558d4c0ed9308',`https://github.com/AccessKit/accesskit/blob/${rev}/LICENSE.chromium`],
 ]:[]),
];
const lyonFiles=rev=>[
 ['lyon-LICENSE-APACHE.txt','216486f29671a4262efe32af6d84a75bef398127f8c5f369b5c8305983887a06',`https://github.com/nical/lyon/blob/${rev}/LICENSE-APACHE`],
 // Lyon's Apache file is a short notice; include the full terms from this separately audited copy.
 ['gl-rs-LICENSE.txt','cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30','https://github.com/rust-windowing/gl-rs/blob/ea503e8d5fb6d73c6030e6191ce738cd3bf3433e/LICENSE'],
 ['lyon-LICENSE-MIT.txt','80f275e90d799911ed3830a7f242a2ef5a4ade2092fe0aa07bfb2d2cf2f2b95e',`https://github.com/nical/lyon/blob/${rev}/LICENSE-MIT`],
];
const upstreamRules=Object.freeze({
 'clipboard-win@5.4.3':{rev:'ed27008ae5d1f10cfbb266de039f0fb35fd05a56',path:'',sourceHash:'59957f466589d8be7999593f94654217d9bfb4680ef981c02e5f34807583a695',files:[['clipboard-win-LICENSE.txt','c9bff75738922193e67fa726fa225535870d2aa1059f91452c411736284ad566','https://github.com/DoumanAsh/clipboard-win/blob/ed27008ae5d1f10cfbb266de039f0fb35fd05a56/LICENSE']]},
 'svg_fmt@0.4.5':{rev:'93f414cd572b5c2781df93dbdc068ed13443d05a',path:'svg_fmt',sourceHash:'b891c7958cb7d4afcea8bd214987bf784bd44decc24981934e19f4416c3c8720',files:[['rust-debug-LICENSE.txt','7e98bed7f100747aaae7d82ab94fbe6b76fd6b84b09a3aaa413bff8196d0fbb1','https://github.com/nical/rust_debug/blob/93f414cd572b5c2781df93dbdc068ed13443d05a/LICENSE']]},
 'bevy_scene_macros@0.19.1':{rev:'b56fc29d3016e641754765244b5ba3f9cc504671',path:'crates/bevy_scene/macros',sourceHash:'f7a2da36d32ccb4cf0ad6c62776b1f96a0dc62518320d7dfb43da0558616c7b5',files:[['bevy-LICENSE-APACHE.txt','a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9','https://github.com/bevyengine/bevy/blob/b56fc29d3016e641754765244b5ba3f9cc504671/LICENSE-APACHE'],['bevy-LICENSE-MIT.txt','508a77d2e7b51d98adeed32648ad124b7b30241a8e70b2e72c99f92d8e5874d1','https://github.com/bevyengine/bevy/blob/b56fc29d3016e641754765244b5ba3f9cc504671/LICENSE-MIT']]},
 'taffy@0.10.1':{rev:'639f0acdd20f914f847abffbeaa903d66cae4882',path:'',sourceHash:'bc46b7401339c1de2dfb68b3e02eeb1c7a6501b1508543b7eb12910cfc2a4a07',files:[['taffy-LICENSE.md','a7083ef3dd41a65eeb2162c2c091b0082e2724fb4c6ed029b363c7840d244ecd','https://github.com/DioxusLabs/taffy/blob/639f0acdd20f914f847abffbeaa903d66cae4882/LICENSE.md']]},
 'encase_derive@0.12.1':{rev:'561121cd17298355de74c07a7206eb8329fd8347',path:'derive',sourceHash:'23f117b3a248546248101b428da696ed1b4569720778e64e64238969a18df9df',files:[['encase-LICENSE.txt','8876cdca96eb380d16d5ea32bf4529d85e87dedc7765ae2861a9bec7f34acbed','https://github.com/teoxoy/encase/blob/561121cd17298355de74c07a7206eb8329fd8347/LICENSE']]},
 'encase_derive_impl@0.12.1':{rev:'561121cd17298355de74c07a7206eb8329fd8347',path:'derive/impl',sourceHash:'01b6b10495231854af9bdf304acaa0cbea88cb8a77b7be094e2b47ce5939408c',files:[['encase-LICENSE.txt','8876cdca96eb380d16d5ea32bf4529d85e87dedc7765ae2861a9bec7f34acbed','https://github.com/teoxoy/encase/blob/561121cd17298355de74c07a7206eb8329fd8347/LICENSE']]},
 'lyon@1.0.19':{rev:'56219c95c658d4f39de7b190d98e57d5773ba241',path:'crates/lyon',sourceHash:'afbe4e0241db801f1e26409d49da6142156a6ac539523d79b6839a177cc2ab14',files:lyonFiles('56219c95c658d4f39de7b190d98e57d5773ba241')},
 'lyon_path@1.0.19':{rev:'56219c95c658d4f39de7b190d98e57d5773ba241',path:'crates/path',sourceHash:'82b7348e3343de1a6cddd4b0f1f4e96cfaa118d77146921e6d6589abd790d02e',files:lyonFiles('56219c95c658d4f39de7b190d98e57d5773ba241')},
 'lyon_algorithms@1.0.22':{rev:'db98d26a1a90076a7a894262dcb51117eef643c8',path:'crates/algorithms',sourceHash:'752ee316c0b45a3ccd0fd4d839ee01705b77d4a5ba229390e610513f798dd9c1',files:lyonFiles('db98d26a1a90076a7a894262dcb51117eef643c8')},
 'lyon_geom@1.0.20':{rev:'db98d26a1a90076a7a894262dcb51117eef643c8',path:'crates/geom',sourceHash:'864bb699e6c303724518c79d9054ac74b137a7dbf698b052662f802c6d9b83f5',files:lyonFiles('db98d26a1a90076a7a894262dcb51117eef643c8')},
 'lyon_tessellation@1.0.23':{rev:'db98d26a1a90076a7a894262dcb51117eef643c8',path:'crates/tessellation',sourceHash:'da60b61c97a54d2eff932d22f9d87e86a4e8e90ba82e12da72ad03127058bb8b',files:lyonFiles('db98d26a1a90076a7a894262dcb51117eef643c8')},
 'accesskit@0.24.1':{rev:'a55d3e1a18bb9ef0e4bccc9083fb13c3e0ad8969',path:'common',sourceHash:'da99bad8c05747eee4c1cee0850488c4e61170f7f55cbf6b39e79cd4e4a1acbf',files:accesskitFiles('a55d3e1a18bb9ef0e4bccc9083fb13c3e0ad8969',true)},
 'accesskit_consumer@0.35.0':{rev:'1bbcf100942bac96c2c3a4a91cb67b0b20201a24',path:'consumer',sourceHash:'f7c619c1c2e4add099d4dece693b36be535262a1f696a188e6040ad6877e34bf',additionalSources:[['src/iterators.rs','22233da2a6c999c355b74edddbf80fc98c6f32bd52f55de2bacd523f013824a7'],['src/node.rs','7ca5b53a821529e168af71f63d064ef20308a3634e019c8ce4f1c965c2b483cf']],files:accesskitFiles('1bbcf100942bac96c2c3a4a91cb67b0b20201a24',true)},
 'accesskit_windows@0.32.1':{rev:'1bbcf100942bac96c2c3a4a91cb67b0b20201a24',path:'platforms/windows',sourceHash:'8a6f20ed5354c9834d88a59091c764f65aa31359f81c57386fe06e9d806e9396',additionalSources:[['src/node.rs','e2e558a8f7ce54cd7d5b66e95cd65f42f7ee2533ece0b7796a999348f6640e49']],files:accesskitFiles('1bbcf100942bac96c2c3a4a91cb67b0b20201a24',true)},
 'accesskit_winit@0.32.2':{rev:'1bbcf100942bac96c2c3a4a91cb67b0b20201a24',path:'platforms/winit',sourceHash:'cbe4ef8dde0a3aa3d9777e51f86eeff2eb4396e290ab5ea83494920b8005b631',files:accesskitFiles('1bbcf100942bac96c2c3a4a91cb67b0b20201a24',false)},
 'spirv@0.4.0+sdk-1.4.341.0':{rev:'8afc3d0ac8e158128cd1410bb2e4b4c26ab11bb4',path:'spirv',sourcePath:'lib.rs',sourceHash:'54127a130c4ef2cdbf7882a1cd6ebde88ae09b755bd8a1c086e21101da9684e9',additionalSources:[['autogen_spirv.rs','2260b34bd7b8a757be33f835ecc7431f7b791c39e9fa30a0acf47c03ae60b436']],files:[['gl-rs-LICENSE.txt','cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30','https://github.com/gfx-rs/rspirv/blob/8afc3d0ac8e158128cd1410bb2e4b4c26ab11bb4/LICENSE']]},
 'profiling@1.0.18':{rev:'8271551172eb6fa4cba47369aedd93790c623df9',path:'profiling',sourceHash:'2eb645fff043b95ac6e37b5304f0d1b0fe6d7935c68a644f21c4d6dd83aa1907',files:[['profiling-LICENSE-APACHE.txt','10d30a673cd5e9349bdc02aeb48f14b3386d27d0da32df8f0a555d4aa16aa551','https://github.com/aclysma/profiling/blob/8271551172eb6fa4cba47369aedd93790c623df9/LICENSE-APACHE'],['profiling-LICENSE-MIT.txt','c8167fdeeed46d3f244d3f85c5bf998ce889343691c32be2c61a8bc4b5c08333','https://github.com/aclysma/profiling/blob/8271551172eb6fa4cba47369aedd93790c623df9/LICENSE-MIT']]},
 'profiling-procmacros@1.0.18':{rev:'8271551172eb6fa4cba47369aedd93790c623df9',path:'profiling-procmacros',sourceHash:'98667c1d5ef4ad26ef7fa273c5d8b38bdc9046cee6b10621b128e2d45496efc5',files:[['profiling-LICENSE-APACHE.txt','10d30a673cd5e9349bdc02aeb48f14b3386d27d0da32df8f0a555d4aa16aa551','https://github.com/aclysma/profiling/blob/8271551172eb6fa4cba47369aedd93790c623df9/LICENSE-APACHE'],['profiling-LICENSE-MIT.txt','c8167fdeeed46d3f244d3f85c5bf998ce889343691c32be2c61a8bc4b5c08333','https://github.com/aclysma/profiling/blob/8271551172eb6fa4cba47369aedd93790c623df9/LICENSE-MIT']]},
 'zune-inflate@0.2.54':{rev:'69502ce83fdfecdd0beefd677e2abb3781b29d98',path:'zune-inflate',sourceHash:'ee2b5b58f48ee36835bd4e312a26dace36be39d1c4af956f4e2e04d0620667f9',files:[['zune-image-LICENSE.md','e870d7fd7819a9f0453c62798cb52ae95d148a8c91dc2de1d983f997143c72de','https://github.com/etemesi254/zune-image/blob/69502ce83fdfecdd0beefd677e2abb3781b29d98/LICENSE.md'],['zune-image-LICENSE-ZLIB.txt','7fa429541e55b1509909e058f2d21a37467e4958ec713b357f6e0cf9dc4ee352','https://github.com/etemesi254/zune-image/blob/69502ce83fdfecdd0beefd677e2abb3781b29d98/LICENSE-ZLIB']]},
 'gpu-descriptor@0.3.2':{rev:'bb28b4e55b742e0ea3e7a0968e7ee6287ba0e06a',path:'gpu-descriptor',sourceHashes:['6fb74a08ad9975e561f4fca7bd391f0cbd96a7cab79b17df7d979021099b50f9','86d91995ea56d2ccf12974cb28f02028d096e00acbe417b1acdaa0d7133ae9f2'],files:gpuDescriptorFiles.map(([f,h,p])=>[f,h,'https://github.com/zakarumych/gpu-descriptor/blob/bb28b4e55b742e0ea3e7a0968e7ee6287ba0e06a/'+p])},
 'gpu-descriptor-types@0.2.0':{rev:'9f2fb9546591322e7b77ee1e79385e01effd85e4',path:'types',sourceHashes:['0ca3f2a281ba43466fb4d29c05b35c9059806d6114cd929b85afc0c41ea9d3ac','bcf06593c390a3999f300333f63ae69542613157d316b3246faf69eded07d896'],files:gpuDescriptorFiles.map(([f,h,p])=>[f,h,'https://github.com/zakarumych/gpu-descriptor/blob/9f2fb9546591322e7b77ee1e79385e01effd85e4/'+p])},
 'gl_generator@0.14.0':{rev:'ea503e8d5fb6d73c6030e6191ce738cd3bf3433e',path:undefined,sourcePath:'lib.rs',sourceHash:'6f47da878479b503b5202bda84ad8935607264ac03c0b68203109fad9cd479db',files:[['gl-rs-LICENSE.txt','cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30','https://github.com/rust-windowing/gl-rs/blob/ea503e8d5fb6d73c6030e6191ce738cd3bf3433e/LICENSE']]},
 'khronos_api@3.1.0':{rev:'f150967b1c44ae888e6676f93f639ebc82771bdc',path:undefined,sourceHash:'befbc0e187097e6c9da1ecbd79e0e1af462b6f6466108247745f6aa38abe0673',files:[['gl-rs-LICENSE.txt','cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30','https://github.com/rust-windowing/gl-rs/blob/f150967b1c44ae888e6676f93f639ebc82771bdc/LICENSE']]},
 'blue_engine_core@0.10.0':{rev:'3009d16d2429064d87d8cb40b9814bd595bb683f',path:'crates/blue_engine_core',sourceHash:'249d232154713cc1fc5d2e7855cc00a2e594b1cd56885a0707756020ce8cf92f',files:[['blue-engine-LICENSE.txt','62048053433f2ed52d4162d67b28d497a74b6a0c85c709569557f01cd8a02146','https://github.com/AryanpurTech/BlueEngine/blob/3009d16d2429064d87d8cb40b9814bd595bb683f/LICENSE.txt']]},
 'blue_engine_dynamic@0.10.0':{rev:'e3ee0f8d8f38a9ff63d09f941d2f542683078194',path:'crates/blue_engine_dynamic',sourcePath:'Cargo.toml.orig',sourceHash:'c76e4fe32e6a742d4529d3ee7a731470d92a5324cf36529ead8a702c1d96aea5',files:[['blue-engine-LICENSE.txt','62048053433f2ed52d4162d67b28d497a74b6a0c85c709569557f01cd8a02146','https://github.com/AryanpurTech/BlueEngine/blob/e3ee0f8d8f38a9ff63d09f941d2f542683078194/LICENSE.txt']]},
 'selectors@0.38.0':{rev:'572ecba2d1600e7c3d490586692a209faf703baa',path:'selectors',sourcePath:'lib.rs',sourceHash:'d54c6e13e9e952dac17d209171df8657e3cae93beaddace4906150cdec8d02e9',files:[['mozilla-MPL-2.0.txt','fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85','https://github.com/mozilla/openbadges-bakery/blob/3057b06f75997c22ad4f039960dd2688578c3d32/LICENSE-MPL-2.0']]},
 'alloc-stdlib@0.3.0':{rev:'0a81fd6928ea3b33c8cd484aa4575d50ffb98012',path:'alloc-stdlib',sourceHash:'3b1358fd99790a1063f0f2b4358d3771a75fcc3ee8e4560984326f86a8c56b0a',files:[['alloc-stdlib-LICENSE.txt','c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae','https://github.com/dropbox/rust-alloc-no-stdlib/blob/0a81fd6928ea3b33c8cd484aa4575d50ffb98012/LICENSE']]},
 'dlopen2@0.8.2':{rev:'cc80e4a0a90d499b677fdf7743699b4b3a43a989',path:'dlopen2',sourceHash:'e99da1fe29c4c9391b6e0f2de4220da7b5dafa62649a9889a9ab4a9f3722cbb5',files:[['dlopen2-LICENSE.txt','39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2','https://github.com/OpenByteDev/dlopen2/blob/cc80e4a0a90d499b677fdf7743699b4b3a43a989/LICENSE']]},
 'dlopen2_derive@0.4.3':{rev:'cc80e4a0a90d499b677fdf7743699b4b3a43a989',path:'dlopen2-derive',sourceHash:'f172e1b7acca5edabefc867c1f7dd8f6deaf3eeb0f53097740961fe47b770a72',files:[['dlopen2-LICENSE.txt','39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2','https://github.com/OpenByteDev/dlopen2/blob/cc80e4a0a90d499b677fdf7743699b4b3a43a989/LICENSE']]},
 'libappindicator-sys@0.9.0':{rev:'eafd1e3682a1247f595410266091e9684021cb6f',path:'sys',sourceHash:'742a25cd2a4c09762f57c3dfaa8958e6c62c771e28916a58b45a85fae84aea6d',files:[['libappindicator-LICENSE-MIT.txt','eb227437252b2a7a9c1fc342c93ade1f3d7ce38cc6dd754f613db07d53ceff0b','https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-MIT'],['libappindicator-LICENSE-APACHE.txt','a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2','https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-APACHE']]},
});
export function auditedUpstreamNotices(packageRoot,noticeRoot,rule) {
 const vcs=JSON.parse(safeFile(packageRoot,resolve(packageRoot,'.cargo_vcs_info.json')).text);
 if(vcs.git?.sha1!==rule.rev||vcs.git?.dirty===true||vcs.path_in_vcs!==rule.path)throw Error('Audited crate revision mismatch or dirty source');
 if(!(rule.sourceHashes??[rule.sourceHash]).includes(safeFile(packageRoot,resolve(packageRoot,rule.sourcePath??'src/lib.rs')).sha256))throw Error('Audited crate source hash mismatch');
 for(const [path,expected] of rule.additionalSources??[])if(safeFile(packageRoot,resolve(packageRoot,path)).sha256!==expected)throw Error('Audited crate source hash mismatch: '+path);
 return rule.files.map(([path,expected,url])=>{
  const f=safeFile(noticeRoot,resolve(noticeRoot,path));
  if(f.sha256!==expected)throw Error('Audited upstream notice hash mismatch');
  return {...f,source:url};
 });
}
export function buildNoticeBundle(metadata,repoRoot,options={}) {
 if(options===null||typeof options!=='object'||Array.isArray(options)||Object.keys(options).some(k=>k!=='platform')) throw Error('Invalid notice bundle options');
 const platform=Object.hasOwn(options,'platform')?options.platform:'Linux';
 if(!['Linux','Windows','macOS'].includes(platform)) throw Error('Unsupported platform label');
 if(!Array.isArray(metadata.packages)||!Array.isArray(metadata.resolve?.nodes)) throw Error('Resolved Cargo metadata required');
 const nodeIds=metadata.resolve.nodes.map(node=>node?.id),packageIds=metadata.packages.map(pkg=>pkg?.id);
 if(!nodeIds.length||nodeIds.some(id=>typeof id!=='string'||!id)||new Set(nodeIds).size!==nodeIds.length||
    packageIds.some(id=>typeof id!=='string'||!id)||new Set(packageIds).size!==packageIds.length)
  throw Error('Invalid resolved graph or duplicate package IDs');
 const available=new Set(packageIds);
 if(nodeIds.some(id=>!available.has(id)))throw Error('Resolved graph references missing package metadata');
 const wanted=new Set(nodeIds);
 if(metadata.resolve.root!=null&&!wanted.has(metadata.resolve.root))throw Error('Resolved graph root is missing');
 for(const node of metadata.resolve.nodes){
  if(!Array.isArray(node.deps)||!Array.isArray(node.dependencies))throw Error('Invalid resolved graph dependency lists');
  const deps=node.deps.map(dep=>dep?.pkg),legacy=node.dependencies;
  if([...deps,...legacy].some(id=>typeof id!=='string'||!wanted.has(id)))throw Error('Resolved graph has a missing dependency node');
  const a=new Set(deps),b=new Set(legacy);
  if(a.size!==b.size||[...a].some(id=>!b.has(id)))throw Error('Resolved graph dependency lists disagree');
 }
 const root=realpathSync(repoRoot),packages=[];
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
  if(pkg.name==='zune-inflate'&&pkg.version==='0.2.54'&&pkg.source==='registry+https://github.com/rust-lang/crates.io-index')notices.push(...auditedSourceHeaders(dir,[['src/lib.rs','ee2b5b58f48ee36835bd4e312a26dace36be39d1c4af956f4e2e04d0620667f9']]));
  packages.push({name:pkg.name,version:pkg.version,license:pkg.license??null,origin:pkg.source?.startsWith('registry+')?'registry':pkg.source?'external':'local',root_mit_fallback:fallback,notices});
 }
 const missing=packages.filter(p=>!p.notices.length).map(p=>p.name+'@'+p.version);
 let text='Pong '+platform+' dependency notice preparation\n\nResolved Cargo graph; may include build/optional packages. All discovered license choices are retained without choosing one. This is not legal clearance or a system-library inventory. Nested source comments and unlabelled bundled code require separate review.\n';
 for(const p of packages)for(const f of p.notices)text+='\n===== '+p.name+' '+p.version+' | '+f.source+' | declared '+p.license+' =====\n'+f.text+'\n';
 if(Buffer.byteLength(text)>32*1024*1024)throw Error('Notice bundle too large');
 const index={schema:1,scope:'Resolved Cargo metadata, not linked-binary attribution',missing_notice_files:missing,bundle_sha256:digest(Buffer.from(text)),packages:packages.map(p=>({...p,notices:p.notices.map(({text,...f})=>f)}))};
 return {text,index};
}
