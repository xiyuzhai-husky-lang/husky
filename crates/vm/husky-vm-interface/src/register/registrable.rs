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

impl<'eval, T> __Registrable<'eval> for Option<T>
where
    T: __Registrable<'eval>,
{
    unsafe fn __to_register(self) -> __Register<'eval> {
        match self {
            Some(v) => v.__to_register(),
            None => __Register::new_undefined(),
        }
    }
}
