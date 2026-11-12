package com.intrepion.pong.nativekotlin

/** One finger owns movement until it lifts; other fingers never inherit its target. */
class TouchInput {
    var pointerId: Int? = null
        private set
    var targetY: Float? = null
        private set

    fun down(id: Int, y: Float) {
        if (pointerId == null) {
            pointerId = id
            targetY = y
        }
    }
    fun move(id: Int, y: Float) {
        if (pointerId == id) targetY = y
    }
    fun up(id: Int) {
        if (pointerId == id) clear()
    }
    fun clear() {
        pointerId = null
        targetY = null
    }
}
