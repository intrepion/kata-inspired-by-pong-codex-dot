// Non-GUI guard checks. Optional local arm64 binary exercises positive staging.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path'),cp=require('node:child_process');
for(const [name,rel,key] of [['macroquad','clients/macroquad/packaging/package-macos.cjs','PONG_MACROQUAD_TEST_BINARY'],['sdl3','experiments/sdl3/packaging/package-macos.cjs','PONG_SDL3_TEST_BINARY'],['raylib','experiments/raylib-rs/packaging/package-macos.cjs','PONG_RAYLIB_TEST_BINARY'],['miniquad','experiments/miniquad/packaging/package-macos.cjs','PONG_MINIQUAD_TEST_BINARY'],['pixels','experiments/pixels/packaging/package-macos.cjs','PONG_PIXELS_TEST_BINARY'],['wgpu','experiments/wgpu/packaging/package-macos.cjs','PONG_WGPU_TEST_BINARY'],['piston','experiments/piston/packaging/package-macos.cjs','PONG_PISTON_TEST_BINARY'],['ggez','experiments/ggez/packaging/package-macos.cjs','PONG_GGEZ_TEST_BINARY'],["nannou","experiments/nannou/packaging/package-macos.cjs","PONG_NANNOU_TEST_BINARY"],["bracket-lib","experiments/bracket-lib/packaging/package-macos.cjs","PONG_BRACKET_LIB_TEST_BINARY"],["bevy","clients/bevy/packaging/package-macos.cjs","PONG_BEVY_TEST_BINARY"],["softbuffer","experiments/softbuffer/packaging/package-macos.cjs","PONG_SOFTBUFFER_TEST_BINARY"],["blue-engine","experiments/blue-engine/packaging/package-macos.cjs","PONG_BLUE_ENGINE_TEST_BINARY"]]) {
  const script=path.resolve(__dirname,'..',rel);
  const run=args=>cp.spawnSync(process.execPath,[script,...args],{encoding:'utf8'});
  test(`${name}: missing arguments rejected`,()=>assert.notEqual(run([]).status,0));
  test(`${name}: non-executable input rejected before output creation`,()=>{
    const t=fs.mkdtempSync(path.join(os.tmpdir(),'pong-package-guard-'));
    try {const input=path.join(t,'text'),out=path.join(t,'out');fs.writeFileSync(input,'not Mach-O');assert.notEqual(run([input,out]).status,0);assert(!fs.existsSync(out));} finally {fs.rmSync(t,{recursive:true});}
  });
  test(`${name}: existing output is never overwritten`,{skip:process.platform!=='darwin'||!process.env[key]},()=>{
    const t=fs.mkdtempSync(path.join(os.tmpdir(),'pong-package-guard-'));
    try {fs.writeFileSync(path.join(t,'sentinel'),'keep');assert.notEqual(run([process.env[key],t]).status,0);assert.equal(fs.readFileSync(path.join(t,'sentinel'),'utf8'),'keep');assert.deepEqual(fs.readdirSync(t),['sentinel']);}finally{fs.rmSync(t,{recursive:true});}
  });
}
