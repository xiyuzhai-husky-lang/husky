use husky_opr::*;
use husky_sema_opr::binary::SemaBinaryOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirBinaryOpr {
    Closed(BinaryClosedOpr),
    Shift(BinaryShiftOpr),
    Assign,
    AssignClosed(BinaryClosedOpr),
    AssignShift(BinaryShiftOpr),
    Comparison(BinaryComparisonOpr),
    ShortCircuitLogic(BinaryShortcuitLogicOpr),
}

impl HirBinaryOpr {
    pub fn from_sema(opr: SemaBinaryOpr) -> Self {
        match opr {
            SemaBinaryOpr::Closed(opr) => HirBinaryOpr::Closed(opr),
            SemaBinaryOpr::Shift(opr) => HirBinaryOpr::Shift(opr),
            SemaBinaryOpr::Assign => HirBinaryOpr::Assign,
            SemaBinaryOpr::AssignClosed(opr) => HirBinaryOpr::AssignClosed(opr),
            SemaBinaryOpr::AssignShift(opr) => HirBinaryOpr::AssignShift(opr),
            SemaBinaryOpr::Comparison(opr) => HirBinaryOpr::Comparison(opr),
            SemaBinaryOpr::ShortCircuitLogic(opr) => HirBinaryOpr::ShortCircuitLogic(opr),

            // For the variants without a direct match, you will need to decide the appropriate action.
            SemaBinaryOpr::ScopeResolution => {
                panic!("ScopeResolution is not supported in HirBinaryOpr")
            }
            SemaBinaryOpr::CurryType => {
                panic!("CurryType is not supported in HirBinaryOpr")
            }
            // Ins and In do not have an equivalent in HirBinaryOpr
            // In a real scenario, consider proper error handling instead of panicking.
            SemaBinaryOpr::Ins => {
                panic!("Ins operator is not expected in HirBinaryOpr")
            }
            SemaBinaryOpr::In => {
                panic!("In operator is not expected in HirBinaryOpr")
            }
            SemaBinaryOpr::As => {
                panic!("As is not considered as an operator in HirBinaryOpr")
            }
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            HirBinaryOpr::Closed(opr) => opr.husky_code(),
            HirBinaryOpr::Shift(opr) => opr.husky_code(),
            HirBinaryOpr::Assign => "=",
            HirBinaryOpr::AssignClosed(opr) => match opr {
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
            HirBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => "<<=",
                BinaryShiftOpr::Shr => ">>=",
            },
            HirBinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            HirBinaryOpr::ShortCircuitLogic(logic_opr) => logic_opr.husky_code(),
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            HirBinaryOpr::Closed(opr) => opr.spaced_husky_code(),
            HirBinaryOpr::Shift(opr) => opr.spaced_husky_code(),
            HirBinaryOpr::Comparison(opr) => opr.spaced_husky_code(),
            HirBinaryOpr::ShortCircuitLogic(opr) => opr.spaced_husky_code(),
            HirBinaryOpr::Assign => " = ",
            HirBinaryOpr::AssignClosed(opr) => match opr {
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
            HirBinaryOpr::AssignShift(opr) => match opr {
                BinaryShiftOpr::Shl => " <<= ",
                BinaryShiftOpr::Shr => " >>= ",
            },
        }
    }
}
