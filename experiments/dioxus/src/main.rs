use dioxus::prelude::*;
use pong_web_ui_common::{Session, CSS};
#[cfg(feature = "desktop")]
mod native_demo;
fn main() {
    #[cfg(feature = "desktop")]
    native_demo::configure();
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let mut session = use_signal(|| {
        #[allow(unused_mut)]
        let mut session = Session::default();
        #[cfg(feature = "desktop")]
        if native_demo::enabled() {
            session.enable_demo();
        }
        session
    });
    use_future(move || async move {
        let mut last = web_time::Instant::now();
        loop {
            #[cfg(feature = "desktop")]
            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
            #[cfg(not(feature = "desktop"))]
            gloo_timers::future::TimeoutFuture::new(16).await;
            let now = web_time::Instant::now();
            session.write().tick(now.duration_since(last).as_secs_f64());
            last = now;
            #[cfg(feature = "desktop")]
            native_demo::finish_if_ready(&session.read());
        }
    });
    let svg = session.read().svg();
    rsx! {style { "{CSS}" }
    main { tabindex: "0", autofocus: true,
    onkeydown: move |event: KeyboardEvent| {if pong_web_ui_common::handled_key(&event.key().to_string()){event.prevent_default();}session.write().key(&event.key().to_string(),true,event.is_auto_repeating());},
    onkeyup: move |event: KeyboardEvent| {session.write().key(&event.key().to_string(),false,false);},
    onblur: move |_| session.write().pause(),
    h1 {"Pong • Dioxus"} p {"Rust shared simulation · Dioxus native desktop / WASM web"}
    div { dangerous_inner_html: svg }
    button {onclick:move |_| session.write().start(),"Start / Restart"}
    button {onclick:move |_| session.write().toggle_pause(),"Pause / Resume"}
    button {onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().movement_button(pong_web_ui_common::Direction::Up,true)},onpointerup:move |_|session.write().movement_button(pong_web_ui_common::Direction::Up,false),onpointerleave:move |_|session.write().movement_button(pong_web_ui_common::Direction::Up,false),onpointercancel:move |_|session.write().movement_button(pong_web_ui_common::Direction::Up,false),"▲ Hold"}
    button {onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().movement_button(pong_web_ui_common::Direction::Down,true)},onpointerup:move |_|session.write().movement_button(pong_web_ui_common::Direction::Down,false),onpointerleave:move |_|session.write().movement_button(pong_web_ui_common::Direction::Down,false),onpointercancel:move |_|session.write().movement_button(pong_web_ui_common::Direction::Down,false),"▼ Hold"}
    p {"W/S or ↑/↓ move · Enter restarts · Space pauses. Click the court to focus."}
    a {href:"https://intrepion.github.io/", "Back to Kata Arcade"}
    }
    }
}
