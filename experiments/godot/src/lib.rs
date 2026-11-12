//! Godot only owns presentation/input. All game rules are shared with other clients.
use godot::classes::{INode, Node};
use godot::prelude::*;
use pong_core::{Game, State};
struct PongExtension;
#[gdextension]
unsafe impl ExtensionLibrary for PongExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct PongSimulation {
    game: Game,
    demo: Option<pong_scenario::Demo>,
    base: Base<Node>,
}
#[godot_api]
impl INode for PongSimulation {
    fn init(base: Base<Node>) -> Self {
        Self {
            game: Game::new(42),
            demo: pong_scenario::Demo::from_args(),
            base,
        }
    }
}
#[godot_api]
impl PongSimulation {
    #[func]
    fn scripted_enabled(&self) -> bool {
        self.demo.is_some()
    }
    #[func]
    fn scripted_advance(&mut self, delta: f64) {
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.game, delta as f32);
        }
    }
    #[func]
    fn scripted_capture_path(&mut self) -> GString {
        self.demo
            .as_mut()
            .and_then(|d| d.capture_path())
            .map(|p| GString::from(p.to_string_lossy().as_ref()))
            .unwrap_or_default()
    }
    #[func]
    fn scripted_finished(&self) -> bool {
        self.demo.as_ref().is_some_and(|d| d.finished())
    }

    #[func]
    fn advance(&mut self, delta: f64, axis: f64) {
        self.game.step(delta as f32, axis as f32);
    }
    #[func]
    fn start(&mut self) {
        self.game.start();
    }
    #[func]
    fn toggle_pause(&mut self) {
        self.game.toggle_pause();
    }
    #[func]
    fn pause(&mut self) {
        self.game.pause();
    }
    #[func]
    fn snapshot(&self) -> VarDictionary {
        let g = &self.game;
        let mut d = VarDictionary::new();
        d.set("player", g.player);
        d.set("cpu", g.cpu);
        d.set("ball_x", g.ball.x);
        d.set("ball_y", g.ball.y);
        d.set("score_player", g.scores[0]);
        d.set("score_cpu", g.scores[1]);
        d.set(
            "state",
            match g.state {
                State::Ready => "Ready",
                State::Playing => "Playing",
                State::Paused => "Paused",
                State::Over => "Over",
            },
        );
        d
    }
}
