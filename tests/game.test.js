const { test } = require('node:test');
const assert = require('node:assert/strict');
const { createGame, H, PADDLE_H, BALL } = require('../game');
test('start, movement and bounds', () => { const g = createGame(() => .2); g.start(); assert.equal(g.state, 'playing'); for (let i = 0; i < 100; i++)
    g.step(.033, -1); assert.equal(g.player, PADDLE_H / 2); });
test('paddle reflects ball and steers angle', () => { const g = createGame(() => .2); g.start(); g.serve = 0; g.player = H / 2; g.ball = { x: 60, y: H / 2 + 25, vx: -350, vy: 0 }; g.step(.033); assert.ok(g.ball.vx > 0); assert.ok(g.ball.vy > 0); });
test('wall reflects and a miss scores, then match ends at seven', () => { const g = createGame(() => .2); g.start(); g.serve = 0; g.ball = { x: 450, y: 9, vx: 300, vy: -200 }; g.step(.033); assert.ok(g.ball.vy > 0); g.scores = [6, 0]; g.ball = { x: 905, y: 20, vx: 350, vy: 0 }; g.step(.033); assert.deepEqual(g.scores, [7, 0]); assert.equal(g.state, 'over'); g.start(); assert.deepEqual(g.scores, [0, 0]); });
test('paused game does not advance', () => { const g = createGame(() => .2); g.start(); g.state = 'paused'; const x = g.ball.x; g.step(.033, 1); assert.equal(g.ball.x, x); assert.equal(g.player, H / 2); });
test('computer reacts after a delay and moves at a finite speed', () => { const g = createGame(() => .5); g.start(); g.serve = 0; g.ball = { x: 500, y: 400, vx: 350, vy: 0 }; const initial = g.cpu; g.step(.033); assert.equal(g.cpu, initial); for (let i = 0; i < 10; i++)
    g.step(.033); assert.ok(g.cpu > initial); assert.ok(g.cpu - initial <= 190 * .363 + 1e-9); });
test('computer returns an accessible shot and misses a fast distant shot', () => { const easy = createGame(() => .5); easy.start(); easy.serve = 0; easy.ball = { x: 650, y: H / 2, vx: 350, vy: 0 }; for (let i = 0; i < 40 && easy.ball.vx > 0; i++)
    easy.step(.033); assert.ok(easy.ball.vx < 0, 'center shot should be returned'); const hard = createGame(() => .5); hard.start(); hard.serve = 0; hard.ball = { x: 750, y: 50, vx: 700, vy: 0 }; for (let i = 0; i < 20 && !hard.scores[0]; i++)
    hard.step(.033); assert.equal(hard.scores[0], 1, 'distant fast shot should score'); });
test('combined controls cannot exceed normal paddle speed', () => { for (const input of [-2, 2]) {
    const g = createGame(() => .5);
    g.start();
    g.step(.016, input);
    assert.ok(Math.abs(g.player - (250 + Math.sign(input) * 7.52)) < 1e-9);
} });
test('nonfinite time or input leaves the simulation unchanged', () => { for (const [dt, input] of [[NaN, 0], [Infinity, 0], [-Infinity, 0], [.016, NaN], [.016, Infinity], [.016, -Infinity]]) {
    const g = createGame(() => .5);
    g.start();
    const before = JSON.stringify(g);
    g.step(dt, input);
    assert.equal(JSON.stringify(g), before);
} });
test('frame loop preserves one second of gameplay at 20, 60 and 120 Hz', () => { const { createFrameLoop } = require('../game'); for (const hz of [20, 60, 120]) {
    const g = createGame(() => .5);
    g.start();
    g.serve = 0;
    const loop = createFrameLoop(g);
    for (let i = 0; i <= hz; i++)
        loop.advance(i * 1000 / hz);
    assert.ok(Math.abs(g.ball.x - 800) < 1e-8, `${hz} Hz: ${g.ball.x}`);
} });
test('long stalls catch up no more than 100 ms per frame', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(5000); assert.ok(Math.abs(g.ball.x - 485) < 1e-8); loop.advance(5000 + 1000 / 120); assert.ok(Math.abs(g.ball.x - 487.9166666667) < 1e-8); });
test('reset discards partial steps and elapsed pause time', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(4); loop.reset(); loop.advance(10000); loop.advance(10004); assert.equal(g.ball.x, 450); loop.advance(10000 + 1000 / 120); assert.ok(Math.abs(g.ball.x - 452.9166666667) < 1e-8); });
test('invalid frame timestamps and controls do not poison the clock', () => { const { createFrameLoop } = require('../game'); const g = createGame(() => .5); g.start(); g.serve = 0; const loop = createFrameLoop(g); loop.advance(0); loop.advance(NaN); loop.advance(Infinity); loop.advance(-10); loop.advance(20, NaN); loop.advance(1000 / 120); assert.ok(Math.abs(g.ball.x - 452.9166666667) < 1e-8); });

