use dioxus::prelude::*;
use pong_web_ui_common::{Session, CSS};
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let mut session = use_signal(Session::default);
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
    button {onclick:move |_| session.write().game.toggle_pause(),"Pause / Resume"}
    button {onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().key("ArrowUp",true,false)},onpointerup:move |_|session.write().key("ArrowUp",false,false),onpointerleave:move |_|session.write().key("ArrowUp",false,false),onpointercancel:move |_|session.write().key("ArrowUp",false,false),"▲ Hold"}
    button {onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().key("ArrowDown",true,false)},onpointerup:move |_|session.write().key("ArrowDown",false,false),onpointerleave:move |_|session.write().key("ArrowDown",false,false),onpointercancel:move |_|session.write().key("ArrowDown",false,false),"▼ Hold"}
    p {"W/S or ↑/↓ move · Enter restarts · Space pauses. Click the court to focus."}
    a {href:"https://intrepion.github.io/", "Back to Kata Arcade"}
    }
    }
}
