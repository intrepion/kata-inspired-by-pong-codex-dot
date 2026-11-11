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
  val title=TextView(this).apply { text="PONG � Native Kotlin"; textSize=24f; setTextColor(Color.WHITE); gravity=Gravity.CENTER }
  field=PongView(this)
  val buttons=LinearLayout(this).apply { gravity=Gravity.CENTER }
  val start=Button(this).apply { text="Start / Restart"; setOnClickListener { field.game.start() } }
  val pause=Button(this).apply { text="Pause / Resume"; setOnClickListener { field.game.togglePause() } }
  buttons.addView(start); buttons.addView(pause)
  layout.addView(title); layout.addView(field,LinearLayout.LayoutParams(-1,0,1f)); layout.addView(buttons)
  layout.setOnApplyWindowInsetsListener { view, insets -> view.setPadding(16,16+insets.systemWindowInsetTop,16,16+insets.systemWindowInsetBottom); insets }
  setContentView(layout)
 }
 override fun onPause() { super.onPause(); field.game.pause(); field.touchY=null; field.active=false }
 override fun onResume() { super.onResume(); field.active=true; field.last=0; field.postInvalidateOnAnimation() }
}
class PongView(context: android.content.Context): View(context) {
 val game=Game(); var touchY: Float?=null; var active=true; var last=0L
 private val paint=Paint(Paint.ANTI_ALIAS_FLAG)
 private var scale=1f; private var ox=0f; private var oy=0f
 override fun onDraw(canvas: Canvas) {
  super.onDraw(canvas)
  val now=System.nanoTime(); if(last!=0L) game.step((now-last)/1_000_000_000f,touchY?.let { ((it-game.player)/20f).coerceIn(-1f,1f) } ?: 0f); last=now
  scale=minOf(width/900f,(height-140f)/500f).coerceAtLeast(.01f); ox=(width-900f*scale)/2; oy=(height-500f*scale)/2
  paint.color=Color.WHITE; paint.textSize=42f; paint.textAlign=Paint.Align.CENTER
  canvas.drawText("${game.scores[0]}    :    ${game.scores[1]}",width/2f,50f,paint)
  canvas.save(); canvas.translate(ox,oy); canvas.scale(scale,scale)
  paint.color=Color.rgb(10,15,31); canvas.drawRect(0f,0f,900f,500f,paint)
  paint.color=Color.GRAY; for(y in 0 until 500 step 24) canvas.drawRect(448f,y.toFloat(),452f,y+12f,paint)
  paint.color=Color.CYAN; canvas.drawRect(32f,game.player-45,44f,game.player+45,paint); canvas.drawRect(856f,game.cpu-45,868f,game.cpu+45,paint)
  paint.color=Color.WHITE; canvas.drawCircle(game.ball.x,game.ball.y,8f,paint); canvas.restore()
  paint.textSize=24f
  val status=when(game.state) { GameState.PAUSED->"Paused"; GameState.OVER->if(game.scores[0]==7) "You win!" else "CPU wins"; else->"First to 7 � Drag to move" }
  canvas.drawText(status,width/2f,height-20f,paint)
  if(active) postInvalidateOnAnimation()
 }
 override fun onTouchEvent(event: MotionEvent): Boolean {
  when(event.actionMasked) { MotionEvent.ACTION_DOWN,MotionEvent.ACTION_MOVE->touchY=(event.y-oy)/scale; MotionEvent.ACTION_UP,MotionEvent.ACTION_CANCEL->{ touchY=null; performClick() } }
  return true
 }
 override fun performClick(): Boolean { super.performClick(); return true }
}
