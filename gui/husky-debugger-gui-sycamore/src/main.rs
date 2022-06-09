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
use sycamore::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|scope| {
        let state = create_signal(scope, 0); 
        let context = provide_context(scope, DebuggerContext::new());
        view! { scope,
           "Hello, World!"
        }
    });
}
