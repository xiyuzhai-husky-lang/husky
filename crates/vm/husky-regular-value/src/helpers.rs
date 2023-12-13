use crate::*;

impl RegularValue {
    pub unsafe fn to_bool_in_condition(self) -> bool {
        match self {
            RegularValue::Bool(val) => val,
            RegularValue::Char(val) => val != Default::default(),
            RegularValue::I8(val) => val != 0,
            RegularValue::I16(val) => val != 0,
            RegularValue::I32(val) => val != 0,
            RegularValue::I64(val) => val != 0,
            RegularValue::I128(val) => val != 0,
            RegularValue::ISize(val) => val != 0,
            RegularValue::U8(val) => val != 0,
            RegularValue::U16(val) => val != 0,
            RegularValue::U32(val) => val != 0,
            RegularValue::U64(val) => val != 0,
            RegularValue::U128(val) => val != 0,
            RegularValue::USize(val) => val != 0,
            RegularValue::R8(val) => val != 0,
            RegularValue::R16(val) => val != 0,
            RegularValue::R32(val) => val != 0,
            RegularValue::R64(val) => val != 0,
            RegularValue::R128(val) => val != 0,
            RegularValue::RSize(val) => val != 0,
            _ => unreachable!(),
        }
    }
}
