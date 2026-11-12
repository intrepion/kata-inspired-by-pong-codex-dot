extends Node2D
var simulation: PongSimulation
var state: Dictionary
var font: Font
var axis := 0.0
var capture_pending := false

func _ready() -> void:
    simulation = PongSimulation.new()
    add_child(simulation)
    font = ThemeDB.fallback_font
    state = simulation.snapshot()
    get_window().focus_exited.connect(func():
        if not simulation.scripted_enabled(): simulation.pause()
    )

func _physics_process(delta: float) -> void:
    if simulation.scripted_enabled():
        if capture_pending: return
        simulation.scripted_advance(delta)
        state = simulation.snapshot()
        queue_redraw()
        var path: String = simulation.scripted_capture_path()
        if not path.is_empty() and DisplayServer.get_name() == "headless":
            push_error("Viewport PNG capture requires a graphics renderer, not --headless")
            get_tree().quit(1)
            return
        if path.is_empty() and simulation.scripted_finished() and DisplayServer.get_name() == "headless":
            get_tree().quit()
            return
        if not path.is_empty() or simulation.scripted_finished():
            capture_pending = true
            await RenderingServer.frame_post_draw
            if not path.is_empty():
                var image := get_viewport().get_texture().get_image()
                if image == null or image.is_empty():
                    push_error("Godot own viewport capture unavailable")
                    get_tree().quit(1)
                    return
                var result := image.save_png(path)
                if result != OK:
                    push_error("Godot own viewport PNG export failed: %s" % result)
                    get_tree().quit(1)
                    return
            capture_pending = false
            if simulation.scripted_finished(): get_tree().quit()
        return

    axis = float(Input.is_physical_key_pressed(KEY_S) or Input.is_physical_key_pressed(KEY_DOWN)) - float(Input.is_physical_key_pressed(KEY_W) or Input.is_physical_key_pressed(KEY_UP))
    simulation.advance(delta, axis)
    state = simulation.snapshot()
    queue_redraw()

func _unhandled_input(event: InputEvent) -> void:
    if simulation.scripted_enabled(): return
    if event is InputEventKey and event.pressed and not event.echo:
        match event.physical_keycode:
            KEY_SPACE, KEY_ENTER:
                if state.state == "Ready" or state.state == "Over": simulation.start()
                else: simulation.toggle_pause()
            KEY_P: simulation.toggle_pause()
            KEY_R: simulation.start()
            KEY_ESCAPE: get_tree().quit()

func _draw() -> void:
    if state == null or state.is_empty(): return
    draw_rect(Rect2(0, 0, 900, 500), Color("101a2a"))
    for y in range(0, 500, 24): draw_rect(Rect2(448, y, 4, 12), Color("354358"))
    draw_rect(Rect2(32, state.player - 45, 12, 90), Color("56dbbc"))
    draw_rect(Rect2(856, state.cpu - 45, 12, 90), Color("ffbd69"))
    draw_circle(Vector2(state.ball_x, state.ball_y), 8, Color.WHITE)
    draw_string(font, Vector2(370, 48), "%d     %d" % [state.score_player, state.score_cpu], HORIZONTAL_ALIGNMENT_LEFT, -1, 32)
    draw_string(font, Vector2(28, 533), "GODOT + RUST  |  First to 7  |  " + state.state, HORIZONTAL_ALIGNMENT_LEFT, -1, 20)
    draw_string(font, Vector2(28, 563), "W/S or arrows: move   Space: start/pause   R: restart   Esc: quit", HORIZONTAL_ALIGNMENT_LEFT, -1, 17)
    if state.state != "Playing":
        var message: String = "Space to start" if state.state == "Ready" else ("Paused - Space to resume" if state.state == "Paused" else "Match over - Space to restart")
        draw_string(font, Vector2(280, 260), message, HORIZONTAL_ALIGNMENT_LEFT, -1, 24)
