extends Node2D
var simulation: PongSimulation
var state: Dictionary
var font: Font
var axis := 0.0

func _ready() -> void:
    simulation = PongSimulation.new()
    add_child(simulation)
    font = ThemeDB.fallback_font
    state = simulation.snapshot()
    get_window().focus_exited.connect(func(): simulation.pause())

func _physics_process(delta: float) -> void:
    axis = float(Input.is_physical_key_pressed(KEY_S) or Input.is_physical_key_pressed(KEY_DOWN)) - float(Input.is_physical_key_pressed(KEY_W) or Input.is_physical_key_pressed(KEY_UP))
    simulation.advance(delta, axis)
    state = simulation.snapshot()
    queue_redraw()

func _unhandled_input(event: InputEvent) -> void:
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
