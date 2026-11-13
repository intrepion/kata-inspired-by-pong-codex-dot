import Foundation
for state: GameState in [.ready, .paused, .over] {
    var game = Game(); var touch = TouchInput(); game.state = state
    touch.update(450, playing: game.state == .playing)
    if state == .paused { game.togglePause() } else { touch.suppressActive(); game.start() }
    touch.update(430, playing: true); let before = game.player
    game.step(0.033, input: touch.axis(player: game.player)); precondition(game.player == before, "non-playing drag rearmed: \(state)")
    touch.end(); touch.update(430, playing: true)
    game.step(0.033, input: touch.axis(player: game.player)); precondition(game.player > before, "fresh drag failed: \(state)")
}
var game = Game(); var touch = TouchInput(); game.start(); touch.update(450, playing: true)
game.state = .over; touch.suppressActive(); game.start(); touch.update(430, playing: true)
let before = game.player; game.step(0.033, input: touch.axis(player: game.player)); precondition(game.player == before, "held drag crossed restart")
touch.end(); touch.update(430, playing: true); game.step(0.033, input: touch.axis(player: game.player)); precondition(game.player > before, "fresh drag after restart failed")
print("PASS: Ready, Paused, Over drag and active drag at game-over remain suppressed until end and fresh drag")
