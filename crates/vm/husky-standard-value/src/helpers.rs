use crate::*;

impl Value {
    pub unsafe fn to_bool_in_condition(self) -> bool {
        match self {
            Value::Bool(val) => val,
            Value::Char(val) => val != Default::default(),
            Value::I8(val) => val != 0,
            Value::I16(val) => val != 0,
            Value::I32(val) => val != 0,
            Value::I64(val) => val != 0,
            Value::I128(val) => val != 0,
            Value::ISize(val) => val != 0,
            Value::U8(val) => val != 0,
            Value::U16(val) => val != 0,
            Value::U32(val) => val != 0,
            Value::U64(val) => val != 0,
            Value::U128(val) => val != 0,
            Value::USize(val) => val != 0,
            Value::R8(val) => val != 0,
            Value::R16(val) => val != 0,
            Value::R32(val) => val != 0,
            Value::R64(val) => val != 0,
            Value::R128(val) => val != 0,
            Value::RSize(val) => val != 0,
            _ => unreachable!(),
        }
    }
}
