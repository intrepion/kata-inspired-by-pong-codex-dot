import XCTest
final class PongUITests: XCTestCase {
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
