import XCTest
@testable import PongCore

final class TouchInputTests: XCTestCase {
    func testPauseSuppressesContinuingDragUntilEnd() {
        var game = Game()
        var touch = TouchInput()
        game.start()
        touch.update(450, playing: true)
        XCTAssertEqual(touch.axis(player: game.player), 1)
        touch.suppressActive()
        game.togglePause()
        touch.update(440, playing: false)
        game.togglePause()
        touch.update(430, playing: true)
        let before = game.player
        game.step(0.033, input: touch.axis(player: game.player))
        XCTAssertEqual(game.player, before)
        touch.end()
        touch.update(430, playing: true)
        game.step(0.033, input: touch.axis(player: game.player))
        XCTAssertGreaterThan(game.player, before)
    }

    func testPausedDragAndCancellationDoNotBlockFreshGesture() {
        var touch = TouchInput()
        touch.update(450, playing: false)
        touch.update(450, playing: true)
        XCTAssertEqual(touch.axis(player: 250), 0)
        touch.end()
        touch.update(50, playing: true)
        XCTAssertEqual(touch.axis(player: 250), -1)
        touch.suppressActive()
        touch.end()
        touch.update(450, playing: true)
        XCTAssertEqual(touch.axis(player: 250), 1)
    }

    func testHeldGestureThroughWinningScoreRequiresFreshDragAfterRestart() {
        for (scores, x, vx) in [([6, 2], Float(907), Float(350)), ([2, 6], Float(-7), Float(-350))] {
            var game = Game()
            var touch = TouchInput()
            game.start()
            touch.update(400, playing: true)
            game.scores = scores
            game.serve = 0
            game.ball.x = x
            game.ball.y = 250
            game.ball.vx = vx
            game.ball.vy = 0
            game.step(0.02, input: 0)
            XCTAssertEqual(game.state, .over)
            // Mirrors the app's restart button ordering.
            touch.suppressActive()
            game.start()
            touch.update(450, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), 0)
            touch.end()
            touch.update(400, playing: true)
            XCTAssertNotEqual(touch.axis(player: game.player), 0)
        }
    }

    func testNonFiniteCoordinatesClearTargetAndPreserveGestureSuppression() {
        for invalid in [Float.nan, Float.infinity, -Float.infinity] {
            var game = Game(); game.start()
            var touch = TouchInput()
            touch.update(invalid, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), 0)
            touch.update(100, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), -1)
            touch.update(invalid, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), 0, "invalid coordinate must clear stale target")
            let before = game.player
            game.step(0.033, input: touch.axis(player: game.player))
            XCTAssertEqual(game.player, before)
            touch.update(100, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), -1, "valid coordinate can recover")

            touch.end()
            touch.update(invalid, playing: false)
            touch.update(100, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), 0, "drag begun while paused stays suppressed")
            touch.end()
            touch.update(100, playing: true)
            XCTAssertEqual(touch.axis(player: game.player), -1, "fresh drag works")
        }
    }

    func testPauseWithoutActiveGestureAllowsNextDrag() {
        var touch = TouchInput()
        touch.suppressActive()
        touch.update(450, playing: true)
        XCTAssertEqual(touch.axis(player: 250), 1)
    }
}
