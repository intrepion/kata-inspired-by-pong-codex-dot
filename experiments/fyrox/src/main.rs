use fyrox::{
    core::{
        algebra::Vector2, color::Color, pool::Handle, reflect::prelude::*, visitor::prelude::*,
    },
    engine::executor::Executor,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    gui::{
        border::BorderBuilder,
        brush::Brush,
        text::{TextBuilder, TextMessage},
        widget::{WidgetBuilder, WidgetMessage},
        Thickness, UiNode, UserInterface,
    },
    keyboard::{KeyCode, PhysicalKey},
    plugin::{error::GameResult, Plugin, PluginContext},
};
use pong_core::*;
#[derive(Visit, Reflect)]
#[reflect(non_cloneable)]
struct App {
    #[visit(skip)]
    #[reflect(hidden)]
    game: Game,
    #[visit(skip)]
    #[reflect(hidden)]
    focused: bool,
    #[visit(skip)]
    #[reflect(hidden)]
    benchmarking: bool,
    #[visit(skip)]
    #[reflect(hidden)]
    benchmark: Option<pong_benchmark::Frames>,
    #[visit(skip)]
    #[reflect(hidden)]
    benchmark_done: bool,
    #[visit(skip)]
    #[reflect(hidden)]
    scenario: pong_scenario::Scenario,
    #[visit(skip)]
    #[reflect(hidden)]
    demo: Option<pong_scenario::Demo>,
    #[visit(skip)]
    #[reflect(hidden)]
    held: [bool; 4],
    #[visit(skip)]
    #[reflect(hidden)]
    suppressed: [bool; 4],
    #[visit(skip)]
    #[reflect(hidden)]
    acc: f32,
    nodes: Vec<Handle<UiNode>>,
    title: Handle<UiNode>,
}
impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PongApp").finish_non_exhaustive()
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            focused: true,
            benchmarking: benchmark_args(&std::env::args().collect::<Vec<_>>()),
            benchmark: None,
            benchmark_done: false,
            scenario: pong_scenario::Scenario::default(),
            demo: pong_scenario::Demo::from_args(),
            held: [false; 4],
            suppressed: [false; 4],
            acc: 0.,
            nodes: Vec::new(),
            title: Handle::NONE,
        }
    }
}
impl App {
    fn movement_axis(&self) -> f32 {
        let active = std::array::from_fn::<_, 4, _>(|i| self.held[i] && !self.suppressed[i]);
        (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
    }
    fn focused_key_input(&mut self, focused: bool, key: KeyCode, down: bool, repeat: bool) {
        if focused { self.key_input(key, down, repeat); }
    }
    fn key_input(&mut self, key: KeyCode, down: bool, repeat: bool) {
        for (i, k) in [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::KeyS, KeyCode::ArrowDown].into_iter().enumerate() {
            if key == k {
                self.held[i] = down;
                if !down { self.suppressed[i] = false; }
                else if self.game.state != State::Playing { self.suppressed[i] = true; }
            }
        }
        if down && !repeat {
            if key == KeyCode::KeyR || (self.game.state == State::Over && key == KeyCode::Enter) {
                self.suppressed = self.held;
            }
            if matches!(key, KeyCode::Space | KeyCode::Escape) && self.game.state == State::Playing {
                self.suppressed = self.held;
            }
            if key == KeyCode::Enter {
                if self.game.state == State::Paused { self.game.toggle_pause(); }
                else if self.game.state != State::Playing { self.game.start(); }
            } else if key == KeyCode::KeyR { self.game.start(); }
            else if key == KeyCode::Space { self.game.toggle_pause(); }
            else if key == KeyCode::Escape { self.game.pause(); }
        }
    }
}
impl Plugin for App {
    fn init(&mut self, _: Option<&str>, context: PluginContext) -> GameResult {
        if context.user_interfaces.iter().next().is_none() {
            context
                .user_interfaces
                .add(UserInterface::new(Vector2::new(W, H + 90.0)));
        }
        let ui = context.user_interfaces.first_mut();
        let ctx = &mut ui.build_ctx();
        // Court, left/right paddle, ball, center net. Real Fyrox UI nodes, no webview.
        for color in [
            Color::opaque(21, 63, 55),
            Color::opaque(217, 255, 174),
            Color::opaque(217, 255, 174),
            Color::WHITE,
            Color::opaque(70, 110, 90),
        ] {
            self.nodes.push(
                BorderBuilder::new(
                    WidgetBuilder::new().with_background(Brush::Solid(color).into()),
                )
                .with_stroke_thickness(Thickness::uniform(0.).into())
                .build(ctx)
                .to_base(),
            );
        }
        self.title = TextBuilder::new(WidgetBuilder::new().with_height(65.))
            .with_text("Pong / Fyrox")
            .build(ctx)
            .to_base();
        // Builders already attach nodes to the root canvas. Avoid a zero-sized
        // intermediate canvas, which clips absolutely positioned children.
        Ok(())
    }
    fn update(&mut self, context: &mut PluginContext) -> GameResult {
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.game, context.dt);
        } else if self.benchmarking {
            self.acc += context.dt.min(0.1);
            while self.acc >= pong_scenario::STEP {
                if self.scenario.finished() {
                    self.scenario = pong_scenario::Scenario::default();
                    self.game = Game::new(pong_scenario::SEED);
                }
                self.scenario.advance(&mut self.game);
                self.acc -= pong_scenario::STEP;
            }
        } else {
            self.acc += context.dt.min(0.1);
            while self.acc >= 1. / 120. {
                self.game.step(
                    1. / 120.,
                    self.movement_axis(),
                );
                self.acc -= 1. / 120.;
            }
        }
        let ui = context.user_interfaces.first_mut();
        let size = ui.screen_size();
        let s = (size.x / W).min(size.y / (H + 90.)).max(0.001);
        let ox = (size.x - W * s) / 2.;
        let oy = (size.y - (H + 90.) * s) / 2. + 90. * s;
        let rects = [
            (0., 0., W, H),
            (32., self.game.player - PADDLE_H / 2., PADDLE_W, PADDLE_H),
            (W - 44., self.game.cpu - PADDLE_H / 2., PADDLE_W, PADDLE_H),
            (
                self.game.ball.x - BALL,
                self.game.ball.y - BALL,
                BALL * 2.,
                BALL * 2.,
            ),
            (449., 0., 2., H),
        ];
        for (node, (x, y, w, h)) in self.nodes.iter().zip(rects) {
            ui.send(
                *node,
                WidgetMessage::DesiredPosition(Vector2::new(ox + x * s, oy + y * s)),
            );
            ui.send(*node, WidgetMessage::Width(w * s));
            ui.send(*node, WidgetMessage::Height(h * s));
        }
        ui.send(
            self.title,
            WidgetMessage::DesiredPosition(Vector2::new(ox, oy - 75. * s)),
        );
        ui.send(self.title, WidgetMessage::Width(W * s));
        ui.send(self.title, WidgetMessage::Height(65. * s));
        ui.send(self.title, TextMessage::FontSize((18. * s).into()));
        ui.send(self.title,TextMessage::Text(format!("PONG / FYROX   YOU {} : {} CPU   {:?}\nW/S or arrows: move | Enter: play/resume | Space: pause | R: restart",self.game.scores[0],self.game.scores[1],self.game.state)));
        Ok(())
    }
    fn before_rendering(&mut self, context: PluginContext) -> GameResult {
        if self.benchmarking && !self.benchmark_done {
            let size = context
                .graphics_context
                .as_initialized_ref()
                .window
                .inner_size();
            let frames = self
                .benchmark
                .get_or_insert_with(|| pong_benchmark::Frames::new(size.width, size.height));
            if frames.frame(size.width, size.height) {
                self.benchmark_done = true;
                println!("PONG_BENCHMARK {}", frames.report());
                context.loop_controller.exit();
            }
        }
        if let Some(path) = self.demo.as_mut().and_then(|d| d.capture_path()) {
            use fyrox::graphics::framebuffer::ReadTarget;
            use fyrox::renderer::ui_renderer::UiRenderInfo;
            use fyrox::resource::texture::{TextureResource, TextureResourceExtension};
            let ui = context.user_interfaces.first();
            let size = ui.screen_size();
            let (width, height) = (size.x as u32, size.y as u32);
            let target = TextureResource::new_render_target(width, height);
            let renderer = &mut context.graphics_context.as_initialized_mut().renderer;
            renderer
                .render_ui(UiRenderInfo {
                    ui,
                    render_target: Some(target.clone()),
                    clear_color: Color::BLACK,
                    resource_manager: context.resource_manager,
                })
                .expect("render application UI capture target");
            let bytes = renderer.ui_frame_buffers[&target.key()]
                .read_pixels(ReadTarget::Color(0))
                .expect("read application UI target");
            assert_eq!(bytes.len(), width as usize * height as usize * 4);
            let rgb: Vec<u32> = bytes
                .chunks_exact(width as usize * 4)
                .rev()
                .flat_map(|row| {
                    row.as_chunks::<4>()
                        .0
                        .iter()
                        .map(|p| (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]))
                })
                .collect();
            pong_scenario::Demo::save_rgb(&path, &rgb, width, height);
        }
        if self.demo.as_ref().is_some_and(|d| d.finished()) {
            context.loop_controller.exit();
        }
        Ok(())
    }
    fn on_os_event(&mut self, event: &Event<()>, _: PluginContext) -> GameResult {
        if self.demo.is_some() || self.benchmarking {
            return Ok(());
        }
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Focused(focused) => {
                    self.focused = *focused;
                    if !focused {
                        self.game.pause();
                        self.held = [false; 4];
                        self.suppressed = [false; 4];
                        self.acc = 0.;
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(key) = event.physical_key {
                        self.focused_key_input(self.focused, key, event.state == ElementState::Pressed, event.repeat);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
fn main() {
    let mut executor = Executor::new(Some(EventLoop::new().expect("create Fyrox event loop")));
    executor.set_desired_update_rate(120.);
    executor.add_plugin(App::default());
    executor.run();
}

fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are exclusive"
    );
    enabled
}
#[cfg(test)]
mod benchmark_tests {
    use super::*;
    #[test]
    fn opt_in_only() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic]
    fn reject_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut app = App::default(); app.game.start(); app.game.scores = [3, 2]; app.game.pause();
        app.focused_key_input(false, KeyCode::Space, true, false);
        assert_eq!(app.game.state, State::Paused);
        app.focused_key_input(false, KeyCode::KeyR, true, false);
        assert_eq!(app.game.state, State::Paused); assert_eq!(app.game.scores, [3, 2]);
        app.focused_key_input(true, KeyCode::Space, true, false);
        assert_eq!(app.game.state, State::Playing);
    }
    #[test]
    fn active_restart_blocks_held_direction_until_release() {
        let mut app = App::default(); app.game.start();
        app.key_input(KeyCode::KeyW, true, false);
        assert_eq!(app.movement_axis(), -1.);
        app.key_input(KeyCode::KeyR, true, false);
        assert_eq!(app.movement_axis(), 0.);
        app.key_input(KeyCode::KeyW, true, true);
        assert_eq!(app.movement_axis(), 0.);
        app.key_input(KeyCode::KeyW, false, false);
        app.key_input(KeyCode::KeyW, true, false);
        assert_eq!(app.movement_axis(), -1.);
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        for key in [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::KeyS, KeyCode::ArrowDown] {
            for restart in [KeyCode::Enter, KeyCode::KeyR] {
                let mut app=App::default();app.game.start();app.key_input(key,true,false);
                app.game.scores=[6,2];app.game.serve=0.;app.game.ball=Ball{x:W+BALL-1.,y:H/2.,vx:350.,vy:0.};
                app.game.step(0.02,0.);assert_eq!(app.game.state,State::Over);
                app.key_input(restart,true,false);assert_eq!(app.game.state,State::Playing);
                assert_eq!(app.movement_axis(),0.,"{key:?} {restart:?}");
                app.key_input(key,true,true);assert_eq!(app.movement_axis(),0.);
                app.key_input(key,false,false);app.key_input(key,true,false);assert_ne!(app.movement_axis(),0.);
            }
        }
    }

    #[test]
    fn nonplaying_presses_stay_suppressed_until_release() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::KeyS, KeyCode::ArrowDown] {
                let mut app=App::default(); app.game.state=state;
                app.key_input(key,true,false); app.key_input(KeyCode::Enter,true,false);
                assert_eq!(app.game.state,State::Playing);
                assert_eq!(app.movement_axis(),0.,"{state:?} {key:?}");
                app.key_input(key,true,true); assert_eq!(app.movement_axis(),0.);
                app.key_input(key,false,false); app.key_input(key,true,false); assert_ne!(app.movement_axis(),0.);
            }
        }
    }

    #[test]
    fn held_movement_does_not_resume_after_pause_until_release() {
        for movement in [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::KeyS, KeyCode::ArrowDown] {
            for pause in [KeyCode::Space, KeyCode::Escape] {
                let mut app = App::default();
                app.game.start();
                app.key_input(movement, true, false);
                assert_ne!(app.movement_axis(), 0.);
                app.key_input(pause, true, false);
                app.key_input(KeyCode::Enter, true, false);
                assert_eq!(app.game.state, State::Playing);
                app.key_input(movement, true, true);
                assert_eq!(app.movement_axis(), 0., "{movement:?} after {pause:?}");
                app.key_input(movement, false, false);
                app.key_input(movement, true, false);
                assert_ne!(app.movement_axis(), 0.);
            }
        }
    }
    #[test]
    fn fresh_alias_and_opposite_bindings_remain_independent() {
        let mut app = App::default();
        app.game.start();
        app.key_input(KeyCode::KeyW, true, false);
        app.key_input(KeyCode::Space, true, false);
        app.key_input(KeyCode::Enter, true, false);
        app.key_input(KeyCode::ArrowUp, true, false);
        assert_eq!(app.movement_axis(), -1.);
        app.key_input(KeyCode::KeyS, true, false);
        assert_eq!(app.movement_axis(), 0.);
        app.key_input(KeyCode::ArrowUp, false, false);
        assert_eq!(app.movement_axis(), 1.);
        app.key_input(KeyCode::KeyW, false, false);
        app.key_input(KeyCode::KeyW, true, false);
        assert_eq!(app.movement_axis(), 0.);
    }

}
