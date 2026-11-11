import Foundation
public enum GameState { case ready, playing, paused, over }
public struct Ball { public var x: Float = 450, y: Float = 250, vx: Float = 0, vy: Float = 0 }
public struct Game {
    public var player: Float = 250, cpu: Float = 250, ball = Ball(), scores = [0, 0], state = GameState.ready, serve: Float = 0
    private var target: Float = 250, reaction: Float = 0, rng: UInt64
    public init(seed: UInt64 = 42) { rng = max(1, seed) }
    private mutating func random() -> Float { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; return Float(rng >> 40) / 16777216 }
    private mutating func reset(_ direction: Float) { ball = Ball(x: 450, y: 250, vx: direction * 350, vy: (random() - 0.5) * 180); serve = 0.65; target = 250; reaction = 0.3 }
    public mutating func start() { scores = [0,0]; player = 250; cpu = 250; state = .playing; let direction: Float = random() < 0.5 ? -1 : 1; reset(direction) }
    public mutating func pause() { if state == .playing { state = .paused } }
    public mutating func togglePause() { if state == .playing { state = .paused } else if state == .paused { state = .playing } }
    public mutating func step(_ elapsed: Float, input: Float) {
        guard state == .playing, elapsed.isFinite, input.isFinite else { return }
        let dt = clamp(elapsed, 0, 0.033)
        player = clamp(player + clamp(input,-1,1) * 470 * dt,45,455)
        reaction -= dt
        if reaction <= 0 { target = ball.vx > 0 ? clamp(ball.y + (random()-0.5)*50,45,455) : 250; reaction = 0.3 }
        let speed: Float = ball.vx > 0 ? 190 : 130
        cpu = clamp(cpu + clamp(target-cpu,-speed*dt,speed*dt),45,455)
        if serve > 0 { serve -= dt; return }
        let oldX = ball.x; ball.x += ball.vx*dt; ball.y += ball.vy*dt
        if ball.y < 8 { ball.y = 16-ball.y; ball.vy = abs(ball.vy) }
        if ball.y > 492 { ball.y = 984-ball.y; ball.vy = -abs(ball.vy) }
        for (x,y,direction): (Float,Float,Float) in [(38,player,1),(862,cpu,-1)] {
            let face = x + direction*14
            if (direction == 1 ? oldX >= face && ball.x <= face : oldX <= face && ball.x >= face) && abs(ball.y-y) <= 53 {
                let angle = clamp((ball.y-y)/45,-1,1)*1.05, speed = min(hypot(ball.vx,ball.vy)*1.055,720)
                ball.x = face; ball.vx = direction*speed*cos(angle); ball.vy = speed*sin(angle)
            }
        }
        if ball.x < -8 || ball.x > 908 { let scorer = ball.x < 0 ? 1 : 0; scores[scorer] += 1; if scores[scorer] >= 7 { state = .over } else { reset(scorer == 0 ? -1 : 1) } }
    }
}
private func clamp(_ value: Float,_ lower: Float,_ upper: Float) -> Float { min(max(value,lower),upper) }