test('portable scenario preserves TS action semantics and is bounded/deterministic', () => {
    const {parseScenario,createScriptedReplay}=require('../game');
    const text=require('node:fs').readFileSync(require('node:path').join(__dirname,'../scenarios/smoke-v1.txt'),'utf8');
    const manifest=parseScenario(text), a=createScriptedReplay(manifest),b=createScriptedReplay(manifest);
    const checkpoints=[];
    while(!a.finished){a.step();b.step();if(manifest.captures.includes(a.tick))checkpoints.push(a.tick);}
    assert.deepEqual(a.reports,b.reports);assert.equal(a.tick,manifest.end);assert.equal(a.reports.length,13);
    assert.deepEqual(a.actions,manifest.events);assert.deepEqual(checkpoints,manifest.captures);
    assert.equal(a.reports.find(r=>r.tick===120).player,45);assert.equal(a.reports.find(r=>r.tick===240).player,455);
    const before=a.reports.find(r=>r.tick===360),paused=a.reports.find(r=>r.tick===480);
    assert.equal(paused.state,'paused');assert.deepEqual(paused.ball,before.ball);assert.equal(paused.player,before.player);
    assert.equal(a.reports.find(r=>r.tick===600).state,'playing');assert.deepEqual(a.reports.find(r=>r.tick===1320).scores,[0,0]);
    const final=JSON.stringify(a.reports);a.step();assert.equal(JSON.stringify(a.reports),final);
});
test('portable manifest rejects unknown actions, duplicates and overlapping ranges',()=>{
 const {parseScenario}=require('../game'),fs=require('node:fs'),path=require('node:path');
 const text=fs.readFileSync(path.join(__dirname,'../scenarios/smoke-v1.txt'),'utf8');
 for(const bad of [text+'\nseed 9',text.replace('event 0 start','event 0 shell'),text.replace('input 120 240 1','input 100 240 1'),text.replace('step_hz 120','step_hz 0')])assert.throws(()=>parseScenario(bad));
});
test('scripted frame pacing lands on every capture tick even after stalls',()=>{
 const {parseScenario,createScriptedReplay}=require('../game'),fs=require('node:fs'),path=require('node:path');
 const m=parseScenario(fs.readFileSync(path.join(__dirname,'../scenarios/smoke-v1.txt'),'utf8')),r=createScriptedReplay(m),seen=new Set();
 for(let t=0;!r.finished && t<100000;t+=137){r.frame(t);if(m.captures.includes(r.tick))seen.add(r.tick);}
 assert.equal(r.finished,true);assert.deepEqual([...seen],m.captures);
});

test('pointer button release preserves other held fingers', () => {
 const {createPointerButtons}=require('../game'); const input=createPointerButtons();
 input.press(1,-1); input.press(2,-1); input.release(1); assert.equal(input.axis(),-1);
 input.release(99); assert.equal(input.axis(),-1); input.release(2); assert.equal(input.axis(),0);
});
test('opposing pointer buttons cancel and release independently', () => {
 const {createPointerButtons}=require('../game'); const input=createPointerButtons();
 input.press(1,-1); input.press(2,1); assert.equal(input.axis(),0);
 input.release(2); assert.equal(input.axis(),-1); input.clear(); assert.equal(input.axis(),0);
 input.release(1); assert.equal(input.axis(),0);
});

