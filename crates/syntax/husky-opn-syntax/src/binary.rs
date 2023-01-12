mod comparison;
mod logic;
mod pure_closed;

pub use comparison::*;
pub use logic::*;
pub use pure_closed::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOpr {
    PureClosed(PureClosedBinaryOpr),
    Comparison(BinaryComparisonOpr),
    ShortcuitLogic(BinaryShortcuitLogicOpr),
    Assign(Option<PureClosedBinaryOpr>),
    ScopeResolution,
    Curry, // ->
    As,    // as
    Is,    // :
}

impl From<Option<PureClosedBinaryOpr>> for BinaryOpr {
    fn from(v: Option<PureClosedBinaryOpr>) -> Self {
        Self::Assign(v)
    }
}

impl From<BinaryShortcuitLogicOpr> for BinaryOpr {
    fn from(v: BinaryShortcuitLogicOpr) -> Self {
        Self::ShortcuitLogic(v)
    }
}

impl From<PureClosedBinaryOpr> for BinaryOpr {
    fn from(v: PureClosedBinaryOpr) -> Self {
        Self::PureClosed(v)
    }
}

impl From<BinaryComparisonOpr> for BinaryOpr {
    fn from(v: BinaryComparisonOpr) -> Self {
        Self::Comparison(v)
    }
}

impl BinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_opr) => pure_opr.husky_code(),
            BinaryOpr::Assign(None) => "=",
            BinaryOpr::Assign(Some(pure_opr)) => match pure_opr {
                PureClosedBinaryOpr::Add => "+=",
                PureClosedBinaryOpr::BitAnd => "&=",
                PureClosedBinaryOpr::BitOr => "|=",
                PureClosedBinaryOpr::BitXor => "^=",
                PureClosedBinaryOpr::Div => "/=",
                PureClosedBinaryOpr::Mul => "*=",
                PureClosedBinaryOpr::RemEuclid => "%=",
                PureClosedBinaryOpr::Power => "**=",
                PureClosedBinaryOpr::Shl => "<<=",
                PureClosedBinaryOpr::Shr => ">>=",
                PureClosedBinaryOpr::Sub => "-=",
            },
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.husky_code(),
            BinaryOpr::ShortcuitLogic(logic_opr) => logic_opr.husky_code(),
            BinaryOpr::Curry => "->",
            BinaryOpr::As => todo!(),
            BinaryOpr::Is => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::PureClosed(pure_binary_opr) => pure_binary_opr.spaced_husky_code(),
            BinaryOpr::Comparison(cmp_opr) => cmp_opr.spaced_husky_code(),
            BinaryOpr::ShortcuitLogic(logic_opr) => logic_opr.spaced_husky_code(),
            BinaryOpr::Assign(opt_binary_opr) => {
                if let Some(binary_opr) = opt_binary_opr {
                    match binary_opr {
                        PureClosedBinaryOpr::Add => " += ",
                        PureClosedBinaryOpr::BitAnd => " &= ",
                        PureClosedBinaryOpr::BitOr => " |= ",
                        PureClosedBinaryOpr::BitXor => " ^= ",
                        PureClosedBinaryOpr::Div => " /= ",
                        PureClosedBinaryOpr::Mul => " *= ",
                        PureClosedBinaryOpr::RemEuclid => " %= ",
                        PureClosedBinaryOpr::Power => " **= ",
                        PureClosedBinaryOpr::Shl => " <<= ",
                        PureClosedBinaryOpr::Shr => " >>= ",
                        PureClosedBinaryOpr::Sub => " -= ",
                    }
                } else {
                    " = "
                }
            }
            BinaryOpr::Curry => " -> ",
            BinaryOpr::As => " as ",
            BinaryOpr::Is => " : ",
            BinaryOpr::ScopeResolution => todo!(),
        }
    }
}
