import SwiftUI
import Combine
import PongCore
@main struct PongApp: App { var body: some Scene { WindowGroup { PongView() } } }
struct PongView: View {
    @State private var game = Game()
#if DEBUG
    @StateObject private var demo = ScriptedDemo()
    private var scripted: Bool { demo.enabled }
    private var displayedGame: Game { demo.replay?.game ?? game }
#else
    private var scripted: Bool { false }
    private var displayedGame: Game { game }
#endif
    @State private var touch = TouchInput()
    @GestureState private var dragging = false
    @State private var last = Date()
    @Environment(\.scenePhase) private var phase
    private let timer = Timer.publish(every: 1.0/60, on: .main, in: .common).autoconnect()
    var body: some View {
        VStack(spacing: 12) {
            Text("PONG · Native Swift").font(.headline)
            Text("\(displayedGame.scores[0])     :     \(displayedGame.scores[1])").font(.largeTitle.monospacedDigit())
            GeometryReader { geo in
                PongCourt(game: displayedGame).gesture(DragGesture(minimumDistance:0).updating($dragging) { _, active, _ in active = true }.onChanged { value in
                    guard !scripted else { return }
                    let scale = min(geo.size.width/900,geo.size.height/500)
                    let y = Float((value.location.y-(geo.size.height-500*scale)/2)/scale)
                    touch.update(y, playing: game.state == .playing)
                }.onEnded { _ in touch.end() })
            }.aspectRatio(1.8,contentMode:.fit)
            .accessibilityElement(children: .ignore)
            .accessibilityIdentifier("pong-court")
            .accessibilityLabel("Pong court")
#if DEBUG
            .accessibilityValue("player=\(displayedGame.player);ballX=\(displayedGame.ball.x);state=\(String(describing: displayedGame.state))")
#endif
            Text(displayedGame.state == .over ? (displayedGame.scores[0] == 7 ? "You win!" : "CPU wins") : displayedGame.state == .paused ? "Paused" : "First to 7 · Drag to move")
#if DEBUG
            if scripted { Text(demo.status).accessibilityIdentifier("replay-status") }
#endif
            HStack {
                Button(game.state == .ready || game.state == .over ? "Start" : "Restart") { touch.suppressActive(); game.start(); last = Date() }
                Button(game.state == .paused ? "Resume" : "Pause") { if game.state == .playing { touch.suppressActive() }; game.togglePause(); last = Date() }.disabled(game.state == .ready || game.state == .over)
            }.buttonStyle(.borderedProminent).disabled(scripted)
        }.padding().preferredColorScheme(.dark)
        .onReceive(timer) { now in
#if DEBUG
            if scripted { demo.advance(); return }
#endif
            game.step(Float(now.timeIntervalSince(last)),input:touch.axis(player: game.player)); last = now }
        .onChange(of: dragging) { active in if !active { touch.end() } }
        .onChange(of:phase) { newPhase in guard !scripted else { return }; if newPhase != .active { touch.suppressActive(); game.pause() }; last = Date() }
    }
}

/// Shared scene renderer used by both normal gameplay and DEBUG frame exports.
struct PongCourt: View {
    let game: Game
    var body: some View {
        Canvas { context, size in
                    let scale = min(size.width/900,size.height/500)
                    context.translateBy(x:(size.width-900*scale)/2,y:(size.height-500*scale)/2); context.scaleBy(x:scale,y:scale)
                    context.fill(Path(CGRect(x:0,y:0,width:900,height:500)),with:.color(Color(red:0.04,green:0.06,blue:0.12)))
                    for y in stride(from:0,to:500,by:24) { context.fill(Path(CGRect(x:448,y:y,width:4,height:12)),with:.color(.gray)) }
                    for (x,y) in [(Float(38),game.player),(862,game.cpu)] { context.fill(Path(CGRect(x:CGFloat(x-6),y:CGFloat(y-45),width:12,height:90)),with:.color(.cyan)) }
                    context.fill(Path(ellipseIn:CGRect(x:CGFloat(game.ball.x-8),y:CGFloat(game.ball.y-8),width:16,height:16)),with:.color(.white))
                }
    }
}
struct PongFrame: View {
    let game: Game
    var body: some View {
        VStack(spacing: 8) {
            Text("PONG · Native Swift  \(game.scores[0]) : \(game.scores[1]) · \(String(describing: game.state))").foregroundStyle(.white)
            PongCourt(game: game).frame(width: 900, height: 500)
        }
    }
}
