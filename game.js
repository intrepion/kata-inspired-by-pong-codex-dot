'use strict';
// The simulation uses seconds and court coordinates so its pace is frame-rate independent.
const W=900,H=500,PADDLE_H=90,PADDLE_W=12,BALL=8, WIN=7;
const clamp=(n,min,max)=>Math.max(min,Math.min(max,n));
function createGame(random=Math.random){
 const s={player:H/2,cpu:H/2,ball:{x:W/2,y:H/2,vx:0,vy:0},scores:[0,0],state:'ready',serve:0,random};
 s.resetBall=(direction)=>{s.ball={x:W/2,y:H/2,vx:direction*350,vy:(random()-.5)*180};s.serve=.65};
 s.start=()=>{s.scores=[0,0];s.player=s.cpu=H/2;s.state='playing';s.resetBall(random()<.5?-1:1)};
 s.step=(dt,input=0)=>{if(s.state!=='playing')return;dt=clamp(dt,0,.033);s.player=clamp(s.player+input*470*dt,PADDLE_H/2,H-PADDLE_H/2);const b=s.ball;const target=b.vx>0?b.y:H/2;s.cpu=clamp(s.cpu+clamp(target-s.cpu,-300*dt,300*dt),PADDLE_H/2,H-PADDLE_H/2);if(s.serve>0){s.serve-=dt;return}const oldX=b.x;b.x+=b.vx*dt;b.y+=b.vy*dt;if(b.y<BALL){b.y=2*BALL-b.y;b.vy=Math.abs(b.vy)}if(b.y>H-BALL){b.y=2*(H-BALL)-b.y;b.vy=-Math.abs(b.vy)}
 for(const [x,y,dir] of [[38,s.player,1],[W-38,s.cpu,-1]]){const face=x+dir*(PADDLE_W/2+BALL);if((dir===1?oldX>=face&&b.x<=face:oldX<=face&&b.x>=face)&&Math.abs(b.y-y)<=PADDLE_H/2+BALL){const angle=clamp((b.y-y)/(PADDLE_H/2),-1,1)*1.05;const speed=Math.min(720,Math.hypot(b.vx,b.vy)*1.055);b.x=face;b.vx=dir*speed*Math.cos(angle);b.vy=speed*Math.sin(angle)}}
 if(b.x<-BALL||b.x>W+BALL){const scorer=b.x<0?1:0;s.scores[scorer]++;if(s.scores[scorer]>=WIN){s.state='over'}else s.resetBall(scorer===0?-1:1)} };
 return s;
}
if(typeof module!=='undefined')module.exports={createGame,W,H,PADDLE_H,BALL};
if(typeof document!=='undefined'){
 const canvas=document.querySelector('#court'),ctx=canvas.getContext('2d'),game=createGame(),keys=new Set();let touch=0,last=0;
 const overlay=document.querySelector('#overlay'),message=document.querySelector('#message'),hint=document.querySelector('#hint'),start=document.querySelector('#start'),pause=document.querySelector('#pause'),status=document.querySelector('#status');
 function sync(){document.querySelector('#you').textContent=game.scores[0];document.querySelector('#cpu').textContent=game.scores[1];pause.disabled=game.state==='ready'||game.state==='over';pause.textContent=game.state==='paused'?'Resume':'Pause';status.textContent=game.state==='playing'?(game.serve>0?'Next serve':'Rally in progress'):game.state==='paused'?'Paused':game.state==='over'?(game.scores[0]===WIN?'You won the match':'Computer wins the match'):'Ready to play';overlay.hidden=game.state==='playing';if(game.state==='paused'){message.textContent='Take a breather.';hint.textContent='Your match is waiting for you.';start.firstChild.textContent='Resume match '}if(game.state==='over'){message.textContent=game.scores[0]===WIN?'You take the court.':'A worthy opponent.';hint.textContent=game.scores[0]===WIN?'Nice rally. Ready for another?':'The computer won this round. Run it back?';start.firstChild.textContent='Play again '}}
 function toggle(){if(game.state==='playing')game.state='paused';else if(game.state==='paused')game.state='playing';last=0;sync()}
 start.addEventListener('click',()=>{if(game.state==='paused')game.state='playing';else game.start();last=0;sync()});pause.addEventListener('click',toggle);
 document.addEventListener('keydown',e=>{if(['ArrowUp','ArrowDown',' ','w','s','W','S'].includes(e.key))e.preventDefault();if(e.key===' '&&!e.repeat&&(game.state==='playing'||game.state==='paused'))toggle();keys.add(e.key.toLowerCase())});document.addEventListener('keyup',e=>keys.delete(e.key.toLowerCase()));window.addEventListener('blur',()=>{keys.clear();touch=0;if(game.state==='playing')toggle()});document.addEventListener('visibilitychange',()=>{if(document.hidden&&game.state==='playing')toggle()});
 for(const [id,dir] of [['up',-1],['down',1]]){const el=document.querySelector('#'+id);el.addEventListener('pointerdown',e=>{e.preventDefault();touch=dir;el.setPointerCapture(e.pointerId)});for(const event of ['pointerup','pointercancel','lostpointercapture'])el.addEventListener(event,()=>{touch=0})}
 canvas.addEventListener('pointerdown',e=>{if(e.pointerType!=='mouse')canvas.setPointerCapture(e.pointerId)});canvas.addEventListener('pointermove',e=>{if(e.buttons&&e.pointerType!=='mouse'&&game.state==='playing'){const rect=canvas.getBoundingClientRect();game.player=clamp((e.clientY-rect.top)/rect.height*H,PADDLE_H/2,H-PADDLE_H/2)}});
 function draw(){ctx.fillStyle='#153f37';ctx.fillRect(0,0,W,H);ctx.strokeStyle='#a8e6bb30';ctx.lineWidth=2;ctx.setLineDash([12,16]);ctx.beginPath();ctx.moveTo(W/2,0);ctx.lineTo(W/2,H);ctx.stroke();ctx.setLineDash([]);ctx.strokeStyle='#a8e6bb25';ctx.beginPath();ctx.arc(W/2,H/2,70,0,Math.PI*2);ctx.stroke();ctx.fillStyle='#d9ffae';ctx.fillRect(32,game.player-PADDLE_H/2,PADDLE_W,PADDLE_H);ctx.fillStyle='#e3f3e7';ctx.fillRect(W-44,game.cpu-PADDLE_H/2,PADDLE_W,PADDLE_H);ctx.beginPath();ctx.arc(game.ball.x,game.ball.y,BALL,0,Math.PI*2);ctx.fill()}
 function frame(t){if(last)game.step((t-last)/1000,touch+(keys.has('arrowdown')||keys.has('s')?1:0)-(keys.has('arrowup')||keys.has('w')?1:0));last=t;draw();sync();requestAnimationFrame(frame)}sync();requestAnimationFrame(frame);
}
