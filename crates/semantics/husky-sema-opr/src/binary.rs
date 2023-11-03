use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemaBinaryOpr {
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

impl SemaBinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            SemaBinaryOpr::Closed(opr) => opr.husky_code(),
            SemaBinaryOpr::Shift(opr) => opr.husky_code(),
            SemaBinaryOpr::Assign => "=",
            SemaBinaryOpr::AssignClosed(opr) => match opr {
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
            SemaBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => "<<=",
                BinaryShiftOpr::Shr => ">>=",
            },
            SemaBinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            SemaBinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
            SemaBinaryOpr::CurryType => "->",
            SemaBinaryOpr::As => todo!(),
            SemaBinaryOpr::Ins => todo!(),
            SemaBinaryOpr::ScopeResolution => todo!(),
            SemaBinaryOpr::In => "in",
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            SemaBinaryOpr::Closed(opr) => opr.spaced_husky_code(),
            SemaBinaryOpr::Shift(opr) => opr.spaced_husky_code(),
            SemaBinaryOpr::Comparison(opr) => opr.spaced_husky_code(),
            SemaBinaryOpr::ShortCircuitLogic(opr) => opr.spaced_husky_code(),
            SemaBinaryOpr::Assign => " = ",
            SemaBinaryOpr::AssignClosed(opr) => match opr {
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
            SemaBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => " <<= ",
                BinaryShiftOpr::Shr => " >>= ",
            },
            SemaBinaryOpr::CurryType => " -> ",
            SemaBinaryOpr::As => " as ",
            SemaBinaryOpr::Ins => " : ",
            SemaBinaryOpr::ScopeResolution => " :: ",
            SemaBinaryOpr::In => " in ",
        }
    }
}
