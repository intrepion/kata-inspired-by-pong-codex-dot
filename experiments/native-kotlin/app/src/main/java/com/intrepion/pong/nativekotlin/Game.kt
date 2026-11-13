package com.intrepion.pong.nativekotlin
import kotlin.math.*
enum class GameState { READY, PLAYING, PAUSED, OVER }
data class Ball(var x: Float=450f,var y: Float=250f,var vx: Float=0f,var vy: Float=0f)
class Game(seed: ULong=42uL) {
 var player=250f; var cpu=250f; var ball=Ball(); var scores=intArrayOf(0,0); var state=GameState.READY; var serve=0f
 private var target=250f; private var reaction=0f; private var rng=seed.coerceAtLeast(1uL)
 private fun random(): Float { rng=rng xor (rng shl 13); rng=rng xor (rng shr 7); rng=rng xor (rng shl 17); return (rng shr 40).toFloat()/16777216f }
 private fun reset(direction: Float) { ball=Ball(450f,250f,direction*350f,(random()-.5f)*180f); serve=.65f; target=250f; reaction=.3f }
 fun start() { scores=intArrayOf(0,0); player=250f; cpu=250f; state=GameState.PLAYING; reset(if(random()<.5f) -1f else 1f) }
 fun pause() { if(state==GameState.PLAYING) state=GameState.PAUSED }
 fun togglePause() { state=when(state) { GameState.PLAYING->GameState.PAUSED; GameState.PAUSED->GameState.PLAYING; else->state } }
 fun step(elapsed: Float,input: Float) {
  if(state!=GameState.PLAYING || !elapsed.isFinite() || !input.isFinite()) return
  val dt=elapsed.coerceIn(0f,.033f)
  val oldPlayer=player; val oldCpu=cpu; val playerSpeed=input.coerceIn(-1f,1f)*470f
  player=(player+playerSpeed*dt).coerceIn(45f,455f)
  reaction-=dt
  if(reaction<=0f) { target=if(ball.vx>0f) (ball.y+(random()-.5f)*50f).coerceIn(45f,455f) else 250f; reaction=.3f }
  val speed=if(ball.vx>0f) 190f else 130f
  cpu=(cpu+(target-cpu).coerceIn(-speed*dt,speed*dt)).coerceIn(45f,455f)
  if(serve>0f) { serve-=dt; return }
  val oldX=ball.x; val oldY=ball.y; val oldVy=ball.vy
  ball.x+=ball.vx*dt; ball.y+=ball.vy*dt
  if(ball.y<8f) { ball.y=16f-ball.y; ball.vy=abs(ball.vy) }
  if(ball.y>492f) { ball.y=984f-ball.y; ball.vy=-abs(ball.vy) }
  for((x,direction) in arrayOf(38f to 1f,862f to -1f)) {
   val face=x+direction*14f
   val crosses=if(direction==1f) oldX>=face && ball.x<=face else oldX<=face && ball.x>=face
   if(!crosses || oldX==ball.x) continue
   val fraction=(face-oldX)/(ball.x-oldX)
   var hitY=oldY+oldVy*dt*fraction
   if(hitY<8f) hitY=16f-hitY else if(hitY>492f) hitY=984f-hitY
   val contactTime=dt*fraction
   val paddleY=if(direction==1f) (oldPlayer+playerSpeed*contactTime).coerceIn(45f,455f)
    else (oldCpu+(target-oldCpu).coerceIn(-speed*contactTime,speed*contactTime)).coerceIn(45f,455f)
   if(abs(hitY-paddleY)<=53f) {
    ball.y=hitY
    val angle=((hitY-paddleY)/45f).coerceIn(-1f,1f)*1.05f
    val hitSpeed=(hypot(ball.vx,ball.vy)*1.055f).coerceAtMost(720f)
    ball.x=face; ball.vx=direction*hitSpeed*cos(angle); ball.vy=hitSpeed*sin(angle)
    val remaining=dt-contactTime
    ball.x+=ball.vx*remaining; ball.y+=ball.vy*remaining
    if(ball.y<8f) { ball.y=16f-ball.y; ball.vy=abs(ball.vy) }
    else if(ball.y>492f) { ball.y=984f-ball.y; ball.vy=-abs(ball.vy) }
    break
   }
  }
  if(ball.x< -8f || ball.x>908f) { val scorer=if(ball.x<0f) 1 else 0; scores[scorer]++; if(scores[scorer]>=7) state=GameState.OVER else reset(if(scorer==0) -1f else 1f) }
 }
}
