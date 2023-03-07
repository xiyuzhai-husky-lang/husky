mod registrable;
mod registrable_safe;
mod vtable;

use husky_vm_primitive_value::PrimitiveValueData;
pub use registrable::*;
pub use registrable_safe::*;
pub use vtable::*;
use wild_utils::wild_arb_ref;

use crate::*;
use std::{
    ffi::c_void,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[repr(C)]
pub struct __Register<'eval> {
    pub(crate) data_kind: __RegisterDataKind,
    pub(crate) data: __RegisterData,
    pub vtable: &'eval __RegisterTyVTable,
}

impl<'eval> std::hash::Hash for __Register<'eval> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            (
                self.data_kind,
                self.data.as_ptr,
                self.vtable.typename_str_hash_u64,
            )
                .hash(state)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union __RegisterData {
    pub as_void: (),
    pub as_bool: bool,
    pub as_i32: i32,
    pub as_i64: i64,
    pub as_r32: u32,
    pub as_b64: u64,
    pub as_not_nan_f32: ordered_float::NotNan<f32>,
    pub as_f64: f64,
    pub as_ptr: *mut c_void,
    pub as_number_of_somes: u8,
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
        self.vtable == other.vtable && unsafe { (self.vtable.eq)(self.any_ref(), other.any_ref()) }
    }
}
impl<'eval> Eq for __Register<'eval> {}

pub trait __StaticInfo {
    type __StaticSelf: __StaticInfo<__StaticSelf = Self::__StaticSelf> + 'static;

    fn __static_type_id__() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }

    fn __static_typename() -> std::borrow::Cow<'static, str>;

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized;
}

impl<T> __StaticInfo for &T
where
    T: __StaticInfo,
{
    type __StaticSelf = &'static T::__StaticSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        wild_arb_ref(self)
    }
}

impl<T> __StaticInfo for &mut T
where
    T: __StaticInfo,
{
    type __StaticSelf = &'static mut T::__StaticSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        wild_arb_ref(self)
    }
}

impl<T> __StaticInfo for Option<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Option<T::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        todo!()
    }
}

impl<T> __StaticInfo for Vec<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Vec<T::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        todo!()
    }
}

