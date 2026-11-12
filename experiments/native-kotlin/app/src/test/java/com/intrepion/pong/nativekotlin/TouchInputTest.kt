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
}
