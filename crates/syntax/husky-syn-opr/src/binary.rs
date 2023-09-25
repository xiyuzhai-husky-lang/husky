mod comparison;
mod logic;
mod pure_closed;
mod shift;

pub use self::comparison::*;
pub use self::logic::*;
pub use self::pure_closed::*;
pub use self::shift::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryOpr {
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

impl From<BinaryComparisonOpr> for BinaryOpr {
    fn from(v: BinaryComparisonOpr) -> Self {
        Self::Comparison(v)
    }
}

impl BinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            BinaryOpr::Closed(opr) => opr.husky_code(),
            BinaryOpr::Shift(opr) => opr.husky_code(),
            BinaryOpr::Assign => "=",
            BinaryOpr::AssignClosed(opr) => match opr {
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
            BinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => "<<=",
                BinaryShiftOpr::Shr => ">>=",
            },
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            BinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
            BinaryOpr::Curry => "->",
            BinaryOpr::As => todo!(),
            BinaryOpr::Ins => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::In => "in",
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::Closed(opr) => opr.spaced_husky_code(),
            BinaryOpr::Shift(opr) => opr.spaced_husky_code(),
            BinaryOpr::Comparison(opr) => opr.spaced_husky_code(),
            BinaryOpr::ShortCircuitLogic(opr) => opr.spaced_husky_code(),
            BinaryOpr::Assign => " = ",
            BinaryOpr::AssignClosed(opr) => match opr {
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
            BinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => " <<= ",
                BinaryShiftOpr::Shr => " >>= ",
            },
            BinaryOpr::Curry => " -> ",
            BinaryOpr::As => " as ",
            BinaryOpr::Ins => " : ",
            BinaryOpr::ScopeResolution => " :: ",
            BinaryOpr::In => " in ",
        }
    }
}
