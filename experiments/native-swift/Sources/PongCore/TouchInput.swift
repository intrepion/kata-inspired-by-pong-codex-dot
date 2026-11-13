/// Tracks one court drag and prevents a drag spanning pause from resuming movement.
public struct TouchInput {
    private var active = false
    private var suppressed = false
    private var target: Float?

    public init() {}

    public mutating func update(_ y: Float, playing: Bool) {
        active = true
        if !playing { suppressed = true }
        if !suppressed { target = y }
    }

    public mutating func suppressActive() {
        if active { suppressed = true }
        target = nil
    }

    public mutating func end() {
        active = false
        suppressed = false
        target = nil
    }

    public func axis(player: Float) -> Float {
        target.map { max(-1, min(1, ($0 - player) / 20)) } ?? 0
    }
}
