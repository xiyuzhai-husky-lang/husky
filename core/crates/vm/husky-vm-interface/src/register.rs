mod registrable;
mod registrable_dyn;
mod registrable_safe;
mod vtable;

use husky_vm_primitive_value::PrimitiveValueData;
pub use registrable::*;
pub use registrable_dyn::*;
pub use registrable_safe::*;
pub use vtable::*;

use crate::*;
use std::{
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[repr(C)]
pub struct __Register<'eval> {
    pub(crate) data_kind: __RegisterDataKind,
    pub(crate) data: __RegisterData,
    pub vtable: &'eval __RegisterVTable,
}

impl<'eval> std::hash::Hash for __Register<'eval> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union __RegisterData {
    pub as_void: (),
    pub as_bool: bool,
    pub as_i32: i32,
    pub as_i64: i64,
    pub as_b32: u32,
    pub as_b64: u64,
    pub as_f32: f32,
    pub as_f64: f64,
    pub as_ptr: *mut (),
}

unsafe impl<'eval> Send for __Register<'eval> {}
unsafe impl<'eval> Sync for __Register<'eval> {}

impl<'eval> std::fmt::Debug for __Register<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("__Register")
    }
}

#[cfg(feature = "extra")]
impl<'eval> Clone for __Register<'eval> {
    fn clone(&self) -> Self {
        unsafe { self.extrinsic_clone() }
    }
}

impl<'eval> PartialEq for __Register<'eval> {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.vtable as *const _, other.vtable as *const _);
        self.vtable.eq.unwrap()(self.any_ref(), other.any_ref())
    }
}
impl<'eval> Eq for __Register<'eval> {}

pub trait __StaticInfo {
    type __StaticSelf: __StaticInfo<__StaticSelf = Self::__StaticSelf> + 'static;

    fn __static_type_id__() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }

    fn __static_typename() -> std::borrow::Cow<'static, str>;
}

