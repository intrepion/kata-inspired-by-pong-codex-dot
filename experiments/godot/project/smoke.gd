extends SceneTree

func _initialize() -> void:
    var sim := PongSimulation.new()
    root.add_child(sim)
    assert(sim.snapshot().state == "Ready")
    sim.start()
    assert(sim.snapshot().state == "Playing")
    var y: float = sim.snapshot().player
    sim.advance(0.016, -1.0)
    assert(sim.snapshot().player < y)
    sim.pause()
    y = sim.snapshot().player
    sim.advance(0.016, 1.0)
    assert(sim.snapshot().player == y)
    assert(sim.snapshot().state == "Paused")
    sim.toggle_pause()
    assert(sim.snapshot().state == "Playing")
    sim.start()
    assert(sim.snapshot().score_player == 0)
    assert(sim.snapshot().score_cpu == 0)
    assert(sim.snapshot().player == 250.0)
    print("GODOT_PONG_SMOKE_PASS: extension, start, move, pause, resume, reset")
    quit()
