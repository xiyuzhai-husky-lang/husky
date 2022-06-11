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
use husky_tracer_protocol::*;
use init::init_debugging_env;
use services::*;
use std::{any::TypeId, cell::RefCell, rc::Rc};
use store::*;
use sycamore::prelude::*;

fn main() {
    init_debugging_env();
    sycamore::render(|scope| {
        let state = create_signal(scope, 0);
        let context = provide_context(scope, TracerContext::new());
        view! {
            scope,
           "Hello, World!"
           HSplitPanel {
               value: state
           }
        }
    });
}
