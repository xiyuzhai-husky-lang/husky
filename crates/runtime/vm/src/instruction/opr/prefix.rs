use super::*;

impl From<PrefixOpr> for Opr {
    fn from(prefix: PrefixOpr) -> Self {
        Self::Prefix(prefix)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrefixOpr {
    Minus,     // -
    Not,       // !$0
    BitNot,    // ~
    Shared,    // &
    Exclusive, // !$0 after WithType or Vec or Array
}

impl PrefixOpr {
    pub fn act_on_primitive(&self, opd: PrimitiveValue) -> PrimitiveValue {
        match self {
            PrefixOpr::Minus => match opd {
                PrimitiveValue::I32(_) => todo!(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::Void => todo!(),
            },
            PrefixOpr::Not => match opd {
                PrimitiveValue::I32(i) => i == 0,
                PrimitiveValue::F32(f) => f == 0.,
                PrimitiveValue::B32(b) => b == 0,
                PrimitiveValue::B64(b) => b == 0,
                PrimitiveValue::Bool(b) => !b,
                PrimitiveValue::Void => panic!(),
            }
            .into(),
            PrefixOpr::BitNot => todo!(),
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Exclusive => todo!(),
        }
    }
}
