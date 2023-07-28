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
pub struct __RegularValue {
    pub(crate) data_kind: __RegisterDataKind,
    pub(crate) data: __RegisterData,
    pub vtable: &'static __RegisterTyVTable,
}

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
pub enum __RegularValueData {
    Box(RawHirType, Box<*mut c_void>),
    Leash(RawHirType, Leash),
    Ref(RawHirType, Ref),
    OptionBox(RawHirType, Option<Box<*mut c_void>>),
    OptionLeash(RawHirType, Option<Leash>),
    OptionRef(RawHirType, Option<Ref>),
    Unit(RawHirType, ()),
    Bool(RawHirType, bool),
    I8(RawHirType, i8),
    I16(RawHirType, i16),
    I32(RawHirType, i32),
    I64(RawHirType, i64),
    ISize(RawHirType, isize),
    U32(RawHirType, u32),
    U64(RawHirType, u64),
    USize(RawHirType, usize),
    R32(RawHirType, u32),
    R64(RawHirType, u64),
    RSize(RawHirType, usize),
}

#[test]
fn regular_value_layout_works() {
    fn t(v: __RegularValueData) {
        assert_eq!(v.raw_hir_ty(), v.raw_hir_ty_safe())
    }
    t(__RegularValueData::I8(RawHirType(215), 12));
    t(__RegularValueData::I16(RawHirType(215), 12));
    t(__RegularValueData::I32(RawHirType(215), 12));
    t(__RegularValueData::I64(RawHirType(215), 12));
    t(__RegularValueData::ISize(RawHirType(215), 12));
}

impl __RegularValueData {
    // todo: write tests for check that this agrees with raw_hir_ty_safe
    pub fn raw_hir_ty(&self) -> RawHirType {
        type Array = [u32; 4];
        let this = self as *const _ as *const Array;
        unsafe {
            let this: &Array = &*this;
            RawHirType(this[1])
        }
    }

    fn raw_hir_ty_safe(&self) -> RawHirType {
        match self {
            __RegularValueData::Box(raw_hir_ty, _)
            | __RegularValueData::Leash(raw_hir_ty, _)
            | __RegularValueData::Ref(raw_hir_ty, _)
            | __RegularValueData::OptionBox(raw_hir_ty, _)
            | __RegularValueData::OptionLeash(raw_hir_ty, _)
            | __RegularValueData::OptionRef(raw_hir_ty, _)
            | __RegularValueData::Unit(raw_hir_ty, _)
            | __RegularValueData::Bool(raw_hir_ty, _)
            | __RegularValueData::I8(raw_hir_ty, _)
            | __RegularValueData::I16(raw_hir_ty, _)
            | __RegularValueData::I32(raw_hir_ty, _)
            | __RegularValueData::I64(raw_hir_ty, _)
            | __RegularValueData::ISize(raw_hir_ty, _)
            | __RegularValueData::U32(raw_hir_ty, _)
            | __RegularValueData::U64(raw_hir_ty, _)
            | __RegularValueData::USize(raw_hir_ty, _)
            | __RegularValueData::R32(raw_hir_ty, _)
            | __RegularValueData::R64(raw_hir_ty, _)
            | __RegularValueData::RSize(raw_hir_ty, _) => *raw_hir_ty,
        }
    }
}

#[test]
fn regular_value_data_size_works() {
    assert_eq!(
        std::mem::size_of::<__RegularValueData>(),
        2 * std::mem::size_of::<usize>()
    )
}

// interface for `HirType` defined in `husky-hir-ty`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RawHirType(u32);

