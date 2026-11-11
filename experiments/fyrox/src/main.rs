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
    keyboard::KeyCode,
    plugin::{error::GameResult, Plugin, PluginContext},
};
use pong_core::*;
#[derive(Debug, Visit, Reflect)]
#[reflect(non_cloneable)]
struct App {
    #[visit(skip)]
    #[reflect(hidden)]
    game: Game,
    #[visit(skip)]
    #[reflect(hidden)]
    held: [bool; 4],
    #[visit(skip)]
    #[reflect(hidden)]
    acc: f32,
    nodes: Vec<Handle<UiNode>>,
    title: Handle<UiNode>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            held: [false; 4],
            acc: 0.,
            nodes: Vec::new(),
            title: Handle::NONE,
        }
    }
}
impl Plugin for App {
    fn init(&mut self, _: Option<&str>, context: PluginContext) -> GameResult {
        if context.user_interfaces.iter().next().is_none() {
            context.user_interfaces.add(UserInterface::new(Vector2::new(W, H + 90.0)));
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
        self.acc += context.dt.min(0.1);
        while self.acc >= 1. / 120. {
            self.game.step(
                1. / 120.,
                (self.held[2] || self.held[3]) as u8 as f32
                    - (self.held[0] || self.held[1]) as u8 as f32,
            );
            self.acc -= 1. / 120.;
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
        ui.send(self.title,TextMessage::Text(format!("PONG / FYROX   YOU {} : {} CPU   {:?}\nW/S or arrows: move | Enter: play/resume | Space: pause | R: restart",self.game.scores[0],self.game.scores[1],self.game.state)));
        Ok(())
    }
    fn on_os_event(&mut self, event: &Event<()>, _: PluginContext) -> GameResult {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Focused(false) => {
                    self.game.pause();
                    self.held = [false; 4];
                    self.acc = 0.;
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let key = event.physical_key;
                    let down = event.state == ElementState::Pressed;
                    for (i, k) in [
                        KeyCode::KeyW,
                        KeyCode::ArrowUp,
                        KeyCode::KeyS,
                        KeyCode::ArrowDown,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        if key == k {
                            self.held[i] = down;
                        }
                    }
                    if down && !event.repeat {
                        if key == KeyCode::Enter {
                            if self.game.state == State::Paused {
                                self.game.toggle_pause();
                            } else if self.game.state != State::Playing {
                                self.game.start();
                            }
                        } else if key == KeyCode::KeyR {
                            self.game.start();
                        } else if key == KeyCode::Space {
                            self.game.toggle_pause();
                        } else if key == KeyCode::Escape {
                            self.game.pause();
                        }
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
