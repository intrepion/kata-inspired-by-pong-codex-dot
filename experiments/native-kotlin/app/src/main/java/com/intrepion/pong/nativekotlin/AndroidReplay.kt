package com.intrepion.pong.nativekotlin

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.util.UUID

/** Debug-only, opt-in app-owned replay. It does not synthesize OS input or capture other apps. */
class AndroidReplay(context: Context) {
    val scenario = ScenarioReplay(ScenarioManifest.parse(context.assets.open("scenarios/smoke-v1.txt").bufferedReader().use { it.readText() }))
    private val directory = File(context.filesDir, "replays/${UUID.randomUUID()}").apply { check(mkdirs()) }
    private val telemetry = File(directory, "telemetry.jsonl")
    private var capturedTick = -1L
    private var completed = false
    init { report() }
    fun advance() {
        if(scenario.finished) return
        scenario.step()
        if(scenario.reportDue) report()
    }
    private fun report() {
        val g = scenario.game
        val record = JSONObject().put("scenario", scenario.manifest.id).put("tick", scenario.tick)
            .put("state", g.state.name).put("player", g.player).put("cpu", g.cpu)
            .put("ball", JSONArray(listOf(g.ball.x,g.ball.y,g.ball.vx,g.ball.vy)))
            .put("scores", JSONArray(g.scores.toList())).put("finished", scenario.finished)
        telemetry.appendText(record.toString()+"\n")
    }
    fun capture(width: Int, height: Int, render: (Canvas) -> Unit) {
        if(width<=0 || height<=0) return
        if(scenario.captureDue && capturedTick!=scenario.tick) {
            val bitmap=Bitmap.createBitmap(width,height,Bitmap.Config.ARGB_8888)
            try {
                render(Canvas(bitmap))
                File(directory,"tick-%04d.png".format(scenario.tick)).outputStream().use {
                    check(bitmap.compress(Bitmap.CompressFormat.PNG,100,it))
                }
                capturedTick=scenario.tick
            } finally { bitmap.recycle() }
        }
        if(scenario.finished && !completed) {
            File(directory,"complete.json").writeText(JSONObject().put("tick",scenario.tick)
                .put("captureCount",directory.listFiles()?.count { it.extension=="png" } ?: 0).toString())
            completed=true
        }
    }
}
