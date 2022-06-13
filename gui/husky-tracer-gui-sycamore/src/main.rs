#![allow(dead_code, warnings)]

mod abstraction;
mod components;
mod context;
mod init;
mod services;
mod store;
mod utils;

use abstraction::*;
use components::*;
use context::*;
use husky_tracer_protocol::*;
use init::init_debugging_env;
use services::*;
use std::{any::TypeId, cell::RefCell, rc::Rc};
use store::*;
use sycamore::prelude::*;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn main() {
    init_debugging_env();
    let gui: Element = get_gui();
    sycamore::render_get_scope(
        |scope| {
            let context = provide_context(scope, TracerContext::new());
            context.window_inner_width.track();
            context.window_inner_height.track();
            view! {
                scope,
                Layout {
                    width: &context.window_inner_width,
                    height: &context.window_inner_height,
                }
            }
        },
        &gui,
    );
}

fn get_gui() -> Element {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    document
        .get_elements_by_class_name("HuskyTracerGui")
        .item(0)
        .unwrap()
}
