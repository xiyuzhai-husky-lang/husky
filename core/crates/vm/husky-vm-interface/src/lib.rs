mod __rust_code_gen__;
mod binding;
mod call_form_value;
mod context;
mod error;
mod layout;
mod linkage;
mod register;
#[cfg(test)]
mod tests;

pub use __rust_code_gen__::*;
pub use call_form_value::*;
pub use context::*;
pub use error::*;
pub use layout::*;
pub use linkage::*;
pub use register::*;

#[repr(C)]
pub struct Play {
    a: u32,
}

#[no_mangle]
pub extern "C" fn play() -> Play {
    todo!()
}
