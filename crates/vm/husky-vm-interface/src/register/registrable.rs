mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;

use super::*;

pub trait __Registrable:
    __StaticInfo + std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized
{
    unsafe fn __to_register(self) -> __RegularValue;
}

impl __StaticInfo for PrimitiveValueData {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        self
    }
}

impl<T> __Registrable for Option<T>
where
    T: __Registrable,
{
    unsafe fn __to_register(self) -> __RegularValue {
        match self {
            Some(v) => v.__to_register(),
            None => __RegularValue::none(0),
        }
    }
}
