use dioxus::prelude::*;
use pong_web_ui_common::{Session, CSS};
#[cfg(feature = "desktop")]
mod native_demo;
fn preserve_activation(event: KeyboardEvent) {
    if pong_web_ui_common::native_activation_key(&event.key().to_string()) {
        event.stop_propagation();
    }
}
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
    main { id: "pong-dioxus-root", tabindex: "0", autofocus: true,
    onkeydown: move |event: KeyboardEvent| {if pong_web_ui_common::handled_key(&event.key().to_string()){event.prevent_default();}session.write().key(&event.key().to_string(),true,event.is_auto_repeating());},
    onkeyup: move |event: KeyboardEvent| {session.write().key(&event.key().to_string(),false,false);},
    onfocusout: move |_| {
        // FocusData has no related target on desktop. Check after the browser's
        // focus transition so moving to an internal button does not pause.
        spawn(async move {
            let next_inside = document::eval(r#"
                await new Promise(resolve => setTimeout(resolve, 0));
                const root = document.getElementById('pong-dioxus-root');
                return Boolean(root && document.hasFocus() && root.contains(document.activeElement));
            "#).join::<bool>().await.unwrap_or(false);
            session.write().focus_left(next_inside);
        });
    },
    h1 {"Pong • Dioxus"} p {"Rust shared simulation · Dioxus native desktop / WASM web"}
    div { dangerous_inner_html: svg }
    button {onkeydown: preserve_activation, onclick:move |_| session.write().start(),"Start / Restart"}
    button {onkeydown: preserve_activation, onclick:move |_| session.write().toggle_pause(),"Pause / Resume"}
    button {onkeydown: move |e:KeyboardEvent| {let key=e.key().to_string(); if pong_web_ui_common::native_activation_key(&key){e.prevent_default();e.stop_propagation();session.write().movement_button_key(pong_web_ui_common::Direction::Up,&key,true,e.is_auto_repeating());}}, onkeyup: move |e:KeyboardEvent| {let key=e.key().to_string(); if pong_web_ui_common::native_activation_key(&key){e.prevent_default();e.stop_propagation();session.write().movement_button_key(pong_web_ui_common::Direction::Up,&key,false,false);}}, onblur: move |_| session.write().movement_button_blur(pong_web_ui_common::Direction::Up), onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),true)},onpointerup:move |e|session.write().movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false),onpointerleave:move |e|session.write().movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false),onpointercancel:move |e|session.write().movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false),"▲ Hold"}
    button {onkeydown: move |e:KeyboardEvent| {let key=e.key().to_string(); if pong_web_ui_common::native_activation_key(&key){e.prevent_default();e.stop_propagation();session.write().movement_button_key(pong_web_ui_common::Direction::Down,&key,true,e.is_auto_repeating());}}, onkeyup: move |e:KeyboardEvent| {let key=e.key().to_string(); if pong_web_ui_common::native_activation_key(&key){e.prevent_default();e.stop_propagation();session.write().movement_button_key(pong_web_ui_common::Direction::Down,&key,false,false);}}, onblur: move |_| session.write().movement_button_blur(pong_web_ui_common::Direction::Down), onpointerdown:move |e:PointerEvent| {e.prevent_default();session.write().movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),true)},onpointerup:move |e|session.write().movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false),onpointerleave:move |e|session.write().movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false),onpointercancel:move |e|session.write().movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false),"▼ Hold"}
    p {"W/S or ↑/↓ move · Enter restarts · Space pauses · Hold Space or Enter on movement buttons. Click the court to focus."}
    a {onkeydown: preserve_activation, href:"https://intrepion.github.io/", "Back to Kata Arcade"}
    }
    }
}
