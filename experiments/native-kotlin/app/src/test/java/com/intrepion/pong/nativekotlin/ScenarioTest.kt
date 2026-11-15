package com.intrepion.pong.nativekotlin
import org.junit.Assert.*
import org.junit.Test
class ScenarioTest {
    private fun fixture() = checkNotNull(javaClass.getResource("/smoke-v1.txt")).readText()
    @Test fun portableReplay() {
        val m = ScenarioManifest.parse(fixture()); val a = ScenarioReplay(m); val b = ScenarioReplay(m)
        val captures = mutableListOf<Long>(); var reports = 1
        while (!a.finished) {
            a.step(); b.step(); assertEquals(a.game.ball.x, b.game.ball.x, 0f); assertArrayEquals(a.game.scores, b.game.scores)
            if (a.captureDue) captures += a.tick
            if (a.reportDue) reports++
            when(a.tick) {
                120L -> assertEquals(45f, a.game.player, 0f)
                240L -> assertEquals(455f, a.game.player, 0f)
                480L -> assertEquals(GameState.PAUSED, a.game.state)
                600L -> assertEquals(GameState.PLAYING, a.game.state)
                1320L -> assertArrayEquals(intArrayOf(0,0), a.game.scores)
            }
        }
        assertEquals(m.captures, captures); assertEquals(13, reports)
        val x = a.game.ball.x; a.step(); assertEquals(m.end, a.tick); assertEquals(x, a.game.ball.x, 0f)
    }
    @Test fun rejectMalformed() {
        val text = fixture()
        for (bad in listOf(text + "\nseed 3", text.replace("pause", "invalid"), text.replace("step_hz 120", "step_hz 0"), text + "\ninput 0 9 1")) {
            assertThrows(IllegalArgumentException::class.java) { ScenarioManifest.parse(bad) }
        }
    }
}
