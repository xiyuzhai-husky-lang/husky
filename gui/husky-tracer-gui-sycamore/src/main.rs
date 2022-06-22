#![allow(dead_code, warnings)]
mod cell;
mod components;
mod context;
mod init;
mod services;
mod store;
mod utils;

use cell::RefCell;
use components::*;
use context::*;
use husky_tracer_protocol::*;
use init::init_debugging_env;
use services::*;
use std::{any::TypeId, rc::Rc};
use store::*;
use sycamore::prelude::*;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn main() {
    init_debugging_env();
    let gui: Element = get_gui();
    sycamore::render_to_static(
        |scope| {
            let context =
                unsafe { as_static_ref(provide_context(scope, DebuggerContext::new_ref(scope))) };
            let layout_width = memo!(scope, || math::round::floor(
                context.window_inner_width.cget(),
                0
            ) as u32);
            let layout_height = memo!(scope, || math::round::floor(
                context.window_inner_height.cget(),
                0
            ) as u32);
            let keydown_handler = context.keydown_handler();
            view! {
                scope,
                div (
                    class="Main",
                    tabindex=1,
                    on:keydown=keydown_handler
                ) {
                    Layout {
                        width: layout_width,
                        height: layout_height,
                    }
                }
            }
        },
        unsafe { as_static_ref(&gui) },
    );
}
