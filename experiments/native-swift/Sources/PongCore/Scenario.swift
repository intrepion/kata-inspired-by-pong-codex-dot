import Foundation

public struct ScenarioEvent { public let tick: UInt32; public let action: String }
public struct ScenarioInput { public let start: UInt32; public let end: UInt32; public let axis: Float }
public struct ScenarioManifest {
    public let id: String, seed: UInt64, hz: UInt32, end: UInt32, reportEvery: UInt32
    public let events: [ScenarioEvent], inputs: [ScenarioInput], captures: [UInt32]
    public enum Invalid: Error { case manifest }
    public static func parse(_ text: String) throws -> ScenarioManifest {
        var fields: [String: String] = [:], events: [ScenarioEvent] = [], inputs: [ScenarioInput] = [], captures: [UInt32] = []
        func number(_ s: String) throws -> UInt32 {
            guard !s.isEmpty, s.utf8.allSatisfy({ $0 >= 48 && $0 <= 57 }), let n = UInt32(s) else { throw Invalid.manifest }; return n
        }
        for line in text.components(separatedBy: .newlines) {
            let t = line.split(separator: "#", maxSplits: 1, omittingEmptySubsequences: false)[0].split(whereSeparator: { $0.isWhitespace }).map(String.init)
            if t.isEmpty { continue }
            switch t[0] {
            case "event":
                guard t.count == 3, ["start", "pause", "toggle_pause"].contains(t[2]) else { throw Invalid.manifest }
                events.append(ScenarioEvent(tick: try number(t[1]), action: t[2]))
            case "input":
                guard t.count == 4, ["-1", "0", "1"].contains(t[3]), let axis = Float(t[3]) else { throw Invalid.manifest }
                inputs.append(ScenarioInput(start: try number(t[1]), end: try number(t[2]), axis: axis))
            case "capture":
                guard t.count == 2 else { throw Invalid.manifest }; captures.append(try number(t[1]))
            default:
                guard t.count == 2, ["schema", "id", "seed", "step_hz", "end_tick", "report_every"].contains(t[0]), fields[t[0]] == nil else { throw Invalid.manifest }
                fields[t[0]] = t[1]
            }
        }
        guard fields.count == 6, fields["schema"] == "1", let id = fields["id"], !id.isEmpty,
              id.utf8.allSatisfy({ ($0 >= 48 && $0 <= 57) || ($0 >= 65 && $0 <= 90) || ($0 >= 97 && $0 <= 122) || $0 == 45 }) else { throw Invalid.manifest }
        let seed = try number(fields["seed"]!), hz = try number(fields["step_hz"]!), end = try number(fields["end_tick"]!), report = try number(fields["report_every"]!)
        guard hz > 0, end > 0, report > 0,
              events.allSatisfy({ $0.tick < end }), zip(events, events.dropFirst()).allSatisfy({ $0.0.tick < $0.1.tick }),
              inputs.allSatisfy({ $0.start < $0.end && $0.end <= end }), zip(inputs, inputs.dropFirst()).allSatisfy({ $0.0.end <= $0.1.start }),
              captures.allSatisfy({ $0 > 0 && $0 <= end }), zip(captures, captures.dropFirst()).allSatisfy({ $0.0 < $0.1 }) else { throw Invalid.manifest }
        return ScenarioManifest(id: id, seed: UInt64(seed), hz: hz, end: end, reportEvery: report, events: events, inputs: inputs, captures: captures)
    }
}

/// Independent native physics. Caller draws game after step; captureDue is not a screenshot.
public struct ScenarioReplay {
    public let manifest: ScenarioManifest
    public private(set) var game: Game
    public private(set) var tick: UInt32 = 0
    public var finished: Bool { tick >= manifest.end }
    public var captureDue: Bool { manifest.captures.contains(tick) }
    public var reportDue: Bool { tick % manifest.reportEvery == 0 }
    public init(_ manifest: ScenarioManifest) { self.manifest = manifest; game = Game(seed: manifest.seed) }
    public mutating func step() {
        guard !finished else { return }
        for event in manifest.events where event.tick == tick {
            switch event.action { case "start": game.start(); case "pause": game.pause(); case "toggle_pause": game.togglePause(); default: break }
        }
        let axis = manifest.inputs.first(where: { tick >= $0.start && tick < $0.end })?.axis ?? 0
        game.step(1 / Float(manifest.hz), input: axis)
        tick += 1
    }
}
