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
    sycamore::render(|cx| {
        let state = create_signal(cx, 0);
        create_effect(cx, || {
            println!("The state changed. New value: {}", state.get())
        });
        // Prints "The state changed. New value: 0"
        // (note that the effect is always executed at least 1 regardless of state changes)

        state.set(1); // Prints "The state changed. New value: 1"
        state.set(2); // Prints "The state changed. New value: 2"
        state.set(3); // Prints "The state changed. New value: 3"
        view! { cx,
           "Hello, World!"
        }
    });
}