test('pause clears keyboard, button and drag ownership until fresh input', () => {
 const {createControls}=require('../game'); const input=createControls();
 input.keyDown('w',false); input.buttonDown(11,1); input.dragDown(21);
 assert.equal(input.axis(),0); assert.equal(input.dragAllowed(21),true);
 input.suspend(); input.keyDown('w',true);
 assert.equal(input.axis(),0); assert.equal(input.dragAllowed(21),false);
 input.keyUp('w'); input.buttonUp(11); input.dragUp(21);
 input.keyDown('w',false); input.buttonDown(12,1); input.dragDown(22);
 assert.equal(input.axis(),0); assert.equal(input.dragAllowed(22),true);
 input.buttonUp(12); assert.equal(input.axis(),-1);
});

test('browser handlers clear movement on pause and reject continuing touch drags', () => {
 const vm=require('node:vm'),fs=require('node:fs'); let nextFrame, playerY;
 class Element {
  constructor(){this.listeners={};this.firstChild={textContent:''};this.textWrites=0;this._text='';}
  get textContent(){return this._text;} set textContent(value){this.textWrites++;this._text=value;}
  addEventListener(name,fn){(this.listeners[name]??=[]).push(fn);}
  emit(name,event={}){const e={defaultPrevented:false,propagationStopped:false,preventDefault(){this.defaultPrevented=true;},stopPropagation(){this.propagationStopped=true;},...event};for(const fn of this.listeners[name]??[])fn(e);return e;}
  setPointerCapture(){} getBoundingClientRect(){return {top:0,height:500};}
 }
 class Button extends Element {}
 class Canvas extends Element {getContext(){return {fillRect(x,y){if(x===32)playerY=y;},setLineDash(){},beginPath(){},moveTo(){},lineTo(){},stroke(){},arc(){},fill(){}};}}
 const selectors=['court','overlay','message','hint','start','pause','status','you','cpu','up','down'];
 const els=Object.fromEntries(selectors.map(k=>[k,k==='court'?new Canvas():['start','pause','up','down'].includes(k)?new Button():new Element()]));
 const doc=new Element();doc.querySelector=s=>els[s.slice(1)];doc.hidden=false;
 const win=new Element();
 vm.runInNewContext(fs.readFileSync(require('node:path').join(__dirname,'../game.js'),'utf8'),{document:doc,window:win,HTMLElement:Element,HTMLButtonElement:Button,HTMLCanvasElement:Canvas,URLSearchParams,location:{search:''},requestAnimationFrame:fn=>{nextFrame=fn;},console});
 const frame=t=>{const fn=nextFrame;fn(t);};
 const spaceOnStart=doc.emit('keydown',{key:' ',repeat:false,target:els.start});assert.equal(spaceOnStart.defaultPrevented,false);
 doc.emit('keydown',{key:'w',repeat:false});
 els.up.emit('pointerdown',{pointerId:101});
 els.court.emit('pointerdown',{pointerType:'touch',pointerId:102});
 els.start.emit('click'); frame(0); const readyStart=playerY;
 doc.emit('keydown',{key:'w',repeat:true});
 els.court.emit('pointermove',{pointerType:'touch',pointerId:102,buttons:1,clientY:450});
 frame(20); assert.equal(playerY,readyStart,'Ready inputs and continuing drag must not arm');
 doc.emit('keyup',{key:'w'}); els.up.emit('pointerup',{pointerId:101}); els.court.emit('pointerup',{pointerId:102});
 els.start.emit('click');frame(0);const statusWrites=els.status.textWrites;frame(0);assert.equal(els.status.textWrites,statusWrites,'unchanged live status must not be rewritten');doc.emit('keydown',{key:'w',repeat:false});frame(20);const moved=playerY;
 els.up.emit('pointerdown',{pointerId:3});const spaceOnPause=doc.emit('keydown',{key:' ',repeat:false,target:els.pause});assert.equal(spaceOnPause.defaultPrevented,false);assert.equal(els.pause.textContent,'Pause');els.pause.emit('click');doc.emit('keydown',{key:'s',repeat:false});els.down.emit('pointerdown',{pointerId:103});els.court.emit('pointerdown',{pointerType:'touch',pointerId:104});els.start.emit('click');doc.emit('keydown',{key:'w',repeat:true});doc.emit('keydown',{key:'s',repeat:true});els.court.emit('pointermove',{pointerType:'touch',pointerId:104,buttons:1,clientY:450});frame(40);frame(60);assert.equal(playerY,moved);doc.emit('keyup',{key:'s'});els.down.emit('pointerup',{pointerId:103});els.court.emit('pointerup',{pointerId:104});
 els.court.emit('pointerdown',{pointerType:'touch',pointerId:8});els.court.emit('pointerdown',{pointerType:'touch',pointerId:7});const beforeSecond=playerY;
 els.court.emit('pointermove',{pointerType:'touch',pointerId:7,buttons:1,clientY:100});frame(70);assert.equal(playerY,beforeSecond);els.court.emit('pointerup',{pointerId:7});
 els.court.emit('pointermove',{pointerType:'touch',pointerId:8,buttons:1,clientY:350});frame(80);const dragged=playerY;
 els.pause.emit('click');els.start.emit('click');els.court.emit('pointermove',{pointerType:'touch',pointerId:8,buttons:1,clientY:100});frame(100);assert.equal(playerY,dragged);
 els.court.emit('pointerup',{pointerId:8});els.court.emit('pointerdown',{pointerType:'touch',pointerId:9});els.court.emit('pointermove',{pointerType:'touch',pointerId:9,buttons:1,clientY:100});frame(120);assert.notEqual(playerY,dragged);
 const beforePauseWrites=els.status.textWrites;const shortcut=doc.emit('keydown',{key:' ',repeat:false,target:els.court});assert.equal(shortcut.defaultPrevented,true);assert.equal(els.status.textContent,'Paused');assert.equal(els.status.textWrites,beforePauseWrites+1);
 frame(130);doc.emit('keydown',{key:' ',repeat:true,target:els.court});assert.equal(els.status.textContent,'Paused');assert.equal(els.status.textWrites,beforePauseWrites+1);
 els.start.emit('click');frame(140);const yBeforeButton=playerY;
 const buttonKey=els.up.emit('keydown',{key:' ',repeat:false,target:els.up});
 assert.equal(buttonKey.defaultPrevented,true);assert.equal(buttonKey.propagationStopped,true);
 frame(160);frame(180);assert.ok(playerY < yBeforeButton);const yReleased=playerY;
 els.up.emit('keyup',{key:' ',target:els.up});frame(200);assert.equal(playerY,yReleased);
 els.down.emit('keydown',{key:'Enter',repeat:false,target:els.down});frame(220);assert.ok(playerY>yReleased);
 els.up.emit('keydown',{key:' ',repeat:false,target:els.up});frame(240);const yCancelled=playerY;
 els.down.emit('keyup',{key:'Enter',target:els.down});frame(260);assert.ok(playerY<yCancelled);
 els.pause.emit('click');els.start.emit('click');els.up.emit('keydown',{key:' ',repeat:true,target:els.up});frame(280);const yAfterPause=playerY;frame(300);assert.equal(playerY,yAfterPause);
 els.up.emit('keyup',{key:' ',target:els.up});els.up.emit('keydown',{key:' ',repeat:false,target:els.up});frame(320);assert.ok(playerY<yAfterPause);
 els.up.emit('blur');const yBlur=playerY;frame(340);assert.equal(playerY,yBlur);
 els.down.emit('click',{detail:0});frame(360);assert.equal(playerY,yBlur+32);
 const afterClick=playerY;els.up.emit('click',{detail:1});frame(380);assert.equal(playerY,afterClick);
 for(let i=0;i<100;i++)els.down.emit('click',{detail:0});frame(390);assert.equal(playerY,410);
 els.pause.emit('click');els.up.emit('click',{detail:0});frame(400);assert.equal(playerY,410);
});

