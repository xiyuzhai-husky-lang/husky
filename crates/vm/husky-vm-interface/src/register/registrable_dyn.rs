use super::*;

// pub trait __RegistrableDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
//     unsafe fn __drop_dyn__(&mut self);
//     fn __static_typename_dyn__(&self) -> std::borrow::Cow<'static, str>;
//     fn __static_type_id_dyn__(&self) -> std::any::TypeId;
//     fn __fmt_debug__(&self) -> String;
//     fn __primitive_dyn__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData;
// }

// impl<T> __RegistrableDyn for T
// where
//     T: __Registrable,
// {
//     unsafe fn __drop_dyn__(&mut self) {
//         let ptr: *mut T = self;
//         drop(Box::from_raw(ptr));
//     }

//     fn __static_typename_dyn__(&self) -> std::borrow::Cow<'static, str> {
//         T::__static_typename()
//     }

//     fn __fmt_debug__(&self) -> String {
//         format!("{:?}", self)
//     }

//     fn __static_type_id_dyn__(&self) -> std::any::TypeId {
//         T::__static_type_id__()
//     }

//     fn __primitive_dyn__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
//         self.__primitive__(data_kind)
//     }
// }
