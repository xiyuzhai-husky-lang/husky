mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;

use super::*;

pub trait __Registrable:
    __StaticInfo + std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized
{
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval>;

    fn __copy__(&self) -> Self;
}

impl __StaticInfo for PrimitiveValueData {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl __Registrable for PrimitiveValueData {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        match self {
            PrimitiveValueData::I32(i) => i.to_register(),
            PrimitiveValueData::I64(i) => i.to_register(),
            PrimitiveValueData::F32(f) => f.to_register(),
            PrimitiveValueData::B32(b) => b.to_register(),
            PrimitiveValueData::B64(_) => todo!(),
            PrimitiveValueData::Bool(_) => todo!(),
            PrimitiveValueData::Void(_) => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        todo!()
    }
}
