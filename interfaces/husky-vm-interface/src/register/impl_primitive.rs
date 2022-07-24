use crate::*;

impl __StaticInfo for i32 {
    type StaticSelf = Self;
}

impl __Registrable for i32 {
    unsafe fn __to_register(self) -> __Register {
        __Register::new_direct::<Self>(self as u64)
    }
}
