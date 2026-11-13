use leptos::prelude::*;
use pong_web_ui_common::{Session, CSS};
use wasm_bindgen::JsCast;
fn preserve_activation(ev: web_sys::KeyboardEvent) {
    if pong_web_ui_common::native_activation_key(&ev.key()) {
        ev.stop_propagation();
    }
}
fn main() {
    mount_to_body(App);
}
#[component]
fn App() -> impl IntoView {
    let session = RwSignal::new(Session::default());
    let game_root = NodeRef::<leptos::html::Main>::new();
    let mut last = web_time::Instant::now();
    let interval = gloo_timers::callback::Interval::new(16, move || {
        let now = web_time::Instant::now();
        session.update(|s| s.tick(now.duration_since(last).as_secs_f64()));
        last = now;
    });
    let interval = send_wrapper::SendWrapper::new(interval);
    on_cleanup(move || drop(interval));
    view! {<style>{CSS}</style><main node_ref=game_root tabindex="0" autofocus=true
    on:keydown=move |ev:web_sys::KeyboardEvent|{if pong_web_ui_common::handled_key(&ev.key()){ev.prevent_default();}session.update(|s|s.key(&ev.key(),true,ev.repeat()));}
    on:keyup=move |ev:web_sys::KeyboardEvent|session.update(|s|s.key(&ev.key(),false,false))
    on:focusout=move |ev:web_sys::FocusEvent| {
        let next_inside = game_root.get_untracked()
            .zip(ev.related_target().and_then(|target| target.dyn_into::<web_sys::Node>().ok()))
            .is_some_and(|(root, next)| root.contains(Some(&next)));
        session.update(|s|s.focus_left(next_inside));
    }>
    <h1>"Pong • Leptos"</h1><p>"Rust shared simulation · Leptos CSR / WASM"</p>
    <div inner_html=move || session.with(|s|s.svg())></div>
    <button on:keydown=preserve_activation on:click=move |_|session.update(|s|s.start())>"Start / Restart"</button>
    <button on:keydown=preserve_activation on:click=move |_|session.update(|s|s.toggle_pause())>"Pause / Resume"</button>
    <button on:keydown=move |e:web_sys::KeyboardEvent|{if pong_web_ui_common::native_activation_key(&e.key()){e.prevent_default();e.stop_propagation();session.update(|s|s.movement_button_key(pong_web_ui_common::Direction::Up,&e.key(),true,e.repeat()));}} on:keyup=move |e:web_sys::KeyboardEvent|{if pong_web_ui_common::native_activation_key(&e.key()){e.prevent_default();e.stop_propagation();session.update(|s|s.movement_button_key(pong_web_ui_common::Direction::Up,&e.key(),false,false));}} on:blur=move |_|session.update(|s|s.movement_button_blur(pong_web_ui_common::Direction::Up)) on:click=move |e:web_sys::MouseEvent|{if e.detail()==0{session.update(|s|s.movement_button_activate(pong_web_ui_common::Direction::Up));}} on:pointerdown=move |e:web_sys::PointerEvent|{e.prevent_default();session.update(|s|s.movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),true))} on:pointerup=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false)) on:pointerleave=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false)) on:pointercancel=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Up,e.pointer_id(),false))>"▲ Hold"</button>
    <button on:keydown=move |e:web_sys::KeyboardEvent|{if pong_web_ui_common::native_activation_key(&e.key()){e.prevent_default();e.stop_propagation();session.update(|s|s.movement_button_key(pong_web_ui_common::Direction::Down,&e.key(),true,e.repeat()));}} on:keyup=move |e:web_sys::KeyboardEvent|{if pong_web_ui_common::native_activation_key(&e.key()){e.prevent_default();e.stop_propagation();session.update(|s|s.movement_button_key(pong_web_ui_common::Direction::Down,&e.key(),false,false));}} on:blur=move |_|session.update(|s|s.movement_button_blur(pong_web_ui_common::Direction::Down)) on:click=move |e:web_sys::MouseEvent|{if e.detail()==0{session.update(|s|s.movement_button_activate(pong_web_ui_common::Direction::Down));}} on:pointerdown=move |e:web_sys::PointerEvent|{e.prevent_default();session.update(|s|s.movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),true))} on:pointerup=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false)) on:pointerleave=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false)) on:pointercancel=move |e|session.update(|s|s.movement_button(pong_web_ui_common::Direction::Down,e.pointer_id(),false))>"▼ Hold"</button>
    <p>"W/S or ↑/↓ move · Enter restarts · Space pauses · Hold Space or Enter on movement buttons. Click the court to focus."</p>
    <a on:keydown=preserve_activation href="https://intrepion.github.io/">"Back to Kata Arcade"</a>
    </main>}
}
