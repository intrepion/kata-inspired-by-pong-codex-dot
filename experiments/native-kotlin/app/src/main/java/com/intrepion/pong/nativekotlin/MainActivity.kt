package com.intrepion.pong.nativekotlin
import android.app.Activity
import android.os.Bundle
import android.graphics.*
import android.view.*
import android.widget.*
class MainActivity: Activity() {
 private lateinit var field: PongView
 override fun onCreate(savedInstanceState: Bundle?) {
  super.onCreate(savedInstanceState)
  val layout=LinearLayout(this).apply { orientation=LinearLayout.VERTICAL; setPadding(16,16,16,16); setBackgroundColor(Color.BLACK) }
  val title=TextView(this).apply { text="PONG / Native Kotlin"; textSize=24f; setTextColor(Color.WHITE); gravity=Gravity.CENTER }
  val scripted=intent.getBooleanExtra("pong_scripted",false) && (applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE)!=0
  field=PongView(this,scripted)
  val buttons=LinearLayout(this).apply { gravity=Gravity.CENTER }
  val start=Button(this).apply { text="Start / Restart"; setOnClickListener { field.touch.start(field.game) } }
  val pause=Button(this).apply { text="Pause / Resume"; setOnClickListener { field.touch.togglePause(field.game) } }
  start.isEnabled=!scripted; pause.isEnabled=!scripted
  buttons.addView(start); buttons.addView(pause)
  layout.addView(title); layout.addView(field,LinearLayout.LayoutParams(-1,0,1f)); layout.addView(buttons)
  layout.setOnApplyWindowInsetsListener { view, insets -> view.setPadding(16,16+insets.systemWindowInsetTop,16,16+insets.systemWindowInsetBottom); insets }
  setContentView(layout)
 }
 override fun onPause() { super.onPause(); if(!field.scripted) field.game.pause(); field.touch.clear(); field.active=false }
 override fun onResume() { super.onResume(); field.active=true; field.last=0; field.postInvalidateOnAnimation() }
}
class PongView(context: android.content.Context, val scripted: Boolean=false): View(context) {
 private val replay=if(scripted) AndroidReplay(context) else null
 val game=replay?.scenario?.game ?: Game(); val touch=TouchInput(); var active=true; var last=0L
 private val paint=Paint(Paint.ANTI_ALIAS_FLAG)
 private var scale=1f; private var ox=0f; private var oy=0f
 override fun onDraw(canvas: Canvas) {
  super.onDraw(canvas)
  if(replay!=null) {
   if(active) replay.advance()
  } else {
  val now=System.nanoTime(); if(last!=0L) game.step((now-last)/1_000_000_000f,touch.targetY?.let { ((it-game.player)/20f).coerceIn(-1f,1f) } ?: 0f); last=now
  }
  render(canvas)
  replay?.capture(width,height,::render)
  if(active && replay?.scenario?.finished!=true) postInvalidateOnAnimation()
 }
 private fun render(canvas: Canvas) {
  canvas.drawColor(Color.BLACK)
  scale=minOf(width/900f,(height-140f)/500f).coerceAtLeast(.01f); ox=(width-900f*scale)/2; oy=(height-500f*scale)/2
  paint.color=Color.WHITE; paint.textSize=42f; paint.textAlign=Paint.Align.CENTER
  canvas.drawText("${game.scores[0]}    :    ${game.scores[1]}",width/2f,50f,paint)
  canvas.save(); canvas.translate(ox,oy); canvas.scale(scale,scale)
  paint.color=Color.rgb(10,15,31); canvas.drawRect(0f,0f,900f,500f,paint)
  paint.color=Color.GRAY; for(y in 0 until 500 step 24) canvas.drawRect(448f,y.toFloat(),452f,y+12f,paint)
  paint.color=Color.CYAN; canvas.drawRect(32f,game.player-45,44f,game.player+45,paint); canvas.drawRect(856f,game.cpu-45,868f,game.cpu+45,paint)
  paint.color=Color.WHITE; canvas.drawCircle(game.ball.x,game.ball.y,8f,paint); canvas.restore()
  paint.textSize=24f
  val status=when(game.state) { GameState.PAUSED->"Paused"; GameState.OVER->if(game.scores[0]==7) "You win!" else "CPU wins"; else->"First to 7 / Drag to move" }
  canvas.drawText(status,width/2f,height-20f,paint)
 }
 override fun onTouchEvent(event: MotionEvent): Boolean {
  if(scripted) return true
  when(event.actionMasked) {
   MotionEvent.ACTION_DOWN,MotionEvent.ACTION_POINTER_DOWN -> {
    val index=event.actionIndex
    touch.down(event.getPointerId(index),(event.getY(index)-oy)/scale)
   }
   MotionEvent.ACTION_MOVE -> {
    val index=touch.pointerId?.let { event.findPointerIndex(it) } ?: -1
    if(index>=0) touch.move(event.getPointerId(index),(event.getY(index)-oy)/scale)
    else touch.clear()
   }
   MotionEvent.ACTION_POINTER_UP -> touch.up(event.getPointerId(event.actionIndex))
   MotionEvent.ACTION_UP -> { touch.clear(); performClick() }
   MotionEvent.ACTION_CANCEL -> touch.clear()
  }
  return true
 }
 override fun performClick(): Boolean { super.performClick(); return true }
}
