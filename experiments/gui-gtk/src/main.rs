use gtk::{glib, prelude::*, Application, ApplicationWindow, DrawingArea, EventControllerKey};
use pong_gui_support::Model;
use std::{cell::RefCell, rc::Rc};
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.intrepion.PongGtk")
        .build();
    app.connect_activate(|app| {
        let model = Rc::new(RefCell::new(Model::default()));
        let area = DrawingArea::builder()
            .content_width(960)
            .content_height(650)
            .focusable(true)
            .build();
        let m = model.clone();
        area.set_draw_func(move |_, cr, w, h| {
            let m = m.borrow();
            let g = &m.game;
            cr.set_source_rgb(0.047, 0.129, 0.118);
            let _ = cr.paint();
            cr.set_source_rgb(0.85, 1., 0.68);
            cr.set_font_size(22.);
            cr.move_to(20., 30.);
            let _ = cr.show_text("RALLYLIGHT / GTK");
            cr.move_to(20., 60.);
            let _ = cr.show_text(&m.status());
            let s = (w as f64 / 900.).min(((h - 130) as f64 / 500.).max(0.01));
            let _ = cr.save();
            cr.translate((w as f64 - 900. * s) / 2., 80.);
            cr.scale(s, s);
            cr.set_source_rgb(0.082, 0.247, 0.216);
            cr.rectangle(0., 0., 900., 500.);
            let _ = cr.fill();
            cr.set_source_rgb(0.28, 0.46, 0.42);
            cr.rectangle(449., 0., 2., 500.);
            let _ = cr.fill();
            cr.set_source_rgb(0.85, 1., 0.68);
            for (x, y) in [(32., g.player), (856., g.cpu)] {
                cr.rectangle(x, y as f64 - 45., 12., 90.);
            }
            let _ = cr.fill();
            cr.set_source_rgb(1., 1., 1.);
            cr.arc(
                g.ball.x as f64,
                g.ball.y as f64,
                8.,
                0.,
                std::f64::consts::TAU,
            );
            let _ = cr.fill();
            let _ = cr.restore();
            cr.set_font_size(14.);
            cr.move_to(20., h as f64 - 18.);
            let _ = cr.show_text("W/S or arrows: move | Enter: start | Space: pause | R: restart");
        });
        let keys = EventControllerKey::new();
        let m = model.clone();
        keys.connect_key_pressed(move |_, key, _, _| {
            m.borrow_mut().key(&key.name().unwrap_or_default(), true);
            glib::Propagation::Stop
        });
        let m = model.clone();
        keys.connect_key_released(move |_, key, _, _| {
            m.borrow_mut().key(&key.name().unwrap_or_default(), false)
        });
        area.add_controller(keys);
        let win = ApplicationWindow::builder()
            .application(app)
            .title("Pong / GTK")
            .default_width(960)
            .default_height(650)
            .child(&area)
            .build();
        let m = model.clone();
        win.connect_is_active_notify(move |w| {
            if !w.is_active() {
                m.borrow_mut().blur();
            }
        });
        area.add_tick_callback(move |area, _| {
            model.borrow_mut().tick();
            area.queue_draw();
            glib::ControlFlow::Continue
        });
        win.present();
        area.grab_focus();
    });
    app.run()
}
