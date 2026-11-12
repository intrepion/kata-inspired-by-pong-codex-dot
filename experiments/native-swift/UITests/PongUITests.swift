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
}
