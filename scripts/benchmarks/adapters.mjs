// Explicit supported adapters; never guess metadata from an arbitrary binary filename.
export const adapters = Object.freeze({
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
  if(positional.length !== 4 || positional.some(x=>!x || x.startsWith('--'))) throw Error('usage: run.mjs BINARY NEW.json BACKEND NOTES [--adapter macroquad|softbuffer|sdl3]');
  const [binary, output, backend, notes] = positional;
  return {binary, output, backend, notes, adapter, ...adapters[adapter]};
}
