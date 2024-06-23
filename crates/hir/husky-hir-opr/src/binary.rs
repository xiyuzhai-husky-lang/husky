use husky_opr::*;
use husky_sem_opr::binary::SemBinaryOpr;

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
    pub fn from_sema(opr: SemBinaryOpr) -> Self {
        match opr {
            SemBinaryOpr::Closed(opr) => HirBinaryOpr::Closed(opr),
            SemBinaryOpr::Shift(opr) => HirBinaryOpr::Shift(opr),
            SemBinaryOpr::Assign => HirBinaryOpr::Assign,
            SemBinaryOpr::AssignClosed(opr) => HirBinaryOpr::AssignClosed(opr),
            SemBinaryOpr::AssignShift(opr) => HirBinaryOpr::AssignShift(opr),
            SemBinaryOpr::Comparison(opr) => HirBinaryOpr::Comparison(opr),
            SemBinaryOpr::ShortCircuitLogic(opr) => HirBinaryOpr::ShortCircuitLogic(opr),

            // For the variants without a direct match, you will need to decide the appropriate action.
            SemBinaryOpr::ScopeResolution => {
                panic!("ScopeResolution is not supported in HirBinaryOpr")
            }
            SemBinaryOpr::CurryType => {
                panic!("CurryType is not supported in HirBinaryOpr")
            }
            // Ins and In do not have an equivalent in HirBinaryOpr
            // In a real scenario, consider proper error handling instead of panicking.
            SemBinaryOpr::Ins => {
                panic!("Ins operator is not expected in HirBinaryOpr")
            }
            SemBinaryOpr::In => {
                panic!("In operator is not expected in HirBinaryOpr")
            }
            SemBinaryOpr::As => {
                panic!("As is not considered as an operator in HirBinaryOpr")
            }
            SemBinaryOpr::Of => {
                panic!("Of is not considered as an operator in HirBinaryOpr")
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
