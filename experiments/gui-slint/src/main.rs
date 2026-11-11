use pong_gui_support::*;
use std::{cell::RefCell, rc::Rc, time::Duration};
slint::slint! {
import { AboutSlint, VerticalBox } from "std-widgets.slint";
export component PongWindow inherits Window {
 title: "Pong / Slint"; width: 960px; height: 650px; background:#0c211e;
 in property<float> player; in property<float> cpu; in property<float> bx; in property<float> by;
 in property<string> status;
 callback key(string,bool); callback blur();
 VerticalBox {
 Text {text:"RALLYLIGHT / SLINT";color:#d9ffae;font-size:24px;}
 Text {text:root.status;color:white;}
 focus := FocusScope {width:100%; vertical-stretch:1;
  key-pressed(event)=>{root.key(event.text,true);accept}
  key-released(event)=>{root.key(event.text,false);accept}
  changed has-focus=>{if !self.has-focus {root.blur();}}
  Rectangle {width:min(parent.width,parent.height*1.8);height:self.width/1.8;x:(parent.width - self.width)/2;y:(parent.height - self.height)/2;background:#153f37;
   Rectangle{x:parent.width*0.5;width:2px;height:parent.height;background:#47766b;}
   Rectangle{x:parent.width*32/900;y:parent.height*(root.player - 45)/500;width:parent.width*12/900;height:parent.height*90/500;background:#d9ffae;}
   Rectangle{x:parent.width*856/900;y:parent.height*(root.cpu - 45)/500;width:parent.width*12/900;height:parent.height*90/500;background:#d9ffae;}
   Rectangle{x:parent.width*(root.bx - 8)/900;y:parent.height*(root.by - 8)/500;width:parent.width*16/900;height:parent.height*16/500;border-radius:8px;background:white;}
  }
 }
 Text {text:"W/S or arrows: move | Enter: start | Space: pause | R: restart";color:white;}
 AboutSlint {height:80px;vertical-stretch:0;}
 }
 init=>{focus.focus();}
}
}
fn main() -> Result<(), slint::PlatformError> {
    let ui = PongWindow::new()?;
    let model = Rc::new(RefCell::new(Model::default()));
    let m = model.clone();
    ui.on_key(move |key, down| {
        let k = match key.as_str() {
            "\u{f700}" => "Up",
            "\u{f701}" => "Down",
            "\n" | "\r" => "Enter",
            "\u{1b}" => "Escape",
            s => s,
        };
        m.borrow_mut().key(k, down);
    });
    let m = model.clone();
    ui.on_blur(move || m.borrow_mut().blur());
    let weak = ui.as_weak();
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(8),
        move || {
            let Some(ui) = weak.upgrade() else { return };
            let mut m = model.borrow_mut();
            m.tick();
            ui.set_player(m.game.player);
            ui.set_cpu(m.game.cpu);
            ui.set_bx(m.game.ball.x);
            ui.set_by(m.game.ball.y);
            ui.set_status(m.status().into());
        },
    );
    ui.run()
}
