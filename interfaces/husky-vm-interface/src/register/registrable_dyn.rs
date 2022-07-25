use super::*;

pub trait __RegistrableDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    unsafe fn drop_dyn(&mut self);
    fn static_type_name(&self) -> std::borrow::Cow<'static, str>;
    fn static_type_id(&self) -> std::any::TypeId;
    fn fmt_debug(&self) -> String;
    fn primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData;
}

impl<T> __RegistrableDyn for T
where
    T: __Registrable,
{
    unsafe fn drop_dyn(&mut self) {
        let ptr: *mut T = self;
        drop(Box::from_raw(ptr));
    }

    fn static_type_name(&self) -> std::borrow::Cow<'static, str> {
        T::__static_type_name()
    }

    fn fmt_debug(&self) -> String {
        format!("{:?}", self)
    }

    fn static_type_id(&self) -> std::any::TypeId {
        T::__static_type_id()
    }

    fn primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        self.__primitive(data_kind)
    }
}
