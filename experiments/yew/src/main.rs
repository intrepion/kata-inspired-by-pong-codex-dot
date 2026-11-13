use pong_web_ui_common::{Session, CSS};
use wasm_bindgen::JsCast;
use yew::prelude::*;
#[function_component(App)]
fn app() -> Html {
    let session = use_mut_ref(Session::default);
    let game_root = use_node_ref();
    let frame = use_state(|| Session::default().svg());
    {
        let session = session.clone();
        let frame = frame.clone();
        use_effect_with((), move |_| {
            let mut last = web_time::Instant::now();
            let interval = gloo_timers::callback::Interval::new(16, move || {
                let mut s = session.borrow_mut();
                let now = web_time::Instant::now();
                s.tick(now.duration_since(last).as_secs_f64());
                last = now;
                frame.set(s.svg());
            });
            move || drop(interval)
        });
    }
    let down = {
        let s = session.clone();
        Callback::from(move |e: KeyboardEvent| {
            if pong_web_ui_common::handled_key(&e.key()) {
                e.prevent_default();
            }
            s.borrow_mut().key(&e.key(), true, e.repeat());
        })
    };
    let preserve_activation = Callback::from(|e: KeyboardEvent| {
        if pong_web_ui_common::native_activation_key(&e.key()) {
            e.stop_propagation();
        }
    });
    let up = {
        let s = session.clone();
        Callback::from(move |e: KeyboardEvent| s.borrow_mut().key(&e.key(), false, false))
    };
    let blur = {
        let s = session.clone();
        let root = game_root.clone();
        Callback::from(move |e: FocusEvent| {
            let next_inside = root
                .cast::<web_sys::Node>()
                .zip(
                    e.related_target()
                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok()),
                )
                .is_some_and(|(root, next)| root.contains(Some(&next)));
            s.borrow_mut().focus_left(next_inside);
        })
    };
    let start = {
        let s = session.clone();
        Callback::from(move |_| s.borrow_mut().start())
    };
    let pause = {
        let s = session.clone();
        Callback::from(move |_| s.borrow_mut().toggle_pause())
    };
    let touch = |direction: pong_web_ui_common::Direction, pressed: bool| {
        let s = session.clone();
        Callback::from(move |e: PointerEvent| {
            e.prevent_default();
            s.borrow_mut()
                .movement_button(direction, e.pointer_id(), pressed)
        })
    };
    let button_key = |direction: pong_web_ui_common::Direction, pressed: bool| {
        let s = session.clone();
        Callback::from(move |e: KeyboardEvent| {
            if pong_web_ui_common::native_activation_key(&e.key()) {
                e.prevent_default();
                e.stop_propagation();
                s.borrow_mut()
                    .movement_button_key(direction, &e.key(), pressed, e.repeat());
            }
        })
    };
    let button_blur = |direction: pong_web_ui_common::Direction| {
        let s = session.clone();
        Callback::from(move |_: FocusEvent| s.borrow_mut().movement_button_blur(direction))
    };
    let activate = |direction: pong_web_ui_common::Direction| {
        let s = session.clone();
        Callback::from(move |e: MouseEvent| {
            if e.detail() == 0 {
                s.borrow_mut().movement_button_activate(direction);
            }
        })
    };
    html! {<><style>{CSS}</style><main ref={game_root} tabindex="0" autofocus=true onkeydown={down} onkeyup={up} onfocusout={blur}>
    <h1>{"Pong • Yew"}</h1><p>{"Rust shared simulation · Yew / WASM"}</p>
    {Html::from_html_unchecked(AttrValue::from((*frame).clone()))}
    <button onkeydown={preserve_activation.clone()} onclick={start}>{"Start / Restart"}</button><button onkeydown={preserve_activation.clone()} onclick={pause}>{"Pause / Resume"}</button>
    <button onkeydown={button_key(pong_web_ui_common::Direction::Up,true)} onkeyup={button_key(pong_web_ui_common::Direction::Up,false)} onblur={button_blur(pong_web_ui_common::Direction::Up)} onclick={activate(pong_web_ui_common::Direction::Up)} onpointerdown={touch(pong_web_ui_common::Direction::Up,true)} onpointerup={touch(pong_web_ui_common::Direction::Up,false)} onpointerleave={touch(pong_web_ui_common::Direction::Up,false)} onpointercancel={touch(pong_web_ui_common::Direction::Up,false)}>{"▲ Hold"}</button>
    <button onkeydown={button_key(pong_web_ui_common::Direction::Down,true)} onkeyup={button_key(pong_web_ui_common::Direction::Down,false)} onblur={button_blur(pong_web_ui_common::Direction::Down)} onclick={activate(pong_web_ui_common::Direction::Down)} onpointerdown={touch(pong_web_ui_common::Direction::Down,true)} onpointerup={touch(pong_web_ui_common::Direction::Down,false)} onpointerleave={touch(pong_web_ui_common::Direction::Down,false)} onpointercancel={touch(pong_web_ui_common::Direction::Down,false)}>{"▼ Hold"}</button>
    <p>{"W/S or ↑/↓ move · Enter restarts · Space pauses · Hold Space or Enter on movement buttons. Click the court to focus."}</p>
    <a onkeydown={preserve_activation} href="https://intrepion.github.io/">{"Back to Kata Arcade"}</a>
    </main></>}
}
fn main() {
    yew::Renderer::<App>::new().render();
}
