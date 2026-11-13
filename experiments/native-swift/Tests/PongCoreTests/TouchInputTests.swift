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

    func testPauseWithoutActiveGestureAllowsNextDrag() {
        var touch = TouchInput()
        touch.suppressActive()
        touch.update(450, playing: true)
        XCTAssertEqual(touch.axis(player: 250), 1)
    }
}
