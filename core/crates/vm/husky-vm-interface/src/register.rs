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

#[test]
fn test_register_data_size() {
    println!(
        "std::mem::size_of::<f64>() = {}",
        std::mem::size_of::<f64>()
    );
    println!(
        "std::mem::size_of::<*mut ()>() = {}",
        std::mem::size_of::<*mut ()>()
    );
    println!(
        "std::mem::size_of::< *mut () >() = {}",
        std::mem::size_of::<*mut ()>()
    );
    assert_eq!(std::mem::size_of::<f64>(), std::mem::size_of::<*mut ()>(),);
    assert_eq!(
        std::mem::size_of::<f64>(),
        std::mem::size_of::<__RegisterData>()
    )
}

// C standard (N1570, 6.7.2.1 Structure and union specifiers) says:
// 16 The size of a union is sufficient to contain the largest of its members.
// The value of at most one of the members can be stored in a union object at any time.
// A pointer to a union object, suitably converted, points to each of its members
// (or if a member is a bit- field, then to the unit in which it resides),
// and vice versa.
#[test]
fn test_alignment() {
    let a = __RegisterData { as_void: () };
    unsafe {
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_void as *const _ as *const (),
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_bool as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_ptr as *const _ as *const ()
        )
    }
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
        todo!()
        // self.data_kind == other.data_kind && self.opt_data == other.opt_data
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

    // unsafe fn move_into_raw(&mut self) -> *mut dyn __RegistrableDyn {
    //     self.data_kind = __RegisterDataKind::Moved;
    //     std::mem::replace(&mut self.data, __RegisterData { as_opt_ptr: None })
    //         .as_opt_ptr
    //         .unwrap()
    // }

    pub fn downcast_void(&self) -> () {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__VOID_VTABLE as *const _);
            self.data.as_void
        }
    }

    pub fn downcast_bool(&self) -> bool {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__BOOL_VTABLE as *const _);
            self.data.as_bool
        }
    }

    pub fn downcast_i32(&self) -> i32 {
        unsafe {
            assert_eq!(self.vtable as *const _, &__I32_VTABLE as *const _);
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_i32,
                _ => *(self.data.as_ptr as *const i32),
            }
        }
    }

    pub fn downcast_i64(&self) -> i64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__I64_VTABLE as *const _);
            self.data.as_i64
        }
    }

    pub fn downcast_b32(&self) -> u32 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__B32_VTABLE as *const _);
            self.data.as_b32
        }
    }

    pub fn downcast_b64(&self) -> u64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__B64_VTABLE as *const _);
            self.data.as_b64
        }
    }

    pub fn downcast_f32(&self) -> f32 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__F32_VTABLE as *const _);
            self.data.as_f32
        }
    }

    pub fn downcast_f64(&self) -> f64 {
        assert_eq!(self.data_kind, __RegisterDataKind::PrimitiveValue);
        unsafe {
            assert_eq!(self.vtable as *const _, &__F64_VTABLE as *const _);
            self.data.as_f64
        }
    }

    // pub fn downcast<T>(&mut self) -> T
    // where
    //     T:  'eval,
    // {
    //     match self.data_kind {
    //         __RegisterDataKind::PrimitiveValue => todo!(),
    //         __RegisterDataKind::Box => unsafe { *Box::from_raw(self.move_into_raw() as *mut T) },
    //         __RegisterDataKind::EvalRef => todo!(),
    //         __RegisterDataKind::TempRef => todo!(),
    //         __RegisterDataKind::TempMut => todo!(),
    //         __RegisterDataKind::Moved => todo!(),
    //         __RegisterDataKind::Undefined => todo!(),
    //         __RegisterDataKind::Unreturned => todo!(),
    //     }
    // }

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

// impl<'eval> From<PrimitiveValueData> for __Register<'eval> {
//     fn from(value: PrimitiveValueData) -> Self {
//         match value {
//             PrimitiveValueData::I32(_) => todo!(),
//             PrimitiveValueData::I64(_) => todo!(),
//             PrimitiveValueData::F32(_) => todo!(),
//             PrimitiveValueData::B32(_) => todo!(),
//             PrimitiveValueData::B64(_) => todo!(),
//             PrimitiveValueData::Bool(_) => todo!(),
//             PrimitiveValueData::Void(_) => todo!(),
//         }
//     }
// }

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
