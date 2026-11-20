use pong_web_ui_common::{Session, CSS};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
#[function_component(App)]
fn app() -> Html {
    let session = use_mut_ref(Session::default);
    let game_root = use_node_ref();
    let frame = use_state(|| Session::default().svg());
    let status = use_state_eq(|| Session::default().status_text());
    {
        let session = session.clone();
        let frame = frame.clone();
        let status = status.clone();
        use_effect_with((), move |_| {
            let mut last = web_time::Instant::now();
            let interval = gloo_timers::callback::Interval::new(16, move || {
                let mut s = session.borrow_mut();
                let now = web_time::Instant::now();
                s.tick(now.duration_since(last).as_secs_f64());
                last = now;
                frame.set(s.svg());
                status.set(s.status_text());
            });
            move || drop(interval)
        });
    }
    // Element focusout is not guaranteed when switching browser tabs or apps.
    {
        let session = session.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().expect("browser window");
            let document = window.document().expect("browser document");
            let blur_session = session.clone();
            let blur = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                blur_session.borrow_mut().focus_left(false);
            });
            let visibility_document = document.clone();
            let visibility = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                if visibility_document.hidden() {
                    session.borrow_mut().focus_left(false);
                }
            });
            window.add_event_listener_with_callback("blur", blur.as_ref().unchecked_ref())
                .expect("register window blur");
            document.add_event_listener_with_callback("visibilitychange", visibility.as_ref().unchecked_ref())
                .expect("register document visibility");
            move || {
                let _ = window.remove_event_listener_with_callback("blur", blur.as_ref().unchecked_ref());
                let _ = document.remove_event_listener_with_callback("visibilitychange", visibility.as_ref().unchecked_ref());
                // Drop closures only after removing their listeners.
                drop((blur, visibility));
            }
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
    let cancel_touch = |direction: pong_web_ui_common::Direction| {
        let s = session.clone();
        Callback::from(move |e: PointerEvent| {
            e.prevent_default();
            s.borrow_mut().movement_button_cancel(direction, e.pointer_id());
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
    <p role="status">{(*status).clone()}</p>
    {Html::from_html_unchecked(AttrValue::from((*frame).clone()))}
    <button onkeydown={preserve_activation.clone()} onclick={start}>{"Start / Restart"}</button><button onkeydown={preserve_activation.clone()} onclick={pause}>{"Pause / Resume"}</button>
    <button onkeydown={button_key(pong_web_ui_common::Direction::Up,true)} onkeyup={button_key(pong_web_ui_common::Direction::Up,false)} onblur={button_blur(pong_web_ui_common::Direction::Up)} onclick={activate(pong_web_ui_common::Direction::Up)} onpointerdown={touch(pong_web_ui_common::Direction::Up,true)} onpointerup={touch(pong_web_ui_common::Direction::Up,false)} onpointerleave={cancel_touch(pong_web_ui_common::Direction::Up)} onpointercancel={cancel_touch(pong_web_ui_common::Direction::Up)}>{"▲ Hold"}</button>
    <button onkeydown={button_key(pong_web_ui_common::Direction::Down,true)} onkeyup={button_key(pong_web_ui_common::Direction::Down,false)} onblur={button_blur(pong_web_ui_common::Direction::Down)} onclick={activate(pong_web_ui_common::Direction::Down)} onpointerdown={touch(pong_web_ui_common::Direction::Down,true)} onpointerup={touch(pong_web_ui_common::Direction::Down,false)} onpointerleave={cancel_touch(pong_web_ui_common::Direction::Down)} onpointercancel={cancel_touch(pong_web_ui_common::Direction::Down)}>{"▼ Hold"}</button>
    <p>{"W/S or ↑/↓ move · Enter restarts · Space pauses · Hold Space or Enter on movement buttons. Click the court to focus."}</p>
    <a onkeydown={preserve_activation} href="https://intrepion.github.io/">{"Back to Kata Arcade"}</a>
    </main></>}
}
fn main() {
    yew::Renderer::<App>::new().render();
}
