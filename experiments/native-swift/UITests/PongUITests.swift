import XCTest
final class PongUITests: XCTestCase {
    func testDebugReplayCompletes() {
        let app = XCUIApplication()
        app.launchArguments = ["--scripted-test"]
        app.launch()
        let status = app.staticTexts["replay-status"]
        XCTAssertTrue(status.waitForExistence(timeout: 15))
        let done = NSPredicate(format: "label == %@", "Replay complete: 1440 ticks, 6 PNG frames")
        expectation(for: done, evaluatedWith: status)
        waitForExpectations(timeout: 40)
        XCTAssertFalse(app.buttons["Start"].isEnabled)
    }
    func testStartPauseResumeRestart() {
        let app = XCUIApplication(); app.launch()
        XCTAssertTrue(app.buttons["Start"].waitForExistence(timeout:15))
        app.buttons["Start"].tap()
        XCTAssertTrue(app.buttons["Restart"].exists)
        app.buttons["Pause"].tap()
        XCTAssertTrue(app.staticTexts["Paused"].exists)
        app.buttons["Resume"].tap()
        XCTAssertTrue(app.buttons["Pause"].exists)
        app.buttons["Restart"].tap()
        XCTAssertTrue(app.staticTexts["0     :     0"].exists)
    }

    func testPausedDragRequiresFreshGestureAfterResume() {
        let app = XCUIApplication(); app.launch()
        XCTAssertTrue(app.buttons["Start"].waitForExistence(timeout: 15))
        app.buttons["Start"].tap()
        let court = app.otherElements["pong-court"]
        XCTAssertTrue(court.waitForExistence(timeout: 10))
        func player() -> Float {
            let value = court.value as? String ?? ""
            return Float(value.split(separator: ";").first?.split(separator: "=").last ?? "") ?? -999
        }
        app.buttons["Pause"].tap()
        XCTAssertTrue(app.staticTexts["Paused"].exists)
        let before = player()
        let center = court.coordinate(withNormalizedOffset: CGVector(dx: 0.3, dy: 0.5))
        let bottom = court.coordinate(withNormalizedOffset: CGVector(dx: 0.3, dy: 0.97))
        center.press(forDuration: 0.1, thenDragTo: bottom, withVelocity: .slow, thenHoldForDuration: 0.5)
        XCTAssertEqual(player(), before, accuracy: 0.1, "Paused drag must not move the paddle")
        app.buttons["Resume"].tap()
        Thread.sleep(forTimeInterval: 1)
        XCTAssertEqual(player(), before, accuracy: 0.1, "Released paused drag must not move on resume")
        center.press(forDuration: 0.1, thenDragTo: bottom, withVelocity: .slow, thenHoldForDuration: 0.5)
        XCTAssertGreaterThan(player(), before + 50, "A fresh drag must move after resume")
    }

    func testTouchReleaseBackgroundAndFullMatch() {
        let app = XCUIApplication(); app.launch()
        XCTAssertTrue(app.buttons["Start"].waitForExistence(timeout:15))
        app.buttons["Start"].tap()
        let court = app.otherElements["pong-court"]
        XCTAssertTrue(court.waitForExistence(timeout: 10))
        func player() -> Float {
            let value = court.value as? String ?? ""
            return Float(value.split(separator: ";").first?.split(separator: "=").last ?? "") ?? -999
        }
        let initial = player()
        let center = court.coordinate(withNormalizedOffset: CGVector(dx: 0.3, dy: 0.5))
        let bottom = court.coordinate(withNormalizedOffset: CGVector(dx: 0.3, dy: 0.97))
        center.press(forDuration: 0.1, thenDragTo: bottom, withVelocity: .slow, thenHoldForDuration: 1)
        let released = player()
        XCTAssertGreaterThan(released, initial + 50, "Touch drag must move paddle")
        Thread.sleep(forTimeInterval: 1)
        XCTAssertEqual(player(), released, accuracy: 0.1, "Release must clear touch input")
        XCUIDevice.shared.press(.home)
        app.activate()
        XCTAssertTrue(app.staticTexts["Paused"].waitForExistence(timeout:10))
        XCTAssertEqual(player(), released, accuracy: 0.1, "Background must not move paddle")
        app.buttons["Resume"].tap()
        // Real normal gameplay, no score injection or replay. Player stays at the edge.
        let ended = NSPredicate(format: "exists == true")
        expectation(for: ended, evaluatedWith: app.buttons["Start"])
        waitForExpectations(timeout: 180)
        XCTAssertTrue(app.staticTexts["CPU wins"].exists || app.staticTexts["You win!"].exists)
        app.buttons["Start"].tap()
        XCTAssertTrue(app.staticTexts["0     :     0"].exists)
    }
}
