import XCTest
@testable import PongCore
final class GameTests: XCTestCase {
    func game() -> Game { var g = Game(); g.start(); g.serve = 0; return g }
    func testBounds() { var g = game(); for _ in 0..<100 { g.step(0.033,input:-1) }; XCTAssertEqual(g.player,45) }
    func testPaddleSteering() { var g = game(); g.ball = Ball(x:60,y:275,vx:-350,vy:0); g.step(0.033,input:0); XCTAssertGreaterThan(g.ball.vx,0); XCTAssertGreaterThan(g.ball.vy,0) }
    func testWall() { var g = game(); g.ball = Ball(x:450,y:9,vx:300,vy:-200); g.step(0.033,input:0); XCTAssertGreaterThan(g.ball.vy,0) }
    func testWinRestart() { var g = game(); g.scores = [6,0]; g.ball = Ball(x:905,y:20,vx:350,vy:0); g.step(0.033,input:0); XCTAssertEqual(g.scores,[7,0]); XCTAssertEqual(g.state,.over); g.start(); XCTAssertEqual(g.scores,[0,0]) }
    func testPause() { var g = game(); let x = g.ball.x; g.pause(); g.step(0.033,input:1); XCTAssertEqual(g.ball.x,x); XCTAssertEqual(g.player,250); g.togglePause(); XCTAssertEqual(g.state,.playing) }
    func testCPUReaction() { var g = game(); g.ball = Ball(x:500,y:400,vx:350,vy:0); g.step(0.033,input:0); XCTAssertEqual(g.cpu,250); for _ in 0..<10 { g.step(0.033,input:0) }; XCTAssertGreaterThan(g.cpu,250) }
    func testCPUCanLose() { var g = game(); g.ball = Ball(x:750,y:50,vx:700,vy:0); for _ in 0..<20 { g.step(0.033,input:0) }; XCTAssertEqual(g.scores[0],1) }
    func testDeterminism() { var a = game(), b = game(); for _ in 0..<200 { a.step(0.016,input:0); b.step(0.016,input:0) }; XCTAssertEqual(a.ball.x,b.ball.x); XCTAssertEqual(a.cpu,b.cpu) }
    func testInvalidTime() { var g = game(); let x = g.ball.x; g.step(.nan,input:0); g.step(-1,input:0); XCTAssertEqual(g.ball.x,x) }
}
