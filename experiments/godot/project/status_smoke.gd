extends SceneTree

func _initialize() -> void:
    var app = load("res://main.gd").new()
    var cases := [
        [{"state": "Ready"}, "Space to start"],
        [{"state": "Playing"}, ""],
        [{"state": "Paused"}, "Paused - Space to resume"],
        [{"state": "Over", "score_player": 7, "score_cpu": 3}, "YOU WIN - Space to restart"],
        [{"state": "Over", "score_player": 2, "score_cpu": 7}, "CPU WINS - Space to restart"],
        [{"state": "Over", "score_player": 7, "score_cpu": 7}, "Match over - Space to restart"],
    ]
    for test in cases:
        var actual: String = app.status_message(test[0])
        if actual != test[1]:
            push_error("Status message mismatch: %s != %s" % [actual, test[1]])
            app.free()
            quit(1)
            return
    app.free()
    print("status smoke: 6 cases passed")
    quit(0)
