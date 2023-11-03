mod comparison;
mod logic;
mod pure_closed;
mod shift;

pub use self::comparison::*;
pub use self::logic::*;
pub use self::pure_closed::*;
pub use self::shift::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SynBinaryOpr {
    Closed(BinaryClosedOpr),
    Shift(BinaryShiftOpr),
    Assign,
    AssignClosed(BinaryClosedOpr),
    AssignShift(BinaryShiftOpr),
    Comparison(BinaryComparisonOpr),
    ShortCircuitLogic(BinaryShortcuitLogicOpr),
    ScopeResolution,
    Curry, // ->
    As,    // as
    Ins,   // :
    In,    // in
}

impl From<BinaryComparisonOpr> for SynBinaryOpr {
    fn from(v: BinaryComparisonOpr) -> Self {
        Self::Comparison(v)
    }
}

impl SynBinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            SynBinaryOpr::Closed(opr) => opr.husky_code(),
            SynBinaryOpr::Shift(opr) => opr.husky_code(),
            SynBinaryOpr::Assign => "=",
            SynBinaryOpr::AssignClosed(opr) => match opr {
                BinaryClosedOpr::Add => "+=",
                BinaryClosedOpr::BitAnd => "&=",
                BinaryClosedOpr::BitOr => "|=",
                BinaryClosedOpr::BitXor => "^=",
                BinaryClosedOpr::Div => "/=",
                BinaryClosedOpr::Mul => "*=",
                BinaryClosedOpr::RemEuclid => "%=",
                BinaryClosedOpr::Power => "**=",
                BinaryClosedOpr::Sub => "-=",
            },
            SynBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => "<<=",
                BinaryShiftOpr::Shr => ">>=",
            },
            SynBinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            SynBinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
            SynBinaryOpr::Curry => "->",
            SynBinaryOpr::As => todo!(),
            SynBinaryOpr::Ins => todo!(),
            SynBinaryOpr::ScopeResolution => todo!(),
            SynBinaryOpr::In => "in",
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            SynBinaryOpr::Closed(opr) => opr.spaced_husky_code(),
            SynBinaryOpr::Shift(opr) => opr.spaced_husky_code(),
            SynBinaryOpr::Comparison(opr) => opr.spaced_husky_code(),
            SynBinaryOpr::ShortCircuitLogic(opr) => opr.spaced_husky_code(),
            SynBinaryOpr::Assign => " = ",
            SynBinaryOpr::AssignClosed(opr) => match opr {
                BinaryClosedOpr::Add => " += ",
                BinaryClosedOpr::BitAnd => " &= ",
                BinaryClosedOpr::BitOr => " |= ",
                BinaryClosedOpr::BitXor => " ^= ",
                BinaryClosedOpr::Div => " /= ",
                BinaryClosedOpr::Mul => " *= ",
                BinaryClosedOpr::RemEuclid => " %= ",
                BinaryClosedOpr::Power => " **= ",
                BinaryClosedOpr::Sub => " -= ",
            },
            SynBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => " <<= ",
                BinaryShiftOpr::Shr => " >>= ",
            },
            SynBinaryOpr::Curry => " -> ",
            SynBinaryOpr::As => " as ",
            SynBinaryOpr::Ins => " : ",
            SynBinaryOpr::ScopeResolution => " :: ",
            SynBinaryOpr::In => " in ",
        }
    }
}
