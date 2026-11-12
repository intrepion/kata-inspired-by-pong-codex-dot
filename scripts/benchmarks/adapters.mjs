// Explicit supported adapters; never guess metadata from an arbitrary binary filename.
export const adapters = Object.freeze({
  fyrox: {presentation:"native window", renderer:"Fyrox native UI renderer (runtime driver unqueried)", pacing:"existing 120Hz update request; sampled in before_rendering callback, before GPU submission"},
  'blue-engine': {presentation:'native window only; headless excluded', renderer:'Blue Engine wgpu (runtime driver unqueried)', pacing:'existing AutoVsync; update callback before renderer submission, object upload precedes callback'},
  bevy: {presentation:'native window', renderer:'Bevy renderer (runtime driver unqueried)', pacing:'existing defaults; main-world Last callback before render-subapp, not GPU timing'},
  'bracket-lib': {presentation:'graphical tile window, not terminal', renderer:'bracket-lib OpenGL CP437 tiles (runtime driver unqueried)', pacing:'existing 60fps cap; sampled after tick tile recording before framework rendering'},
  wgpu: {presentation:'native window', renderer:'raw wgpu surface (runtime driver unqueried)', pacing:'existing winit Poll/default surface config; sampled after successful surface present request only'},
  nannou: {presentation:'native window', renderer:'nannou Bevy/wgpu (runtime driver unqueried)', pacing:'existing 120Hz update limit; sampled at end of view command recording before rendering'},
  ggez: {presentation:'native window', renderer:'ggez wgpu canvas (runtime driver unqueried)', pacing:'existing 120Hz remainder sleep; sampled after Canvas.finish, before framework end_frame'},
  piston: {presentation:'native window', renderer:'Piston wgpu draw submission (driver unqueried)', pacing:'existing 120UPS/default render pacing; sampled after draw_2d on render events only'},
  miniquad: {presentation:'native window', renderer:'Miniquad GPU (backend enum reported; driver unqueried)', pacing:'existing Miniquad defaults; sampled after commit_frame in draw callback'},
  pixels: {presentation:'native window', renderer:'pixels CPU raster plus wgpu presentation (runtime driver unqueried)', pacing:'existing winit Poll; sampled after successful pixels.render returns'},
  raylib: {presentation:'native window', renderer:'raylib-rs GPU (driver unqueried)', pacing:'existing target120fps; sampled after EndDrawing including its pacing wait'},
  macroquad: {presentation:'native window', renderer:'Macroquad GPU', pacing:'framework next_frame boundary; requested vsync'},
  sdl3: {presentation:'native window', renderer:'SDL3 renderer (runtime driver/renderer reported by app)', pacing:'existing 8ms sleep; sampled after canvas.present returns; vsync not explicitly configured'},
  softbuffer: {presentation:'native window', renderer:'CPU raster + softbuffer', pacing:'winit 8ms WaitUntil; sampled after buffer present submission'},
});
export function parseArguments(args) {
  const positional = [...args];
  const index = positional.indexOf('--adapter');
  let adapter = 'macroquad'; // Preserve existing pilot command lines.
  if(index >= 0) {
    adapter = positional[index + 1];
    positional.splice(index, 2);
  }
  if(!Object.hasOwn(adapters, adapter)) throw Error('Unsupported benchmark adapter');
  if(positional.length !== 4 || positional.some(x=>!x || x.startsWith('--'))) throw Error('usage: run.mjs BINARY NEW.json BACKEND NOTES [--adapter macroquad|softbuffer|sdl3|raylib|pixels|miniquad|piston|ggez|nannou|wgpu|bracket-lib|bevy|blue-engine|fyrox]');
  const [binary, output, backend, notes] = positional;
  return {binary, output, backend, notes, adapter, ...adapters[adapter]};
}