pub struct Leash(&'static c_void);

pub struct Ref(&'static c_void);

impl std::hash::Hash for __RegularValue {
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

unsafe impl Send for __RegularValue {}
unsafe impl Sync for __RegularValue {}

impl std::fmt::Debug for __RegularValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("__RegularValue")
    }
}

#[cfg(feature = "extra")]
impl Clone for __RegularValue {
    fn clone(&self) -> Self {
        unsafe { self.extrinsic_clone() }
    }
}

impl PartialEq for __RegularValue {
    fn eq(&self, other: &Self) -> bool {
        self.vtable == other.vtable && unsafe { (self.vtable.eq)(self.any_ref(), other.any_ref()) }
    }
}

impl Eq for __RegularValue {}

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

impl __RegularValue {
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
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        __RegularValue {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data,
            vtable: proto,
        }
    }

    pub fn new_opt_box<T>(
        opt_value: Option<T>,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        if let Some(value) = opt_value {
            let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
            __RegularValue {
                data_kind: __RegisterDataKind::Box,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __RegularValue::none(0)
        }
    }

    pub fn new_box<T>(value: T, proto: &'static __RegisterTyVTable) -> __RegularValue {
        let ptr: *mut T = Box::<T>::into_raw(Box::new(value));
        __RegularValue {
            data_kind: __RegisterDataKind::Box,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub fn new_leash<T: 'static>(
        value: &'static T,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        let ptr: *const T = value;
        __RegularValue {
            data_kind: __RegisterDataKind::Leash,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub fn new_opt_leash<T: 'static>(
        opt_value: Option<&'static T>,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        if let Some(value) = opt_value {
            Self::new_leash(value, proto)
        } else {
            Self::none(0)
        }
    }

    pub unsafe fn new_temp_ref<T>(value: &T, proto: &'static __RegisterTyVTable) -> __RegularValue {
        let ptr: *const T = value;
        __RegularValue {
            data_kind: __RegisterDataKind::TempRef,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_opt_temp_ref<T>(
        opt_value: Option<&T>,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        if let Some(value) = opt_value {
            let ptr: *const T = value;
            __RegularValue {
                data_kind: __RegisterDataKind::TempRef,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __RegularValue::none(0)
        }
    }

    pub unsafe fn new_temp_mut<T>(
        value: &mut T,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        let ptr: *const T = value;
        __RegularValue {
            data_kind: __RegisterDataKind::TempMut,
            data: __RegisterData {
                as_ptr: ptr as *mut c_void,
            },
            vtable: proto,
        }
    }

    pub unsafe fn new_opt_temp_mut<T>(
        opt_value: Option<&mut T>,
        proto: &'static __RegisterTyVTable,
    ) -> __RegularValue {
        if let Some(value) = opt_value {
            let ptr: *const T = value;
            __RegularValue {
                data_kind: __RegisterDataKind::TempMut,
                data: __RegisterData {
                    as_ptr: ptr as *mut c_void,
                },
                vtable: proto,
            }
        } else {
            __RegularValue::none(0)
        }
    }

    pub fn register_move(&mut self) -> __RegularValue {
        let moved = __RegularValue {
            data_kind: __RegisterDataKind::Moved,
            data: __RegisterData { as_void: () },
            vtable: self.vtable,
        };
        std::mem::replace(self, moved)
    }

    pub fn none(number_of_somes_before_none: u8) -> __RegularValue {
        __RegularValue {
            data_kind: __RegisterDataKind::SomeNone,
            data: __RegisterData {
                as_number_of_somes: number_of_somes_before_none,
            },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn new_void() -> __RegularValue {
        __RegularValue {
            data_kind: __RegisterDataKind::PrimitiveValue,
            data: __RegisterData { as_void: () },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn unreturned() -> __RegularValue {
        __RegularValue {
            data_kind: __RegisterDataKind::Unreturned,
            data: __RegisterData { as_void: () },
            vtable: &__VOID_VTABLE,
        }
    }

    pub fn new_moved(vtable: &'static __RegisterTyVTable) -> __RegularValue {
        __RegularValue {
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

    pub fn downcast_unbox<T>(self, target_ty_vtable: &__RegisterTyVTable) -> T {
        if self.vtable.typename_str_hash_u64 != target_ty_vtable.typename_str_hash_u64 {
            panic!()
        }
        assert_eq!(self.data_kind, __RegisterDataKind::Box);
        let t = unsafe { *Box::from_raw(self.data.as_ptr as *mut T) };
        std::mem::forget(self);
        t
    }

    pub fn downcast_move<T>(&mut self, target_ty_vtable: &__RegisterTyVTable) -> T {
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

    pub fn downcast_leash<T: 'static>(&self, target_ty_vtable: &__RegisterTyVTable) -> &'static T {
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

    pub fn downcast_opt_leash<T: 'static>(
        &self,
        target_ty_vtable: &__RegisterTyVTable,
    ) -> Option<&'static T> {
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
                    Some(__RegularValue::none(number_of_somes_before_none - 1))
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

impl Drop for __RegularValue {
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
        __RegularValue::new_box::<$INTRINSIC_FIELD_TY>($argument, &$TYPE_VTABLE)
    }};
    (Optional, BoxCopyable, $argument: expr, $INTRINSIC_FIELD_TY: ty, $TYPE_VTABLE: expr) => {{
        __RegularValue::new_opt_box::<$INTRINSIC_FIELD_TY>($argument, &$TYPE_VTABLE)
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
