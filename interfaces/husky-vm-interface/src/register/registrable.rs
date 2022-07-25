use super::*;

pub trait __Registrable:
    __StaticInfo + std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe
{
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval>;
    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        panic!()
    }
}
