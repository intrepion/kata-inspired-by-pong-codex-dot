import XCTest
@testable import PongCore

final class CollisionTests: XCTestCase {
    func game() -> Game { var g = Game(); g.start(); g.serve = 0; return g }

    func testDiagonalCrossingOnBothSides() {
        for (x, vx): (Float, Float) in [(60, -350), (840, 350)] {
            for (y, vy, shouldBounce): (Float, Float, Bool) in [(180, 600, false), (215, -600, true)] {
                var g = game(); g.ball = Ball(x: x, y: y, vx: vx, vy: vy)
                g.step(0.033, input: 0)
                XCTAssertEqual(g.ball.vx * vx < 0, shouldBounce, "x=\(x) y=\(y)")
            }
        }
    }

    func testWallBeforeAndAfterContactOnBothSides() {
        for (x, vx): (Float, Float) in [(60, -350), (840, 350)] {
            var before = game(); before.player = 45; before.cpu = 45
            before.ball = Ball(x: x, y: 10, vx: vx, vy: -600); before.step(0.033, input: 0)
            XCTAssertLessThan(before.ball.vx * vx, 0); XCTAssertGreaterThanOrEqual(before.ball.y, 8)
            var after = game(); after.player = 45; after.cpu = 45
            after.ball = Ball(x: x == 60 ? 54 : 846, y: 8.3, vx: vx, vy: 0); after.step(1/120, input: 0)
            XCTAssertLessThan(after.ball.vx * vx, 0); XCTAssertGreaterThanOrEqual(after.ball.y, 8); XCTAssertGreaterThan(after.ball.vy, 0)
        }
    }

    func testMovingPlayerAndRemainingTime() {
        var miss = game(); miss.ball = Ball(x: 54, y: 306.3, vx: -350, vy: 0)
        miss.step(1/120, input: 1); XCTAssertLessThan(miss.ball.vx, 0)
        var hit = game(); hit.ball = Ball(x: 54, y: 250, vx: -350, vy: 0)
        hit.step(1/120, input: 0); XCTAssertGreaterThan(hit.ball.vx, 0); XCTAssertGreaterThan(hit.ball.x, 52.8)
    }

    func testMovingCPUAtContact() {
        var g = game(); g.ball = Ball(x: 450, y: 400, vx: 350, vy: 0)
        for _ in 0..<10 { g.step(0.033, input: 0) }
        g.cpu = 250; g.ball = Ball(x: 846, y: 304.3, vx: 350, vy: 0)
        g.step(1/120, input: 0); XCTAssertGreaterThan(g.ball.vx, 0)
    }
}
