use crate::*;

impl __StaticInfo for i32 {
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl __Registrable for i32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => {
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
}

impl __StaticInfo for () {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl __Registrable for () {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(0)
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => ().into(),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => todo!(),
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
        }
    }
}

impl __StaticInfo for f32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl __Registrable for f32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => {
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
}

impl __StaticInfo for u32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl __Registrable for u32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => {
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
}

impl __StaticInfo for u64 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl __Registrable for u64 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self)
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => {
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
}

impl __StaticInfo for bool {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl __Registrable for bool {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }

    fn __primitive(&self, data_kind: __RegisterDataKind) -> PrimitiveValueData {
        match data_kind {
            __RegisterDataKind::Value => {
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
