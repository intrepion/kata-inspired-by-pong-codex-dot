const { test } = require('node:test');
const path = require('node:path');
test('blurred or hidden clicks cannot resume; focused activation remains available', () => {
const vm = require('node:vm'), fs = require('node:fs'), assert = require('node:assert/strict');
class Element {
  constructor() { this.listeners = {}; this.firstChild = { textContent: '' }; this.textContent = ''; }
  addEventListener(name, fn) { (this.listeners[name] ??= []).push(fn); }
  emit(name, event = {}) { const e = { defaultPrevented: false, preventDefault() { this.defaultPrevented = true; }, stopPropagation() {}, ...event }; for (const fn of this.listeners[name] ?? []) fn(e); return e; }
  setPointerCapture() {} getBoundingClientRect() { return { top: 0, height: 500 }; }
}
class Button extends Element {}
class Canvas extends Element { getContext() { return { fillRect() {}, setLineDash() {}, beginPath() {}, moveTo() {}, lineTo() {}, stroke() {}, arc() {}, fill() {} }; } }
const names = ['court','overlay','message','hint','start','pause','status','you','cpu','up','down'];
const els = Object.fromEntries(names.map(k => [k, k === 'court' ? new Canvas() : ['start','pause','up','down'].includes(k) ? new Button() : new Element()]));
const doc = new Element(); doc.querySelector = s => els[s.slice(1)]; doc.hidden = false;
const win = new Element();
vm.runInNewContext(fs.readFileSync(path.join(__dirname, '../game.js'), 'utf8'), { document: doc, window: win, HTMLElement: Element, HTMLButtonElement: Button, HTMLCanvasElement: Canvas, URLSearchParams, location: { search: '' }, requestAnimationFrame() {}, console });
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
