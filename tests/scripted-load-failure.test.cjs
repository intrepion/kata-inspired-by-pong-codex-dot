const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

function fixture(fetch) {
  let nextFrame, scheduled = 0;
  class Element {
    constructor() { this.listeners = {}; this.firstChild = {textContent:''}; this.textContent = ''; }
    addEventListener(name, fn) { (this.listeners[name] ??= []).push(fn); }
    setPointerCapture() {}
  }
  class Button extends Element {}
  class Canvas extends Element {
    getContext() { return {fillRect(){}, setLineDash(){}, beginPath(){}, moveTo(){}, lineTo(){}, stroke(){}, arc(){}, fill(){}}; }
  }
  const names = ['court','overlay','message','hint','start','pause','status','you','cpu','up','down'];
  const els = Object.fromEntries(names.map(name => [name, name === 'court' ? new Canvas() : ['start','pause','up','down'].includes(name) ? new Button() : new Element()]));
  const document = new Element(); document.querySelector = selector => els[selector.slice(1)]; document.hidden = false;
  const context = {document, window:new Element(), HTMLElement:Element, HTMLButtonElement:Button, HTMLCanvasElement:Canvas,
    URLSearchParams, location:{search:'?scripted-test'}, fetch, console:{error(){}},
    requestAnimationFrame(fn) { scheduled++; nextFrame = fn; }};
  vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../game.js'),'utf8'), context);
  return {els, get scheduled() { return scheduled; }, frame(t) { const fn = nextFrame; nextFrame = undefined; fn(t); }};
}
test('failed manifest load displays error and stops scheduling frames', async () => {
  const f = fixture(() => Promise.resolve({ok:false}));
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(f.scheduled, 1);
  f.frame(0);
  assert.match(f.els.status.textContent, /Scenario manifest unavailable/);
  assert.equal(f.scheduled, 1);
});
test('pending manifest load continues scheduling until it resolves', () => {
  const f = fixture(() => new Promise(() => {}));
  f.frame(0);
  assert.equal(f.scheduled, 2);
});
test('valid manifest continues scheduling replay frames', async () => {
  const text = 'schema 1\nid valid\nseed 1\nstep_hz 120\nend_tick 100\nreport_every 10\nevent 0 start\n';
  const f = fixture(() => Promise.resolve({ok:true, text:() => Promise.resolve(text)}));
  await new Promise(resolve => setImmediate(resolve));
  f.frame(0); f.frame(10);
  assert.equal(f.scheduled, 3);
  assert.doesNotMatch(f.els.status.textContent, /Error|unavailable/);
});
test('invalid manifest and rejected fetch display errors and stop scheduling', async () => {
  for (const fetch of [() => Promise.resolve({ok:true, text:() => Promise.resolve('not a scenario')}), () => Promise.reject(new Error('offline'))]) {
    const f = fixture(fetch);
    await new Promise(resolve => setImmediate(resolve));
    f.frame(0);
    assert.match(f.els.status.textContent, /Invalid or duplicate scenario line|offline/);
    assert.equal(f.scheduled, 1);
  }
});
test('even an empty rejection stops the failed scripted loop', async () => {
  const f = fixture(() => Promise.reject(''));
  await new Promise(resolve => setImmediate(resolve));
  f.frame(0); assert.equal(f.scheduled, 1);
});
