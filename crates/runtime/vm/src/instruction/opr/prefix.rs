use super::*;

impl From<PrefixOpr> for Opr {
    fn from(prefix: PrefixOpr) -> Self {
        Self::Prefix(prefix)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrefixOpr {
    Minus,  // -
    Not,    // !$0
    BitNot, // ~
    Shared, // &
    Move,   // !$0 after WithType or Vec or Array
}

impl PrefixOpr {
    pub fn act_on_primitive(&self, opd: CopyableValue) -> CopyableValue {
        match self {
            PrefixOpr::Minus => match opd {
                CopyableValue::I32(i) => (-i).into(),
                CopyableValue::F32(f) => (-f).into(),
                CopyableValue::B32(_) => todo!(),
                CopyableValue::B64(_) => todo!(),
                CopyableValue::Bool(_) => todo!(),
                CopyableValue::Void(_) => panic!(),
                CopyableValue::EnumKind(_) => panic!(),
            },
            PrefixOpr::Not => match opd {
                CopyableValue::I32(i) => i == 0,
                CopyableValue::F32(f) => f == 0.,
                CopyableValue::B32(b) => b == 0,
                CopyableValue::B64(b) => b == 0,
                CopyableValue::Bool(b) => !b,
                CopyableValue::Void(_) => panic!(),
                CopyableValue::EnumKind(_) => panic!(),
            }
            .into(),
            PrefixOpr::BitNot => match opd {
                CopyableValue::B32(b) => (!b).into(),
                CopyableValue::B64(b) => (!b).into(),
                _ => panic!(),
            },
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Move => todo!(),
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            PrefixOpr::Minus => "-",
            PrefixOpr::Not => "!",
            PrefixOpr::BitNot => "~",
            PrefixOpr::Shared => "&",
            PrefixOpr::Move => "!!",
        }
    }
}
