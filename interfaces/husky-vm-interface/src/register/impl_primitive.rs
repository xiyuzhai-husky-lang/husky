use crate::*;

impl __StaticInfo for i32 {
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl __Registrable for i32 {
    unsafe fn __to_register(self) -> __Register {
        __Register::new_direct::<Self>(self as u64)
    }
}
