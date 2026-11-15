package com.intrepion.pong.nativekotlin

/** Portable scenario actions, independent of Android and of the Rust physics implementation. */
data class ScenarioEvent(val tick: Long, val action: String)
data class ScenarioInput(val start: Long, val end: Long, val axis: Float)
data class ScenarioManifest(val id: String, val seed: ULong, val hz: Long, val end: Long,
    val reportEvery: Long, val events: List<ScenarioEvent>, val inputs: List<ScenarioInput>, val captures: List<Long>) {
    companion object {
        fun parse(text: String): ScenarioManifest {
            val fields = mutableMapOf<String, String>()
            val events = mutableListOf<ScenarioEvent>()
            val inputs = mutableListOf<ScenarioInput>()
            val captures = mutableListOf<Long>()
            fun number(s: String): Long {
                require(s.matches(Regex("[0-9]+")))
                return s.toLong().also { require(it <= 4294967295L) }
            }
            for (line in text.lineSequence()) {
                val clean = line.substringBefore('#').trim()
                if (clean.isEmpty()) continue
                val t = clean.split(Regex("\\s+"))
                when (t[0]) {
                    "event" -> { require(t.size == 3 && t[2] in listOf("start", "pause", "toggle_pause")); events += ScenarioEvent(number(t[1]), t[2]) }
                    "input" -> { require(t.size == 4 && t[3] in listOf("-1", "0", "1")); inputs += ScenarioInput(number(t[1]), number(t[2]), t[3].toFloat()) }
                    "capture" -> { require(t.size == 2); captures += number(t[1]) }
                    else -> { require(t.size == 2 && t[0] in listOf("schema", "id", "seed", "step_hz", "end_tick", "report_every")); require(fields.put(t[0], t[1]) == null) }
                }
            }
            require(fields.size == 6 && fields["schema"] == "1")
            val id = fields.getValue("id"); require(id.matches(Regex("[A-Za-z0-9-]+")))
            val seed = number(fields.getValue("seed")).toULong()
            val hz = number(fields.getValue("step_hz")); val end = number(fields.getValue("end_tick")); val report = number(fields.getValue("report_every"))
            require(hz > 0 && end > 0 && report > 0)
            require(events.all { it.tick < end } && events.zipWithNext().all { (a,b) -> a.tick < b.tick })
            require(inputs.all { it.start < it.end && it.end <= end } && inputs.zipWithNext().all { (a,b) -> a.end <= b.start })
            require(captures.all { it > 0 && it <= end } && captures.zipWithNext().all { (a,b) -> a < b })
            return ScenarioManifest(id, seed, hz, end, report, events, inputs, captures)
        }
    }
}

/** Call step once per logical tick, draw game, then inspect captureDue/reportDue.
 * No wall clock, platform input injection, file writes or UI mode changes. */
class ScenarioReplay(val manifest: ScenarioManifest) {
    val game = Game(manifest.seed)
    var tick = 0L; private set
    val finished get() = tick >= manifest.end
    val captureDue get() = tick in manifest.captures
    val reportDue get() = tick % manifest.reportEvery == 0L
    fun step() {
        if (finished) return
        for (event in manifest.events.filter { it.tick == tick }) when(event.action) {
            "start" -> game.start(); "pause" -> game.pause(); "toggle_pause" -> game.togglePause()
        }
        val axis = manifest.inputs.firstOrNull { tick >= it.start && tick < it.end }?.axis ?: 0f
        game.step(1f / manifest.hz.toFloat(), axis)
        tick++
    }
}
