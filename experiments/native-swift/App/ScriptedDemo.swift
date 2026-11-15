#if DEBUG
import SwiftUI
import PongCore

/// Explicit opt-in, app-owned render export. Never captures other apps or the OS.
@MainActor final class ScriptedDemo: ObservableObject {
    @Published private(set) var replay: ScenarioReplay?
    @Published private(set) var status = ""
    private var directory: URL?
    private var reports = ""
    let enabled = ProcessInfo.processInfo.arguments.contains("--scripted-test")
    init() {
        guard enabled else { return }
        do {
            guard let fixture = Bundle.main.url(forResource: "smoke-v1", withExtension: "txt") else { throw Failure.missingFixture }
            let manifest = try ScenarioManifest.parse(String(contentsOf: fixture, encoding: .utf8))
            let root = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
            let output = root.appendingPathComponent("replay-" + UUID().uuidString, isDirectory: true)
            try FileManager.default.createDirectory(at: output, withIntermediateDirectories: false)
            directory = output
            replay = ScenarioReplay(manifest)
            try report()
            status = "Replay tick 0"
            print("PONG_REPLAY_OUTPUT \(output.path)")
        } catch { status = "Replay failed: \(error)" }
    }
    enum Failure: Error { case missingFixture, renderFailed }
    func advance() {
        guard let current = replay, !current.finished, !status.hasPrefix("Replay failed") else { return }
        do {
            // Two fixed 120Hz ticks per display update. No wall-clock catch-up or live input.
            for _ in 0..<2 {
                replay?.step()
                guard let state = replay else { return }
                if state.reportDue { try report() }
                if state.captureDue { try capture(state) }
                if state.finished {
                    try Data("complete\n".utf8).write(to: directory!.appendingPathComponent("complete.txt"), options: .atomic)
                    status = "Replay complete: \(state.tick) ticks, 6 PNG frames"
                    return
                }
            }
            status = "Replay tick \(replay!.tick)"
        } catch { status = "Replay failed: \(error)" }
    }
    private func report() throws {
        let state = replay!, g = state.game
        let record: [String: Any] = ["scenario":state.manifest.id,"seed":state.manifest.seed,"tick":state.tick,
            "state":String(describing:g.state),"player":g.player,"cpu":g.cpu,
            "ball":[g.ball.x,g.ball.y,g.ball.vx,g.ball.vy],"scores":g.scores,"finished":state.finished]
        let data = try JSONSerialization.data(withJSONObject:record,options:[.sortedKeys])
        reports += String(decoding:data,as:UTF8.self) + "\n"
        try Data(reports.utf8).write(to:directory!.appendingPathComponent("telemetry.jsonl"),options:.atomic)
    }
    private func capture(_ state: ScenarioReplay) throws {
        // Same SwiftUI court as the live view, rendered at fixed logical size.
        // This is a render export, not a screenshot of the simulator/window compositor.
        let renderer = ImageRenderer(content: PongFrame(game:state.game).frame(width:900,height:560).background(Color.black).preferredColorScheme(.dark))
        renderer.scale = 1
        guard let data = renderer.uiImage?.pngData() else { throw Failure.renderFailed }
        try data.write(to:directory!.appendingPathComponent(String(format:"tick-%04d.png",state.tick)),options:.atomic)
    }
}
#endif
