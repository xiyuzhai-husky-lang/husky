use std::panic;

pub(crate) fn init_debugging_env() {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
