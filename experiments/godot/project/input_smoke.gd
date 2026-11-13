extends SceneTree

func _initialize() -> void:
    call_deferred("run")

func key(code: int, pressed: bool) -> void:
    var event := InputEventKey.new()
    event.physical_keycode = code
    event.pressed = pressed
    Input.parse_input_event(event)

func check(condition: bool, message: String) -> bool:
    if not condition:
        push_error(message)
        quit(1)
    return condition

func run() -> void:
    var app = load("res://main.tscn").instantiate()
    root.add_child(app)
    app.set_physics_process(false)
    # Check a press in Ready, including start in the same sampled frame.
    for code in [KEY_W, KEY_UP, KEY_S, KEY_DOWN]:
        var ready_app = load("res://main.tscn").instantiate()
        root.add_child(ready_app)
        ready_app.set_physics_process(false)
        key(code, true)
        await process_frame
        var start := InputEventKey.new()
        start.physical_keycode = KEY_SPACE
        start.pressed = true
        ready_app._unhandled_input(start)
        ready_app.state = ready_app.simulation.snapshot()
        if not check(ready_app.state.state == "Playing", "Start failed"): return
        var initial: float = ready_app.state.player
        ready_app._physics_process(0.016)
        if not check(ready_app.simulation.snapshot().player == initial, "Key held in Ready moved after start: %s" % code): return
        key(code, false)
        await process_frame
        ready_app._physics_process(0.016)
        key(code, true)
        await process_frame
        ready_app._physics_process(0.016)
        if not check(ready_app.simulation.snapshot().player != initial, "Fresh key after Ready did not move: %s" % code): return
        key(code, false)
        await process_frame
        ready_app.queue_free()
    for code in [KEY_W, KEY_UP, KEY_S, KEY_DOWN]:
      for pause_code in [KEY_SPACE, KEY_ENTER, KEY_P]:
        app.simulation.start()
        app.state = app.simulation.snapshot()
        key(code, true)
        await process_frame
        app._physics_process(0.016)
        var pause := InputEventKey.new()
        pause.physical_keycode = pause_code
        pause.pressed = true
        app._unhandled_input(pause)
        app.state = app.simulation.snapshot()
        if not check(app.state.state == "Paused", "Pause failed"): return
        app._unhandled_input(pause)
        app.state = app.simulation.snapshot()
        var before: float = app.state.player
        app._physics_process(0.016)
        if not check(app.simulation.snapshot().player == before, "Held key moved after pause: %s" % code): return
        key(code, false)
        await process_frame
        app._physics_process(0.016)
        key(code, true)
        await process_frame
        app._physics_process(0.016)
        if not check(app.simulation.snapshot().player != before, "Fresh key failed to move: %s" % code): return
        key(code, false)
        await process_frame
        app._physics_process(0.016)
    # Press while paused, then resume while the same physical key is held.
    for code in [KEY_W, KEY_UP, KEY_S, KEY_DOWN]:
      for resume_code in [KEY_SPACE, KEY_ENTER, KEY_P, KEY_R]:
        app.simulation.start()
        app.simulation.pause()
        app.state = app.simulation.snapshot()
        key(code, true)
        await process_frame
        app._physics_process(0.016)
        var resume := InputEventKey.new()
        resume.physical_keycode = resume_code
        resume.pressed = true
        app._unhandled_input(resume)
        app.state = app.simulation.snapshot()
        if not check(app.state.state == "Playing", "Resume failed"): return
        var stopped: float = app.state.player
        app._physics_process(0.016)
        if not check(app.simulation.snapshot().player == stopped, "Key pressed while paused moved after resume: %s" % code): return
        key(code, false)
        await process_frame
        app._physics_process(0.016)
        key(code, true)
        await process_frame
        app._physics_process(0.016)
        if not check(app.simulation.snapshot().player != stopped, "Fresh key after paused press did not move: %s" % code): return
        key(code, false)
        await process_frame
        app._physics_process(0.016)
    app.simulation.start()
    app.state = app.simulation.snapshot()
    key(KEY_W, true)
    await process_frame
    var pause := InputEventKey.new()
    pause.physical_keycode = KEY_SPACE
    pause.pressed = true
    app._unhandled_input(pause)
    app.state = app.simulation.snapshot()
    app._unhandled_input(pause)
    app.state = app.simulation.snapshot()
    var before: float = app.state.player
    key(KEY_UP, true)
    await process_frame
    app._physics_process(0.016)
    if not check(app.simulation.snapshot().player < before, "Fresh alias was suppressed"): return
    key(KEY_W, false)
    key(KEY_UP, false)
    await process_frame
    print("GODOT_INPUT_SMOKE_PASS: Ready, held-across-pause and pressed-while-paused, four bindings, three pause keys and four resume/restart controls, fresh press and independent alias")
    quit()
