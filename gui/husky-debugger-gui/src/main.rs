mod components;
mod services;
mod state;

use components::*;
use services::*;
use state::*;
use std::rc::Rc;
use yew::{prelude::*, *};

#[function_component(DebuggerPanel)]
fn debugger_panel() -> Html {
    let ctx = use_state(|| DebuggerState::new());

    html! {
        <ContextProvider<Rc<DebuggerState>> context={(*ctx).clone()}>
            <div>{"sdfsdf"}</div>
        </ContextProvider<Rc<DebuggerState>>>
    }
}

fn main() {
    yew::start_app::<DebuggerPanel>();
}
