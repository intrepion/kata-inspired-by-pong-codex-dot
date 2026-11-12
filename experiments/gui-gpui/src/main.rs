use gpui::{prelude::*, *};
use pong_gui_support::Model;
fn court_transform(width: f32, height: f32) -> (f32, f32, f32, f32) {
    let scale = (width / 900.).min(height / 500.).max(0.);
    (
        scale,
        scale,
        (width - 900. * scale) / 2.,
        (height - 500. * scale) / 2.,
    )
}
struct Pong {
    model: Model,
    demo: Option<pong_scenario::Demo>,
    previous: std::time::Instant,
    focus: FocusHandle,
    _activation: Subscription,
}
impl Render for Pong {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.previous).as_secs_f32();
        self.previous = now;
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.model.game, elapsed);
        } else if !window.is_window_active() {
            self.model.blur();
        } else {
            self.model.tick();
        }
        let entity = cx.entity().downgrade();
        if window.is_window_active() || self.demo.is_some() {
            window.on_next_frame(move |_, cx| {
                let _ = entity.update(cx, |this, cx| {
                    if this.demo.as_ref().is_some_and(|d| d.finished()) {
                        cx.quit();
                    } else {
                        cx.notify();
                    }
                });
            });
        }
        let game = self.model.game.clone();
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
                if this.demo.is_none() && !event.is_held {
                    this.model.key(&event.keystroke.key, true);
                    cx.notify();
                }
            }))
            .on_key_up(cx.listener(|this, event: &KeyUpEvent, _, cx| {
                if this.demo.is_none() {
                    this.model.key(&event.keystroke.key, false);
                }
                cx.notify();
            }))
            .child("RALLYLIGHT / GPUI")
            .child(self.model.status())
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        let (sx, _, ox, oy) =
                            court_transform(bounds.size.width.into(), bounds.size.height.into());
                        if sx <= 0. {
                            return;
                        }
                        let rect = |x: f32, y: f32, w: f32, h: f32| {
                            Bounds::new(
                                bounds.origin + point(px(ox + x * sx), px(oy + y * sx)),
                                size(px(w * sx), px(h * sx)),
                            )
                        };
                        window.paint_quad(fill(rect(0., 0., 900., 500.), rgb(0x153f37)));
                        window.paint_quad(fill(rect(449., 0., 2., 500.), rgb(0x47766b)));
                        window.paint_quad(fill(
                            rect(32., game.player - 45., 12., 90.),
                            rgb(0xd9ffae),
                        ));
                        window
                            .paint_quad(fill(rect(856., game.cpu - 45., 12., 90.), rgb(0xd9ffae)));
                        window.paint_quad(quad(
                            rect(game.ball.x - 8., game.ball.y - 8., 16., 16.),
                            px(8. * sx),
                            rgb(0xffffff),
                            px(0.),
                            transparent_black(),
                            Default::default(),
                        ));
                    },
                )
                .w_full()
                .flex_1(),
            )
            .child("W/S or arrows: move | Enter: start | Space: pause | R: restart")
    }
}
fn main() {
    if std::env::args().any(|a| a == "--scripted-captures") {
        eprintln!("GPUI 0.2.2 has no supported app-owned pixel export here; use --scripted-test for state-only replay. No screenshot is produced.");
        std::process::exit(2);
    }
    let demo = pong_scenario::Demo::from_args();
    Application::new().run(move |cx: &mut App| {
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
            move |window, cx| {
                cx.new(|cx| {
                    let focus = cx.focus_handle();
                    focus.focus(window);
                    Pong {
                        model: Model::default(),
                        demo,
                        previous: std::time::Instant::now(),
                        focus,
                        _activation: cx.observe_window_activation(
                            window,
                            |this: &mut Pong, window, cx| {
                                if this.demo.is_none() && !window.is_window_active() {
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

#[cfg(test)]
mod court_tests {
    use super::court_transform;
    #[test]
    fn court_keeps_world_aspect_and_is_centered_on_resize() {
        for (width, height) in [
            (900., 500.),
            (1000., 300.),
            (300., 1000.),
            (0., 500.),
            (900., 0.),
        ] {
            let (sx, sy, ox, oy) = court_transform(width, height);
            assert_eq!(sx, sy, "world pixels must have the same scale on both axes");
            assert!(sx >= 0. && ox >= 0. && oy >= 0.);
            assert!((900. * sx + 2. * ox - width).abs() < 0.001);
            assert!((500. * sy + 2. * oy - height).abs() < 0.001);
        }
    }
}
