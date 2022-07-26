mod impl_cyclic_slice;
mod impl_hashmap;
mod impl_primitive;

use super::*;

pub trait __Registrable:
    __StaticInfo + std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized
{
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval>;

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        panic!()
    }

    fn __copy__(&self) -> Self;
}
