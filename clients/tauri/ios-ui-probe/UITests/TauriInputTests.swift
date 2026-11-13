import XCTest
final class TauriInputTests: XCTestCase {
    func testStartPauseResume() {
        continueAfterFailure = false
        let app = XCUIApplication(bundleIdentifier: "com.intrepion.katapongcomparison")
        app.launch()
        let start = app.buttons["Start / restart"]
        XCTAssertTrue(start.waitForExistence(timeout: 60), "Start not accessible: \(app.debugDescription)")
        let ready = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Ready'")).firstMatch
        XCTAssertTrue(ready.waitForExistence(timeout: 90), "Ready not reached: \(app.debugDescription)")
        start.tap()
        let playing = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Playing'")).firstMatch
        XCTAssertTrue(playing.waitForExistence(timeout: 15), "Start did not reach Playing: \(app.debugDescription)")
        let pause = app.buttons["Pause / resume"]
        XCTAssertTrue(pause.exists)
        pause.tap()
        let paused = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Paused'")).firstMatch
        XCTAssertTrue(paused.waitForExistence(timeout: 15), "Pause did not reach Paused: \(app.debugDescription)")
        pause.tap()
        XCTAssertTrue(playing.waitForExistence(timeout: 15), "Resume did not reach Playing: \(app.debugDescription)")
    }
    func testMatchOverAndRestart() {
        continueAfterFailure = false
        let app = XCUIApplication(bundleIdentifier: "com.intrepion.katapongcomparison")
        app.launch()
        let start = app.buttons["Start / restart"]
        XCTAssertTrue(start.waitForExistence(timeout: 120))
        let ready = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Ready'")).firstMatch
        XCTAssertTrue(ready.waitForExistence(timeout: 90))
        start.tap()
        let playing = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Playing'")).firstMatch
        XCTAssertTrue(playing.waitForExistence(timeout: 15))
        let over = app.staticTexts.matching(NSPredicate(format: "label CONTAINS 'Match over'")).firstMatch
        XCTAssertTrue(over.waitForExistence(timeout: 180), "No match over: \(app.debugDescription)")
        start.tap()
        XCTAssertTrue(playing.waitForExistence(timeout: 15), "Restart did not start: \(app.debugDescription)")
    }

}
