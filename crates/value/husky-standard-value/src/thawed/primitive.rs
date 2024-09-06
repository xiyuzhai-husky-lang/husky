use super::*;

macro_rules! impl_thawed_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Thawed for $primitive_ty {
            type Frozen = Self;

            fn is_copyable() -> bool {
                true
            }

            fn try_copy_thawed(&self) -> Option<ThawedValue> {
                Some((*self).into())
            }

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                serde_json::to_value(self).unwrap()
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!("")
            }
        }
    };
}

for_all_primitive_tys!(impl_thawed_for_primitive_ty);

impl From<()> for ThawedValue {
    fn from(value: ()) -> Self {
        ThawedValue::Unit(())
    }
}

impl Into<()> for ThawedValue {
    fn into(self) -> () {
        match self {
            ThawedValue::Unit(()) => (),
            _ => {
                println!("self = {:?}", self);
                unreachable!()
            }
        }
    }
}

impl From<bool> for ThawedValue {
    fn from(value: bool) -> Self {
        ThawedValue::Bool(value)
    }
}

impl Into<bool> for ThawedValue {
    fn into(self) -> bool {
        match self {
            ThawedValue::Bool(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for ThawedValue {
    fn from(value: u8) -> Self {
        ThawedValue::U8(value)
    }
}

impl Into<u8> for ThawedValue {
    fn into(self) -> u8 {
        match self {
            ThawedValue::U8(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u16> for ThawedValue {
    fn from(value: u16) -> Self {
        ThawedValue::U16(value)
    }
}

impl Into<u16> for ThawedValue {
    fn into(self) -> u16 {
        match self {
            ThawedValue::U16(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for ThawedValue {
    fn from(value: u32) -> Self {
        ThawedValue::U32(value)
    }
}

impl Into<u32> for ThawedValue {
    fn into(self) -> u32 {
        match self {
            ThawedValue::U32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u64> for ThawedValue {
    fn from(value: u64) -> Self {
        ThawedValue::U64(value)
    }
}

impl Into<u64> for ThawedValue {
    fn into(self) -> u64 {
        match self {
            ThawedValue::U64(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<u128> for ThawedValue {
    fn from(value: u128) -> Self {
        ThawedValue::U128(value)
    }
}

impl Into<u128> for ThawedValue {
    fn into(self) -> u128 {
        match self {
            ThawedValue::U128(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for ThawedValue {
    fn from(value: usize) -> Self {
        ThawedValue::USize(value)
    }
}

impl Into<usize> for ThawedValue {
    fn into(self) -> usize {
        match self {
            ThawedValue::USize(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i8> for ThawedValue {
    fn from(value: i8) -> Self {
        ThawedValue::I8(value)
    }
}

impl Into<i8> for ThawedValue {
    fn into(self) -> i8 {
        match self {
            ThawedValue::I8(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i16> for ThawedValue {
    fn from(value: i16) -> Self {
        ThawedValue::I16(value)
    }
}

impl Into<i16> for ThawedValue {
    fn into(self) -> i16 {
        match self {
            ThawedValue::I16(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i32> for ThawedValue {
    fn from(value: i32) -> Self {
        ThawedValue::I32(value)
    }
}

impl Into<i32> for ThawedValue {
    fn into(self) -> i32 {
        match self {
            ThawedValue::I32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for ThawedValue {
    fn from(value: i64) -> Self {
        ThawedValue::I64(value)
    }
}

impl Into<i64> for ThawedValue {
    fn into(self) -> i64 {
        match self {
            ThawedValue::I64(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<i128> for ThawedValue {
    fn from(value: i128) -> Self {
        ThawedValue::I128(value)
    }
}

impl Into<i128> for ThawedValue {
    fn into(self) -> i128 {
        match self {
            ThawedValue::I128(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<isize> for ThawedValue {
    fn from(value: isize) -> Self {
        ThawedValue::ISize(value)
    }
}

impl Into<isize> for ThawedValue {
    fn into(self) -> isize {
        match self {
            ThawedValue::ISize(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<f32> for ThawedValue {
    fn from(value: f32) -> Self {
        ThawedValue::F32(value)
    }
}

impl Into<f32> for ThawedValue {
    fn into(self) -> f32 {
        match self {
            ThawedValue::F32(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<f64> for ThawedValue {
    fn from(value: f64) -> Self {
        ThawedValue::F64(value)
    }
}

impl Into<f64> for ThawedValue {
    fn into(self) -> f64 {
        match self {
            ThawedValue::F64(value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<char> for ThawedValue {
    fn from(value: char) -> Self {
        ThawedValue::Char(value)
    }
}

impl Into<char> for ThawedValue {
    fn into(self) -> char {
        match self {
            ThawedValue::Char(value) => value,
            _ => unreachable!(),
        }
    }
}
