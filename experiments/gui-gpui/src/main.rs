use gpui::{prelude::*, *};
use pong_gui_support::Model;
struct Pong {
    model: Model,
    focus: FocusHandle,
    _activation: Subscription,
}
impl Render for Pong {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !window.is_window_active() {
            self.model.blur();
        } else {
            self.model.tick();
        }
        let entity = cx.entity().downgrade();
        if window.is_window_active() {
            window.on_next_frame(move |_, cx| {
                let _ = entity.update(cx, |_, cx| cx.notify());
            });
        }
        let g = &self.model.game;
        let rect = |x: f32, y: f32, w: f32, h: f32, color: u32| {
            div()
                .absolute()
                .left(relative(x / 900.))
                .top(relative(y / 500.))
                .w(relative(w / 900.))
                .h(relative(h / 500.))
                .bg(rgb(color))
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .bg(rgb(0x0c211e))
            .text_color(rgb(0xffffff))
            .track_focus(&self.focus)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if !event.is_held {
                    this.model.key(&event.keystroke.key, true);
                    cx.notify();
                }
            }))
            .on_key_up(cx.listener(|this, event: &KeyUpEvent, _, cx| {
                this.model.key(&event.keystroke.key, false);
                cx.notify();
            }))
            .child("RALLYLIGHT / GPUI")
            .child(self.model.status())
            .child(
                div()
                    .relative()
                    .w_full()
                    .flex_1()
                    .bg(rgb(0x153f37))
                    .child(rect(449., 0., 2., 500., 0x47766b))
                    .child(rect(32., g.player - 45., 12., 90., 0xd9ffae))
                    .child(rect(856., g.cpu - 45., 12., 90., 0xd9ffae))
                    .child(rect(g.ball.x - 8., g.ball.y - 8., 16., 16., 0xffffff).rounded_full()),
            )
            .child("W/S or arrows: move | Enter: start | Space: pause | R: restart")
    }
}
fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.), px(650.)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Pong / GPUI".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    focus.focus(window);
                    Pong {
                        model: Model::default(),
                        focus,
                        _activation: cx.observe_window_activation(
                            window,
                            |this: &mut Pong, window, cx| {
                                if !window.is_window_active() {
                                    this.model.blur();
                                }
                                cx.notify();
                            },
                        ),
                    }
                })
            },
        )
        .expect("open GPUI window");
        cx.activate(true);
    });
}
