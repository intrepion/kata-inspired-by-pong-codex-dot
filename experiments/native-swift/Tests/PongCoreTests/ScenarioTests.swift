import XCTest
@testable import PongCore
final class ScenarioTests: XCTestCase {
    func fixture() throws -> String {
        let url = try XCTUnwrap(Bundle.module.url(forResource: "smoke-v1", withExtension: "txt", subdirectory: "Fixtures"))
        return try String(contentsOf: url, encoding: .utf8)
    }
    func testPortableReplay() throws {
        let m = try ScenarioManifest.parse(fixture())
        var a = ScenarioReplay(m), b = ScenarioReplay(m)
        var captures: [UInt32] = [], reports = 1
        while !a.finished {
            a.step(); b.step()
            XCTAssertEqual(a.game.ball.x, b.game.ball.x)
            XCTAssertEqual(a.game.scores, b.game.scores)
            if a.captureDue { captures.append(a.tick) }; if a.reportDue { reports += 1 }
            if a.tick == 120 { XCTAssertEqual(a.game.player, 45) }
            if a.tick == 240 { XCTAssertEqual(a.game.player, 455) }
            if a.tick == 480 { XCTAssertEqual(a.game.state, .paused) }
            if a.tick == 600 { XCTAssertEqual(a.game.state, .playing) }
            if a.tick == 1320 { XCTAssertEqual(a.game.scores, [0,0]) }
        }
        XCTAssertEqual(captures, m.captures); XCTAssertEqual(reports, 13)
        let x = a.game.ball.x; a.step(); XCTAssertEqual(a.tick, m.end); XCTAssertEqual(a.game.ball.x, x)
    }
    func testRejectMalformed() throws {
        let text = try fixture()
        for bad in [text + "\nseed 3", text.replacingOccurrences(of: "pause", with: "invalid"), text.replacingOccurrences(of: "step_hz 120", with: "step_hz 0"), text + "\ninput 0 9 1"] {
            XCTAssertThrowsError(try ScenarioManifest.parse(bad))
        }
    }
}
