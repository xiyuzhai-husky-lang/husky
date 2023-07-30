use crate::*;

impl __RegularValue {
    pub unsafe fn to_bool_in_condition(self) -> bool {
        match self {
            __RegularValue::Bool(val) => val,
            __RegularValue::Char(val) => val != Default::default(),
            __RegularValue::I8(val) => val != 0,
            __RegularValue::I16(val) => val != 0,
            __RegularValue::I32(val) => val != 0,
            __RegularValue::I64(val) => val != 0,
            __RegularValue::I128(val) => val != 0,
            __RegularValue::ISize(val) => val != 0,
            __RegularValue::U8(val) => val != 0,
            __RegularValue::U16(val) => val != 0,
            __RegularValue::U32(val) => val != 0,
            __RegularValue::U64(val) => val != 0,
            __RegularValue::U128(val) => val != 0,
            __RegularValue::USize(val) => val != 0,
            __RegularValue::R8(val) => val != 0,
            __RegularValue::R16(val) => val != 0,
            __RegularValue::R32(val) => val != 0,
            __RegularValue::R64(val) => val != 0,
            __RegularValue::R128(val) => val != 0,
            __RegularValue::RSize(val) => val != 0,
            _ => unreachable!(),
        }
    }
}