test('independent original collision uses crossing height on both sides',()=>{
 for(const [x,vx] of [[60,-350],[840,350]])for(const [y,vy,bounce] of [[180,600,false],[215,-600,true]]){
  const g=createGame(()=>.5);g.start();g.serve=0;g.cpuReaction=1;g.ball={x,y,vx,vy};g.step(.033,0);
  assert.equal(Math.sign(g.ball.vx)!==Math.sign(vx),bounce,`x=${x}, y=${y}, vy=${vy}`);
 }
});
test('independent original collision uses moving paddle at contact and keeps remaining travel',()=>{
 for(const side of ['player','cpu']){
  const g=createGame(()=>.5);g.start();g.serve=0;g.cpuReaction=1;g.cpuTarget=455;
  g.ball={x:side==='player'?54:846,y:side==='player'?306.3:304.3,vx:side==='player'?-350:350,vy:0};
  g.step(1/120,side==='player'?1:0);assert.equal(Math.sign(g.ball.vx),side==='player'?-1:1,`${side} was not at the face in time`);
 }
 for(const [x,vx,face] of [[54,-350,52],[846,350,848]]){
  const g=createGame(()=>.5);g.start();g.serve=0;g.cpuReaction=1;g.ball={x,y:250,vx,vy:0};g.step(1/120,0);
  assert.ok(vx<0?g.ball.x>face:g.ball.x<face,'ball travels after contact');
 }
});
test('independent original collision handles wall reflection before and after contact',()=>{
 for(const [x,vx] of [[60,-350],[840,350]])for(const [y,vy,paddle] of [[10,-600,45],[490,600,455]]){
  const g=createGame(()=>.5);g.start();g.serve=0;g.player=paddle;g.cpu=paddle;g.cpuTarget=paddle;g.cpuReaction=1;g.ball={x,y,vx,vy};g.step(.033,0);
  assert.equal(Math.sign(g.ball.vx),-Math.sign(vx));assert.ok(g.ball.y>=BALL && g.ball.y<=H-BALL);
 }
 for(const [x,vx] of [[54,-350],[846,350]])for(const [y,paddle,sign] of [[8.3,45,1],[491.7,455,-1]]){
  const g=createGame(()=>.5);g.start();g.serve=0;g.player=paddle;g.cpu=paddle;g.cpuTarget=paddle;g.cpuReaction=1;g.ball={x,y,vx,vy:0};g.step(1/120,0);
  assert.equal(Math.sign(g.ball.vx),-Math.sign(vx));assert.equal(Math.sign(g.ball.vy),sign);assert.ok(g.ball.y>=BALL && g.ball.y<=H-BALL);
 }
});
test('independent original collision handles tangent contact and a stationary ball at the face',()=>{
 for(const [x,vx,face] of [[54,-350,52],[846,350,848]]){
  const g=createGame(()=>.5);g.start();g.serve=0;g.cpuReaction=1;g.ball={x,y:303,vx,vy:0};g.step(1/120,0);assert.equal(Math.sign(g.ball.vx),-Math.sign(vx));
  g.ball={x:face,y:250,vx:0,vy:0};g.step(1/120,0);assert.equal(g.ball.vx,0);assert.equal(g.ball.x,face);
 }
});

test('canvas drag remains owned by first pointer until release or cancellation',()=>{
 const {createControls}=require('../game'),input=createControls();
 input.dragDown(10);input.dragDown(20);assert.equal(input.dragAllowed(10),true);assert.equal(input.dragAllowed(20),false);
 input.dragUp(20);assert.equal(input.dragAllowed(10),true);
 input.dragDown(20);input.dragUp(10);assert.equal(input.dragAllowed(20),false);
 input.dragDown(30);assert.equal(input.dragAllowed(30),true);input.suspend();assert.equal(input.dragAllowed(30),false);
 input.dragDown(40);assert.equal(input.dragAllowed(40),true);input.dragUp(30);assert.equal(input.dragAllowed(40),true);
});
