package com.intrepion.pong.nativekotlin

import org.junit.Assert.*
import org.junit.Test

class TouchInputTest {
    @Test fun secondaryFingerCannotStealOrReleaseMovement() {
        val input = TouchInput()
        input.down(7, 100f)
        input.down(12, 400f)
        input.move(12, 450f)
        input.up(12)
        assertEquals(7, input.pointerId)
        assertEquals(100f, input.targetY)
        input.move(7, 120f)
        assertEquals(120f, input.targetY)
    }
    @Test fun controllingPointerUpClearsEvenWhenAnotherFingerRemains() {
        val input = TouchInput()
        input.down(7, 100f)
        input.down(12, 400f)
        input.up(7)
        input.move(12, 450f)
        assertNull(input.targetY)
        assertNull(input.pointerId)
        input.down(15, 200f)
        assertEquals(15, input.pointerId)
        assertEquals(200f, input.targetY)
    }
    @Test fun cancelAndLifecycleClearIgnoreLaterStaleMoves() {
        val input = TouchInput()
        input.down(7, 100f)
        input.clear()
        input.move(7, 400f)
        input.up(7)
        assertNull(input.targetY)
        assertNull(input.pointerId)
    }
    @Test fun heldGestureDoesNotMoveAfterPauseAndResume() {
        val game = Game()
        val input = TouchInput()
        game.start()
        input.down(7, 400f)
        input.togglePause(game)
        input.togglePause(game)
        input.move(7, 450f)
        val before = game.player
        val axis = input.targetY?.let { ((it - game.player) / 20f).coerceIn(-1f, 1f) } ?: 0f
        game.step(0.016f, axis)
        assertEquals(before, game.player, 0f)
        input.up(7)
        input.move(7, 100f)
        assertNull(input.targetY)
        input.down(8, 400f)
        assertEquals(400f, input.targetY!!, 0f)
        game.step(0.016f, ((input.targetY!! - game.player) / 20f).coerceIn(-1f, 1f))
        assertTrue(game.player > before)
        input.clear() // cancellation
        input.move(8, 450f)
        assertNull(input.targetY)
    }
    @Test fun heldGestureDoesNotMoveAfterRestart() {
        val game = Game()
        val input = TouchInput()
        game.start()
        input.down(7, 400f)
        input.start(game)
        input.move(7, 450f)
        val before = game.player
        val axis = input.targetY?.let { ((it - game.player) / 20f).coerceIn(-1f, 1f) } ?: 0f
        game.step(0.016f, axis)
        assertEquals(before, game.player, 0f)
        input.up(7)
        input.move(7, 300f)
        assertNull(input.targetY)
        input.down(8, 400f)
        game.step(0.016f, ((input.targetY!! - game.player) / 20f).coerceIn(-1f, 1f))
        assertTrue(game.player > before)
    }
}
