use std::panic;

use crate::utils::{add_event_listener, presentation_dialog};
use husky_trace_protocol_old::SampleId;
use wasm_bindgen::JsCast;

pub(crate) fn init() {
    init_debugging_env();
}

fn init_debugging_env() {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
