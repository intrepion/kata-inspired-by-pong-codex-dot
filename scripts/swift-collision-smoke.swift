import Foundation
func game() -> Game { var g = Game(); g.start(); g.serve = 0; return g }
var failures = 0
func check(_ ok: Bool, _ label: String) { print("\(ok ? "PASS" : "FAIL") \(label)"); if !ok { failures += 1 } }
for (x, vx): (Float, Float) in [(60, -350), (840, 350)] {
    for (y, vy, expected): (Float, Float, Bool) in [(180, 600, false), (215, -600, true)] {
        var g = game(); g.ball = Ball(x: x, y: y, vx: vx, vy: vy); g.step(0.033, input: 0)
        check((g.ball.vx * vx < 0) == expected, "crossing x=\(x) y=\(y)")
    }
    var wall = game(); wall.player = 45; wall.cpu = 45; wall.ball = Ball(x: x, y: 10, vx: vx, vy: -600); wall.step(0.033, input: 0)
    check(wall.ball.vx * vx < 0 && wall.ball.y >= 8 && wall.ball.y < 30, "prior wall x=\(x)")
    var post = game(); post.player = 45; post.cpu = 45; post.ball = Ball(x: x == 60 ? 54 : 846, y: 8.3, vx: vx, vy: 0); post.step(1/120, input: 0)
    check(post.ball.vx * vx < 0 && post.ball.y >= 8 && post.ball.vy > 0, "wall after contact x=\(x)")
}
var moving = game(); moving.ball = Ball(x: 54, y: 306.3, vx: -350, vy: 0); moving.step(1/120, input: 1)
check(moving.ball.vx < 0, "moving player at crossing")
var remaining = game(); remaining.ball = Ball(x: 54, y: 250, vx: -350, vy: 0); remaining.step(1/120, input: 0)
check(remaining.ball.x > 52.8 && remaining.ball.vx > 0, "remaining step after contact")
var cpu = game(); cpu.ball = Ball(x: 450, y: 400, vx: 350, vy: 0)
for _ in 0..<10 { cpu.step(0.033, input: 0) }
cpu.cpu = 250; cpu.ball = Ball(x: 846, y: 304.3, vx: 350, vy: 0); cpu.step(1/120, input: 0)
check(cpu.ball.vx > 0, "moving CPU at crossing")
for (scores, x, vx): ([Int], Float, Float) in [([6,2],907,350),([2,6],-7,-350)] {
    var g = game(); var touch = TouchInput(); touch.update(400, playing: true)
    g.scores = scores; g.ball = Ball(x: x,y: 250,vx: vx,vy: 0); g.step(0.02,input:0)
    let ended = g.state == .over; touch.suppressActive(); g.start(); touch.update(450,playing:true)
    let suppressed = touch.axis(player:g.player) == 0; touch.end(); touch.update(400,playing:true)
    check(ended && suppressed && touch.axis(player:g.player) != 0, "winning score held touch \(scores)")
}
if failures > 0 { exit(1) }
