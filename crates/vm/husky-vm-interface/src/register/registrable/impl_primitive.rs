use ordered_float::NotNan;

use crate::*;

impl __StaticInfo for i32 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for i32 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_i32: self }, &__I32_VTABLE)
    }
}

impl __StaticInfo for i64 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "i64".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for i64 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_i64: self }, &__I64_VTABLE)
    }
}

impl __StaticInfo for () {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "void".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for () {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_void: () }, &__VOID_VTABLE)
    }
}

impl __StaticInfo for f32 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for f32 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(
            __RegisterData {
                as_not_nan_f32: NotNan::new(self).unwrap(),
            },
            &__F32_VTABLE,
        )
    }
}

impl __StaticInfo for f64 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "f64".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for f64 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_f64: self }, &__F64_VTABLE)
    }
}

impl __StaticInfo for u32 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for u32 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_r32: self }, &__B32_VTABLE)
    }
}

impl __StaticInfo for u64 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for u64 {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_b64: self }, &__B64_VTABLE)
    }
}

impl __StaticInfo for bool {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl __Registrable for bool {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_primitive_value(__RegisterData { as_bool: self }, &__BOOL_VTABLE)
    }
}

// pub trait __AsU64: Copy {
//     fn __as_u64(self) -> u64 {
//         panic!()
//     }
// }

// impl __AsU64 for bool {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for i32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for i64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for u32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for u64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for f32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for f64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }
