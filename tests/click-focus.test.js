const { test } = require('node:test');
const path = require('node:path');
function harness() {
const vm = require('node:vm'), fs = require('node:fs'), assert = require('node:assert/strict');
class Element {
  constructor() { this.listeners = {}; this.firstChild = { textContent: '' }; this.textContent = ''; }
  addEventListener(name, fn) { (this.listeners[name] ??= []).push(fn); }
  emit(name, event = {}) { const e = { defaultPrevented: false, preventDefault() { this.defaultPrevented = true; }, stopPropagation() {}, ...event }; for (const fn of this.listeners[name] ?? []) fn(e); return e; }
  setPointerCapture() {} getBoundingClientRect() { return { top: 0, height: 500 }; }
}
class Button extends Element {}
let paddle;
class Canvas extends Element { getContext() { return { fillRect(x, y) { if(x === 32) paddle = y; }, setLineDash() {}, beginPath() {}, moveTo() {}, lineTo() {}, stroke() {}, arc() {}, fill() {} }; } }
const names = ['court','overlay','message','hint','start','pause','status','you','cpu','up','down'];
const els = Object.fromEntries(names.map(k => [k, k === 'court' ? new Canvas() : ['start','pause','up','down'].includes(k) ? new Button() : new Element()]));
const doc = new Element(); doc.querySelector = s => els[s.slice(1)]; doc.hidden = false;
const win = new Element();
let frame;
vm.runInNewContext(fs.readFileSync(path.join(__dirname, '../game.js'), 'utf8'), { document: doc, window: win, HTMLElement: Element, HTMLButtonElement: Button, HTMLCanvasElement: Canvas, URLSearchParams, location: { search: '' }, requestAnimationFrame(fn) { frame = fn; }, console });
return {els, doc, win, frame: t => frame(t), get paddle() { return paddle; }};
}
test('blurred or hidden clicks cannot resume; focused activation remains available', () => {
const {els, doc, win} = harness();
const assert = require('node:assert/strict');
// Click detail 1 represents pointer-origin activation and detail 0 a non-pointer activation in this mock only.
els.start.emit('click', {detail: 1}); assert.equal(els.pause.textContent, 'Pause');
win.emit('blur'); assert.equal(els.status.textContent, 'Paused');
els.pause.emit('click', {detail: 1}); assert.equal(els.status.textContent, 'Paused', 'blurred pause click');
els.start.emit('click', {detail: 0}); assert.equal(els.status.textContent, 'Paused', 'blurred start click');
win.emit('focus'); els.start.emit('click', {detail: 0}); assert.notEqual(els.status.textContent, 'Paused', 'focused non-pointer resume');
doc.hidden = true; doc.emit('visibilitychange'); assert.equal(els.status.textContent, 'Paused');
els.pause.emit('click', {detail: 0}); assert.equal(els.status.textContent, 'Paused', 'hidden pause click');
els.start.emit('click', {detail: 1}); assert.equal(els.status.textContent, 'Paused', 'hidden start click');
doc.hidden = false; doc.emit('visibilitychange'); els.pause.emit('click', {detail: 1}); assert.notEqual(els.status.textContent, 'Paused', 'focused pointer resume');
const key = doc.emit('keydown', {key: ' ', repeat: false, target: els.pause});
assert.equal(key.defaultPrevented, false, 'focused button keeps native keyboard activation');
els.pause.emit('click', {detail: 0}); assert.equal(els.status.textContent, 'Paused', 'focused non-pointer pause');


});

for (const source of ['keyboard', 'pointer']) {
 test(`quick ${source} tap survives zero-step frames and moves once`, () => {
    const assert = require('node:assert/strict');
    const h = harness(), {els,doc} = h;
    els.start.emit('click');
    h.frame(100);
    const before = h.paddle;
    if (source === 'keyboard') {
      doc.emit('keydown', {key:'w', repeat:false});
      doc.emit('keyup', {key:'w'});
    } else {
      els.up.emit('pointerdown', {pointerId:1});
      els.up.emit('pointerup', {pointerId:1});
      els.up.emit('lostpointercapture', {pointerId:1});
    }
    h.frame(104); assert.equal(h.paddle,before,source);
    h.frame(110); assert.ok(h.paddle<before,source);
    const after = h.paddle;
    h.frame(150); assert.equal(h.paddle,after,source);
 });
}
