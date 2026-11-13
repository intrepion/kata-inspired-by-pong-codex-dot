package com.intrepion.pong.nativekotlin
import org.junit.Test
import org.junit.Assert.*
class GameTest {
 private fun game()=Game().apply { start(); serve=0f }
 @Test fun bounds() { val g=game(); repeat(100) { g.step(.033f,-1f) }; assertEquals(45f,g.player,0f) }
 @Test fun paddle() { val g=game(); g.ball=Ball(60f,275f,-350f,0f); g.step(.033f,0f); assertTrue(g.ball.vx>0 && g.ball.vy>0) }
 @Test fun wall() { val g=game(); g.ball=Ball(450f,9f,300f,-200f); g.step(.033f,0f); assertTrue(g.ball.vy>0) }
 @Test fun winRestart() { val g=game(); g.scores=intArrayOf(6,0); g.ball=Ball(905f,20f,350f,0f); g.step(.033f,0f); assertArrayEquals(intArrayOf(7,0),g.scores); assertEquals(GameState.OVER,g.state); g.start(); assertArrayEquals(intArrayOf(0,0),g.scores) }
 @Test fun pause() { val g=game(); val x=g.ball.x; g.pause(); g.step(.033f,1f); assertEquals(x,g.ball.x,0f); assertEquals(250f,g.player,0f); g.togglePause(); assertEquals(GameState.PLAYING,g.state) }
 @Test fun cpuReaction() { val g=game(); g.ball=Ball(500f,400f,350f,0f); g.step(.033f,0f); assertEquals(250f,g.cpu,0f); repeat(10) { g.step(.033f,0f) }; assertTrue(g.cpu>250f) }
 @Test fun cpuCanLose() { val g=game(); g.ball=Ball(750f,50f,700f,0f); repeat(20) { g.step(.033f,0f) }; assertEquals(1,g.scores[0]) }
 @Test fun deterministic() { val a=game(); val b=game(); repeat(200) { a.step(.016f,0f); b.step(.016f,0f) }; assertEquals(a.ball.x,b.ball.x,0f); assertEquals(a.cpu,b.cpu,0f) }
 @Test fun invalidTime() { val g=game(); val x=g.ball.x; g.step(Float.NaN,0f); g.step(-1f,0f); assertEquals(x,g.ball.x,0f) }
 @Test fun crossingHeightDecidesBothPaddles() {
  for ((x,vx) in listOf(60f to -350f, 840f to 350f)) {
   for ((y,vy,hit) in listOf(Triple(180f,600f,false), Triple(215f,-600f,true))) {
    val g=game(); g.ball=Ball(x,y,vx,vy); g.step(.033f,0f)
    assertEquals("x=$x y=$y",hit,g.ball.vx * vx < 0f)
   }
  }
 }
 @Test fun crossingAccountsForPriorWallAndRemainingMotion() {
  for ((x,vx) in listOf(60f to -350f,840f to 350f)) {
   val g=game(); g.player=45f; g.cpu=45f; g.ball=Ball(x,10f,vx,-600f); g.step(.033f,0f)
   assertTrue(g.ball.vx*vx<0f); assertTrue(g.ball.y>=8f && g.ball.y<30f)
  }
  for ((x,vx) in listOf(54f to -350f,846f to 350f)) {
   val g=game(); g.ball=Ball(x,250f,vx,0f); g.step(1f/120f,0f)
   assertTrue(if(vx<0f) g.ball.x>52.8f else g.ball.x<847.2f)
  }
 }
 @Test fun movingPlayerIsSampledAtContact() {
  val g=game(); g.ball=Ball(54f,306.3f,-350f,0f); g.step(1f/120f,1f)
  assertTrue(g.ball.vx<0f)
 }
 @Test fun wallReflectsAfterEitherPaddleContact() {
  for ((x,vx) in listOf(54f to -350f,846f to 350f)) {
   val g=game(); g.player=45f; g.cpu=45f; g.ball=Ball(x,8.3f,vx,0f); g.step(1f/120f,0f)
   assertTrue(g.ball.vx*vx<0f); assertTrue(g.ball.y>=8f && g.ball.vy>0f)
  }
 }
 @Test fun movingCpuIsSampledAtContact() {
  val g=game(); g.ball=Ball(450f,400f,350f,0f)
  repeat(10) { g.step(.033f,0f) }
  g.cpu=250f; g.ball=Ball(846f,304.3f,350f,0f); g.step(1f/120f,0f)
  assertTrue(g.ball.vx>0f)
 }
 @Test fun paddleClampAppliesAtContact() {
  val g=game(); g.player=454.5f; g.ball=Ball(54f,490f,-350f,0f); g.step(1f/120f,1f)
  assertTrue(g.ball.vx>0f)
 }
}
