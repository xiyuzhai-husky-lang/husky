use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemBinaryOpr {
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
    Of,
}

impl SemBinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            SemBinaryOpr::Closed(opr) => opr.husky_code(),
            SemBinaryOpr::Shift(opr) => opr.husky_code(),
            SemBinaryOpr::Assign => "=",
            SemBinaryOpr::AssignClosed(opr) => match opr {
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
            SemBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => "<<=",
                BinaryShiftOpr::Shr => ">>=",
            },
            SemBinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            SemBinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
            SemBinaryOpr::CurryType => "->",
            SemBinaryOpr::As => "as",
            SemBinaryOpr::Of => "of",
            SemBinaryOpr::Ins => todo!(),
            SemBinaryOpr::ScopeResolution => todo!(),
            SemBinaryOpr::In => "in",
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            SemBinaryOpr::Closed(opr) => opr.spaced_husky_code(),
            SemBinaryOpr::Shift(opr) => opr.spaced_husky_code(),
            SemBinaryOpr::Comparison(opr) => opr.spaced_husky_code(),
            SemBinaryOpr::ShortCircuitLogic(opr) => opr.spaced_husky_code(),
            SemBinaryOpr::Assign => " = ",
            SemBinaryOpr::AssignClosed(opr) => match opr {
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
            SemBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => " <<= ",
                BinaryShiftOpr::Shr => " >>= ",
            },
            SemBinaryOpr::CurryType => " -> ",
            SemBinaryOpr::As => " as ",
            SemBinaryOpr::Of => " of ",
            SemBinaryOpr::Ins => " : ",
            SemBinaryOpr::ScopeResolution => " :: ",
            SemBinaryOpr::In => " in ",
        }
    }
}
