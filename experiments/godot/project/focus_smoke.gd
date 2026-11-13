extends SceneTree

func _initialize() -> void:
    call_deferred("run")

func key(code: int) -> InputEventKey:
    var event := InputEventKey.new()
    event.physical_keycode = code
    event.pressed = true
    return event

func run() -> void:
    var app = load("res://main.tscn").instantiate()
    root.add_child(app)
    app.set_physics_process(false)
    app.simulation.start()
    app.simulation.pause()
    app.state = app.simulation.snapshot()
    app.get_window().focus_exited.emit()
    if app.input_focused:
        push_error("Focus exit did not gate input")
        quit(1)
        return
    app.suppressed_keys[KEY_W] = true
    var release := key(KEY_W)
    release.pressed = false
    app._unhandled_input(release)
    if app.suppressed_keys.has(KEY_W):
        push_error("Unfocused release did not clear suppression")
        quit(1)
        return
    for code in [KEY_SPACE, KEY_ENTER, KEY_P, KEY_R]:
        app._unhandled_input(key(code))
        if app.simulation.snapshot().state != "Paused":
            push_error("Unfocused action changed state for key %d" % code)
            quit(1)
            return
    app.get_window().focus_entered.emit()
    if not app.input_focused:
        push_error("Focus entry did not restore input")
        quit(1)
        return
    app._unhandled_input(key(KEY_SPACE))
    if app.simulation.snapshot().state != "Playing":
        push_error("Focused resume failed")
        quit(1)
        return
    print("focus action smoke: passed")
    quit(0)
