#![allow(dead_code, warnings)]

mod abstraction;
mod components;
mod context;
mod init;
mod services;
mod store;

use abstraction::*;
use components::*;
use context::*;
use husky_debugger_protocol::*;
use init::init_debugging_env;
use services::*;
use std::{cell::RefCell, rc::Rc};
use store::*;
use sycamore::prelude::*;

fn main() {
    init_debugging_env();
    sycamore::render(|scope| {
        let state = create_signal(scope, 0);
        let context = provide_context(scope, DebuggerContext::new());
        view! { scope,
           "Hello, World!"
           HSplitPanel {
               value: state
           }
        }
    });
}
