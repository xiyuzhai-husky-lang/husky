#![allow(dead_code, warnings)]

mod components;
mod context;
mod services;
mod store;

use components::*;
use context::*;
use services::*;
use std::{cell::RefCell, rc::Rc};
use store::*;
use yew::{prelude::*, *};

#[function_component(DebuggerPanel)]
fn debugger_panel() -> Html {
    let ctx = use_state(|| DebuggerContext::new());
    let i = use_store(ctx.get_store());
    let j = use_state(|| 1);

    let onclick = {
        let i = i.clone();
        let j = j.clone();
        Callback::from(move |_what| {
            i.set(*i + 1);
            j.set(*j + 1)
        })
    };

    html! {
        <ContextProvider<Rc<DebuggerContext>> context={(*ctx).clone()}>
            <div>{"i = "}{i}{", j = "}{*j}</div>
            <button {onclick}></button>
        </ContextProvider<Rc<DebuggerContext>>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<DebuggerPanel>();
}