impl<'eval> __Register<'eval> {
    fn any_ref(&self) -> &c_void {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => unsafe {
                &*(&self.data as *const _ as *const c_void)
            },
            __RegisterDataKind::Box | __RegisterDataKind::Leash | __RegisterDataKind::TempRef => unsafe {
                &*self.data.as_ptr
            },
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::SomeNone => todo!(),
            __RegisterDataKind::Unreturned => unsafe {
                &*(&self.data.as_void as *const _ as *const c_void)
            },
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
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data,
            vtable: proto,
        }
    }

    pub fn new_opt_box<T>(
        opt_value: Option<T>,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        if let Some(value) = opt_value {
            let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
            __Register {
                data_kind: __RegisterDataKind::Box,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __Register::none(0)
        }
    }

    pub fn new_box<T>(value: T, proto: &'eval __RegisterTyVTable) -> __Register<'eval> {
        let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
        __Register {
            data_kind: __RegisterDataKind::Box,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub fn new_eval_ref<T: 'eval>(
        value: &'eval T,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::Leash,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub fn new_opt_eval_ref<T: 'eval>(
        opt_value: Option<&'eval T>,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        if let Some(value) = opt_value {
            Self::new_eval_ref(value, proto)
        } else {
            Self::none(0)
        }
    }

    pub unsafe fn new_temp_ref<T>(
        value: &T,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempRef,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_opt_temp_ref<T>(
        opt_value: Option<&T>,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        if let Some(value) = opt_value {
            let ptr: *const T = value;
            __Register {
                data_kind: __RegisterDataKind::TempRef,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __Register::none(0)
        }
    }

    pub unsafe fn new_temp_mut<T>(
        value: &mut T,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        let ptr: *const T = value;
        __Register {
            data_kind: __RegisterDataKind::TempMut,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_opt_temp_mut<T>(
        opt_value: Option<&mut T>,
        proto: &'eval __RegisterTyVTable,
    ) -> __Register<'eval> {
        if let Some(value) = opt_value {
            let ptr: *const T = value;
            __Register {
                data_kind: __RegisterDataKind::TempMut,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __Register::none(0)
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

    pub fn none(number_of_somes_before_none: u8) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::SomeNone,
            data: __RegisterData {
                as_number_of_somes: number_of_somes_before_none,
            },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn new_void() -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data: __RegisterData { as_void: () },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn unreturned() -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Unreturned,
            data: __RegisterData { as_void: () },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn new_moved(vtable: &'eval __RegisterTyVTable) -> __Register<'eval> {
        __Register {
            data_kind: __RegisterDataKind::Moved,
            data: __RegisterData { as_void: () },
            vtable,
        }
    }

    pub unsafe fn __copy__(&self) -> Self {
        todo!()
        // Self {
        //     data_kind: self.data_kind,
        //     data: match self.data_kind {
        //         __RegisterDataKind::Data => todo!(),
        //         __RegisterDataKind::Box => todo!(),
        //         __RegisterDataKind::Leash => todo!(),
        //         __RegisterDataKind::TempRef => todo!(),
        //         __RegisterDataKind::TempMut => todo!(),
        //         __RegisterDataKind::Moved => todo!(),
        //     },
        // }
    }

    pub fn downcast_unbox<T>(self, target_ty_vtable: &__RegisterTyVTable) -> T
    where
        T: 'eval,
    {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!()
        }
        assert_eq!(self.data_kind, __RegisterDataKind::Box);
        let t = unsafe { *Box::from_raw(self.data.as_ptr as *mut T) };
        std::mem::forget(self);
        t
    }

    pub fn downcast_move<T>(&mut self, target_ty_vtable: &__RegisterTyVTable) -> T
    where
        T: 'eval,
    {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!()
        }
        assert_eq!(self.data_kind, __RegisterDataKind::Box);
        let t = unsafe { *Box::from_raw(self.data.as_ptr as *mut T) };
        self.data_kind = __RegisterDataKind::Moved;
        t
    }

    pub unsafe fn downcast_temp<T>(&mut self, _target_ty_vtable: &__RegisterTyVTable) -> T {
        todo!()
    }

    pub fn downcast_eval_ref<T: 'eval>(&self, target_ty_vtable: &__RegisterTyVTable) -> &'eval T {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!(
                "self is `{:?}` of type `{}`, but target is of type `{}`",
                self.data_kind, self.vtable.typename_str, target_ty_vtable.typename_str
            )
        }
        unsafe {
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::Leash => &*(self.data.as_ptr as *const T),
                __RegisterDataKind::TempRef => panic!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::SomeNone => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            }
        }
    }

    pub fn downcast_opt_eval_ref<T: 'eval>(
        &self,
        target_ty_vtable: &__RegisterTyVTable,
    ) -> Option<&'eval T> {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!()
        }
        unsafe {
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::Leash => Some(&*(self.data.as_ptr as *const T)),
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::SomeNone => {
                    assert_eq!(unsafe { self.data.as_number_of_somes }, 0);
                    None
                }
                __RegisterDataKind::Unreturned => todo!(),
            }
        }
    }

    pub fn downcast_temp_ref<T>(&self, target_ty_vtable: &__RegisterTyVTable) -> &T {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!(
                "expect vtable to be {}, got {} instead",
                target_ty_vtable.typename_str, self.vtable.typename_str
            )
        }
        unsafe {
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box
                | __RegisterDataKind::Leash
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut => &*(self.data.as_ptr as *const T),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::SomeNone => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            }
        }
    }

    pub fn downcast_opt_temp_ref<T>(&self, target_ty_vtable: &__RegisterTyVTable) -> Option<&T> {
        if self.data_kind == __RegisterDataKind::SomeNone {
            assert_eq!(unsafe { self.data.as_number_of_somes }, 0);
            return None;
        }
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!(
                "expect vtable to be {}, got {} instead",
                target_ty_vtable.typename_str, self.vtable.typename_str
            )
        }
        Some(unsafe {
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => todo!(),
                __RegisterDataKind::Box
                | __RegisterDataKind::Leash
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut => &*(self.data.as_ptr as *const T),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::SomeNone => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            }
        })
    }

    pub unsafe fn downcast_temp_mut<T>(
        &mut self,
        _target_ty_vtable: &__RegisterTyVTable,
    ) -> &mut T {
        match self.data_kind {
            __RegisterDataKind::PrimitiveValue => &mut *(&mut self.data as *mut _ as *mut T),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::Leash => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => &mut *(self.data.as_ptr as *mut T),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::SomeNone => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    pub fn number_of_somes_before_none(&self) -> u8 {
        assert_eq!(self.data_kind, __RegisterDataKind::SomeNone);
        unsafe { self.data.as_number_of_somes }
    }

    pub fn is_some(&self) -> bool {
        match self.data_kind() {
            __RegisterDataKind::PrimitiveValue
            | __RegisterDataKind::Box
            | __RegisterDataKind::Leash
            | __RegisterDataKind::TempRef
            | __RegisterDataKind::TempMut => true,
            __RegisterDataKind::SomeNone => self.number_of_somes_before_none() > 0,
            __RegisterDataKind::Moved => panic!(),
            __RegisterDataKind::Unreturned => panic!(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn wrap_in_some(mut self, number_of_somes: u8) -> Self {
        match self.data_kind {
            __RegisterDataKind::SomeNone => {
                self.data = unsafe {
                    __RegisterData {
                        as_number_of_somes: self.number_of_somes_before_none() + number_of_somes,
                    }
                }
            }
            __RegisterDataKind::Moved => panic!(),
            __RegisterDataKind::Unreturned => panic!(),
            _ => (),
        }
        self
    }

    pub fn unveil(self) -> Option<Self> {
        match self.data_kind {
            __RegisterDataKind::SomeNone => {
                let number_of_somes_before_none = self.number_of_somes_before_none();
                if number_of_somes_before_none > 0 {
                    Some(__Register::none(number_of_somes_before_none - 1))
                } else {
                    None
                }
            }
            __RegisterDataKind::Moved => panic!(),
            __RegisterDataKind::Unreturned => panic!(),
            _ => Some(self),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum __RegisterDataKind {
    PrimitiveValue,
    Box,
    Leash,
    TempRef,
    TempMut,
    Moved,
    SomeNone,
    Unreturned,
}

impl<'eval> Drop for __Register<'eval> {
    fn drop(&mut self) {
        match self.data_kind {
            __RegisterDataKind::Box => unsafe {
                (self.vtable.drop)(self.data.as_ptr)
                // (*std::mem::replace(&mut self.data, __RegisterData { as_opt_ptr: None })
                //     .as_opt_ptr
                //     .unwrap())
                // .__drop_dyn__()
            },
            _ => (),
        }
    }
}

#[macro_export]
macro_rules! register_new_copyable {
    (Intrinsic, Direct, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        ($argument).to_register()
    }};
    (Optional, Direct, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        ($argument).to_register()
    }};
    (Leash, Direct, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        todo!()
    }};
    (OptionalLeash, Direct, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        todo!()
    }};
    (Intrinsic, BoxCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        __Register::new_box::<$INTRINSIC_FIELD_TY>($argument, &$TYPE_VTABLE)
    }};
    (Optional, BoxCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        __Register::new_opt_box::<$INTRINSIC_FIELD_TY>($argument, &$TYPE_VTABLE)
    }};
    (Leash, BoxCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        todo!()
    }};
    (OptionalLeash, BoxCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        todo!()
    }};
    (Intrinsic, BoxNonCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        panic!("register_new_copyable invalid")
    }};
    (Optional, BoxNonCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        panic!("register_new_copyable invalid")
    }};
    (Leash, BoxNonCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        panic!("register_new_copyable invalid")
    }};
    (OptionalLeash, BoxNonCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        panic!("register_new_copyable invalid")
    }};
}
