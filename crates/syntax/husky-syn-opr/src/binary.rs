// mod comparison;
// mod logic;
// mod pure_closed;
// mod shift;

use husky_opr::precedence::{HasPrecedence, Precedence};

// pub use self::comparison::*;
// pub use self::logic::*;
// pub use self::pure_closed::*;
// pub use self::shift::*;
use crate::*;

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
    CurryType, // ->
    As,        // as
    Ins,       // :
    In,        // in
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
            SynBinaryOpr::CurryType => "->",
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
            SynBinaryOpr::CurryType => " -> ",
            SynBinaryOpr::As => " as ",
            SynBinaryOpr::Ins => " : ",
            SynBinaryOpr::ScopeResolution => " :: ",
            SynBinaryOpr::In => " in ",
        }
    }
}

impl HasPrecedence for SynBinaryOpr {
    fn precedence(self) -> precedence::Precedence {
        match self {
            SynBinaryOpr::Closed(opr) => opr.precedence(),
            SynBinaryOpr::Shift(opr) => match opr {
                BinaryShiftOpr::Shl | BinaryShiftOpr::Shr => Precedence::Shift,
            },
            SynBinaryOpr::Comparison(opr) => opr.precedence(),
            SynBinaryOpr::Assign | SynBinaryOpr::AssignClosed(_) | SynBinaryOpr::AssignShift(_) => {
                Precedence::Assign
            }
            SynBinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            SynBinaryOpr::CurryType => Precedence::Curry,
            SynBinaryOpr::As => Precedence::As,
            SynBinaryOpr::Ins => Precedence::Is,
            SynBinaryOpr::In => todo!(),
            SynBinaryOpr::ShortCircuitLogic(opr) => opr.precedence(),
        }
    }
}
