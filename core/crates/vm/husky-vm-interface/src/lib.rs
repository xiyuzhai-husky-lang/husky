mod __rust_codegen__;
mod binding;
mod context;
mod error;
mod layout;
mod linkage;
mod register;
#[cfg(test)]
mod tests;

pub use __rust_codegen__::*;
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
