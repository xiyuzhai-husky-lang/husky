use crate::*;

impl __StaticInfo for i32 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl __Registrable for i32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_i32: self }, &__I32_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const i32 = self;
                let data = ptr as i32;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for i64 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "i64".into()
    }
}

impl __Registrable for i64 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_i64: self }, &__I64_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const Self = self;
                let data = ptr as Self;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for () {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl __Registrable for () {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_void: () }, &__VOID_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => ().into(),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for f32 {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl __Registrable for f32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_f32: self }, &__F32_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const Self = self;
                let data = ptr as u64 as Self;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for f64 {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "f64".into()
    }
}

impl __Registrable for f64 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_f64: self }, &__F64_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        todo!()
        // match data_kind {
        //     __RegisterDataKind::PrimitiveValue => {
        //         let ptr: *const Self = self;
        //         let data = ptr as u64 as Self;
        //         data.into()
        //     }
        //     __RegisterDataKind::Box => todo!(),
        //     __RegisterDataKind::EvalRef => todo!(),
        //     __RegisterDataKind::TempRef => todo!(),
        //     __RegisterDataKind::TempMut => todo!(),
        //     __RegisterDataKind::Moved => todo!(),
        //     __RegisterDataKind::Undefined => todo!(),
        //     __RegisterDataKind::Unreturned => todo!(),
        // }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for u32 {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl __Registrable for u32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_b32: self }, &__B32_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const Self = self;
                let data = ptr as Self;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for u64 {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl __Registrable for u64 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_b64: self }, &__B64_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const Self = self;
                let data = ptr as Self;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
    }
}

impl __StaticInfo for bool {
    type __StaticSelf = Self;
    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl __Registrable for bool {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_primitive_value::<Self>(__RegisterData { as_bool: self }, &__BOOL_VTABLE)
    }

    fn __primitive__(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::PrimitiveValue => {
                let ptr: *const Self = self;
                let data = (ptr as u64) != 0;
                data.into()
            }
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }

    fn __copy__(&self) -> Self {
        *self
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