impl<'eval> __Register<'eval> {
    fn any_ref(&self) -> &() {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => unsafe {
                &*(&self.data as *const _ as *const ())
            },
            __RegisterDataKind::Box | __RegisterDataKind::EvalRef | __RegisterDataKind::TempRef => unsafe {
                &*self.data.as_ptr
            },
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub fn data_kind(&self) -> __RegisterDataKind {
        self.data_kind
    }
    pub fn data(&self) -> __RegisterData {
        self.data
    }

    pub unsafe fn new_primitive_value(
        data: __RegisterData,
        proto: &'eval __RegisterVTable,
    ) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data,
            vtable: proto,
        }
    }

    pub fn new_box<T>(value: T, proto: &'eval __RegisterVTable) -> __Register<'eval> {
        let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
        __Register {
            data_kind: __RegisterDataKind::Box,
            data: __RegisterData {
                as_ptr: ptr as *mut (),
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_eval_ref<T: 'eval>(
        value: &'eval T,
        proto: &'eval __RegisterVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::EvalRef,
            data: __RegisterData {
                as_ptr: ptr as *mut (),
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_temp_ref<T: __Registrable>(
        value: &T,
        proto: &'eval __RegisterVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempRef,
            data: __RegisterData {
                as_ptr: ptr as *mut (),
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_temp_mut<T: __Registrable>(
        value: &mut T,
        proto: &'eval __RegisterVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempMut,
            data: __RegisterData {
                as_ptr: ptr as *mut (),
            },
            vtable: proto,
        }
    }

    pub fn register_move(&mut self) -> __Register<'eval> {
        let moved = __Register {
            data_kind: __RegisterDataKind::Moved,
            data: __RegisterData { as_void: () },
            vtable: self.vtable,
        };
        std::mem::replace(self, moved)
    }

    pub fn new_undefined(proto: &'eval __RegisterVTable) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            data: __RegisterData { as_void: () },
            vtable: proto,
        }
    }

    pub fn new_void() -> __Register<'eval> {
        unsafe {
            __Register {
                data_kind: __RegisterDataKind::PrimitiveValue,
                data: __RegisterData { as_void: () },
                vtable: &__VOID_VTABLE,
            }
        }
    }

    pub fn new_unreturned(vtable: &'eval __RegisterVTable) -> __Register<'eval> {
        unsafe {
            __Register {
                data_kind: __RegisterDataKind::Unreturned,
                data: __RegisterData { as_void: () },
                vtable,
            }
        }
    }

    pub fn new_moved(vtable: &'eval __RegisterVTable) -> __Register<'eval> {
        unsafe {
            __Register {
                data_kind: __RegisterDataKind::Moved,
                data: __RegisterData { as_void: () },
                vtable,
            }
        }
    }

    pub unsafe fn new_undefined_with_message(
        proto: &'eval __RegisterVTable,
        message: String,
    ) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Undefined,
            data: __RegisterData { as_void: () },
            vtable: proto,
        }
    }

    pub unsafe fn __copy__(&self) -> Self {
        todo!()
        // Self {
        //     data_kind: self.data_kind,
        //     data: match self.data_kind {
        //         __RegisterDataKind::Data => todo!(),
        //         __RegisterDataKind::Box => todo!(),
        //         __RegisterDataKind::EvalRef => todo!(),
        //         __RegisterDataKind::TempRef => todo!(),
        //         __RegisterDataKind::TempMut => todo!(),
        //         __RegisterDataKind::Moved => todo!(),
        //     },
        // }
    }

    pub fn downcast_unbox<T>(self) -> T
    where
        T: 'eval,
    {
        assert_eq!(self.data_kind, __RegisterDataKind::Box);
        let t = unsafe { *Box::from_raw(self.data.as_ptr as *mut T) };
        std::mem::forget(self);
        t
    }

    pub unsafe fn downcast_temp<T>(&mut self) -> T {
        todo!()
    }

    pub unsafe fn downcast_eval_ref<T: 'eval>(&self) -> &'eval T {
        todo!()
    }

    pub unsafe fn downcast_temp_ref<T>(&self) -> &T {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => todo!(),
            __RegisterDataKind::Box
            | __RegisterDataKind::EvalRef
            | __RegisterDataKind::TempRef
            | __RegisterDataKind::TempMut => &*(self.data.as_ptr as *const T),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub unsafe fn downcast_temp_mut<T>(&mut self) -> &mut T {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => &mut *(&mut self.data as *mut _ as *mut T),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => &mut *(self.data.as_ptr as *mut T),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum __RegisterDataKind {
    PrimitiveValue,
    Box,
    EvalRef,
    TempRef,
    TempMut,
    Moved,
    Undefined,
    Unreturned,
}

impl<'eval> Drop for __Register<'eval> {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box => unsafe {
                (self.vtable.drop.unwrap())(self.data.as_ptr)
                // (*std::mem::replace(&mut self.data, __RegisterData { as_opt_ptr: None })
                //     .as_opt_ptr
                //     .unwrap())
                // .__drop_dyn__()
            },
            __RegisterDataKind::Undefined => {
                // when undefined, opt_data might hold a message
                todo!()
            }
            _ => (),
        }
    }
}

#[macro_export]
macro_rules! register_new_copyable {
    ($argument: expr, $Type: ty, direct) => {{
        ($argument).to_register()
    }};
}

// impl<'temp, 'eval: 'temp> dyn __RegistrableDyn + 'temp {
//     #[inline]
//     pub fn __downcast_ref<'a, T: __Registrable>(&'a self) -> &'a T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_typename(),
//                 self.__static_typename_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { &*ptr }
//     }
//     #[inline]
//     pub fn __downcast_copy<'a, T:  Copy>(&'a self) -> T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_typename(),
//                 self.__static_typename_dyn()
//             )
//         }
//         let ptr: *const dyn __AnyValueDyn = &*self;
//         let ptr: *const T = ptr as *const T;
//         unsafe { *ptr }
//     }

//     #[inline]
//     pub fn __downcast_mut<T: __Registrable>(&mut self) -> &mut T {
//         if T::__static_type_id() != self.__static_type_id_dyn() {
//             panic!(
//                 "expect type `{}`, but got `{}` instead",
//                 T::__static_typename(),
//                 self.__static_typename_dyn()
//             )
//         }
//         let ptr: *mut dyn __AnyValueDyn = &mut *self;
//         let ptr: *mut T = ptr as *mut T;
//         unsafe { &mut *ptr }
//     }
// }
