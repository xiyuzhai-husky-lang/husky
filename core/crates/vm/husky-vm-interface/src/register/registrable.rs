mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;

use super::*;

pub trait __Registrable<'eval>:
    __StaticInfo + std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized
{
    unsafe fn __to_register(self) -> __Register<'eval>;
}

impl __StaticInfo for PrimitiveValueData {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}
