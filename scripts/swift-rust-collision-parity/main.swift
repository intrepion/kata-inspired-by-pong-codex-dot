import Foundation
while let line = readLine() {
    let v = line.split(separator:",").map { Float($0)! }
    var g = Game(seed:42); g.start(); g.serve = 0
    g.ball = Ball(x:v[0],y:v[1],vx:v[2],vy:v[3]); g.player = v[4]; g.cpu = v[5]
    g.step(v[6],input:v[7])
    print("\(g.ball.x),\(g.ball.y),\(g.ball.vx),\(g.ball.vy),\(g.player),\(g.cpu)")
}
