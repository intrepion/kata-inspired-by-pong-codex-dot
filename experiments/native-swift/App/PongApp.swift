import SwiftUI
import PongCore
@main struct PongApp: App { var body: some Scene { WindowGroup { PongView() } } }
struct PongView: View {
    @State private var game = Game()
    @State private var touchY: Float? = nil
    @State private var last = Date()
    @Environment(\.scenePhase) private var phase
    private let timer = Timer.publish(every: 1.0/60, on: .main, in: .common).autoconnect()
    var body: some View {
        VStack(spacing: 12) {
            Text("PONG · Native Swift").font(.headline)
            Text("\(game.scores[0])     :     \(game.scores[1])").font(.largeTitle.monospacedDigit())
            GeometryReader { geo in
                Canvas { context, size in
                    let scale = min(size.width/900,size.height/500)
                    context.translateBy(x:(size.width-900*scale)/2,y:(size.height-500*scale)/2); context.scaleBy(x:scale,y:scale)
                    context.fill(Path(CGRect(x:0,y:0,width:900,height:500)),with:.color(Color(red:0.04,green:0.06,blue:0.12)))
                    for y in stride(from:0,to:500,by:24) { context.fill(Path(CGRect(x:448,y:y,width:4,height:12)),with:.color(.gray)) }
                    for (x,y) in [(Float(38),game.player),(862,game.cpu)] { context.fill(Path(CGRect(x:CGFloat(x-6),y:CGFloat(y-45),width:12,height:90)),with:.color(.cyan)) }
                    context.fill(Path(ellipseIn:CGRect(x:CGFloat(game.ball.x-8),y:CGFloat(game.ball.y-8),width:16,height:16)),with:.color(.white))
                }.gesture(DragGesture(minimumDistance:0).onChanged { value in
                    let scale = min(geo.size.width/900,geo.size.height/500)
                    let y = Float((value.location.y-(geo.size.height-500*scale)/2)/scale)
                    touchY = y
                }.onEnded { _ in touchY = nil })
            }.aspectRatio(1.8,contentMode:.fit)
            Text(game.state == .over ? (game.scores[0] == 7 ? "You win!" : "CPU wins") : game.state == .paused ? "Paused" : "First to 7 · Drag to move")
            HStack {
                Button(game.state == .ready || game.state == .over ? "Start" : "Restart") { game.start(); last = Date() }
                Button(game.state == .paused ? "Resume" : "Pause") { game.togglePause(); last = Date() }.disabled(game.state == .ready || game.state == .over)
            }.buttonStyle(.borderedProminent)
        }.padding().preferredColorScheme(.dark)
        .onReceive(timer) { now in game.step(Float(now.timeIntervalSince(last)),input:touchY.map { max(-1,min(1,($0-game.player)/20)) } ?? 0); last = now }
        .onChange(of:phase) { newPhase in if newPhase != .active { game.pause(); touchY = nil }; last = Date() }
    }
}
