use pong_web_ui_common::{Session, CSS};
use yew::prelude::*;
#[function_component(App)]
fn app() -> Html {
    let session = use_mut_ref(Session::default);
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
    let up = {
        let s = session.clone();
        Callback::from(move |e: KeyboardEvent| s.borrow_mut().key(&e.key(), false, false))
    };
    let blur = {
        let s = session.clone();
        Callback::from(move |_| s.borrow_mut().pause())
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
            s.borrow_mut().movement_button(direction, pressed)
        })
    };
    html! {<><style>{CSS}</style><main tabindex="0" autofocus=true onkeydown={down} onkeyup={up} onblur={blur}>
    <h1>{"Pong • Yew"}</h1><p>{"Rust shared simulation · Yew / WASM"}</p>
    {Html::from_html_unchecked(AttrValue::from((*frame).clone()))}
    <button onclick={start}>{"Start / Restart"}</button><button onclick={pause}>{"Pause / Resume"}</button>
    <button onpointerdown={touch(pong_web_ui_common::Direction::Up,true)} onpointerup={touch(pong_web_ui_common::Direction::Up,false)} onpointerleave={touch(pong_web_ui_common::Direction::Up,false)} onpointercancel={touch(pong_web_ui_common::Direction::Up,false)}>{"▲ Hold"}</button>
    <button onpointerdown={touch(pong_web_ui_common::Direction::Down,true)} onpointerup={touch(pong_web_ui_common::Direction::Down,false)} onpointerleave={touch(pong_web_ui_common::Direction::Down,false)} onpointercancel={touch(pong_web_ui_common::Direction::Down,false)}>{"▼ Hold"}</button>
    <p>{"W/S or ↑/↓ move · Enter restarts · Space pauses. Click the court to focus."}</p>
    <a href="https://intrepion.github.io/">{"Back to Kata Arcade"}</a>
    </main></>}
}
fn main() {
    yew::Renderer::<App>::new().render();
}
