mod comparison;
mod logic;
mod pure_closed;

pub use comparison::*;
pub use logic::*;
pub use pure_closed::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryPunctuation {
    PureClosed(BinaryPureClosedPunctuation),
    Comparison(BinaryComparisonPunctuation),
    ShortcuitLogic(BinaryShortcuitLogicPunctuation),
    Assign(Option<BinaryPureClosedPunctuation>),
    ScopeResolution, // ::
    Curry,           // ->
    As,              // as
}

impl BinaryPunctuation {
    pub fn code(self) -> &'static str {
        match self {
            BinaryPunctuation::PureClosed(pure_opr) => pure_opr.husky_code(),
            BinaryPunctuation::Assign(None) => "=",
            BinaryPunctuation::Assign(Some(pure_opr)) => match pure_opr {
                BinaryPureClosedPunctuation::Add => "+=",
                BinaryPureClosedPunctuation::BitAnd => "&=",
                BinaryPureClosedPunctuation::BitOr => "|=",
                BinaryPureClosedPunctuation::BitXor => "^=",
                BinaryPureClosedPunctuation::Div => "/=",
                BinaryPureClosedPunctuation::Mul => "*=",
                BinaryPureClosedPunctuation::RemEuclid => "%=",
                BinaryPureClosedPunctuation::Power => "**=",
                BinaryPureClosedPunctuation::Shl => "<<=",
                BinaryPureClosedPunctuation::Shr => ">>=",
                BinaryPureClosedPunctuation::Sub => "-=",
            },
            BinaryPunctuation::Comparison(cmp_opr) => cmp_opr.husky_code(),
            BinaryPunctuation::ShortcuitLogic(logic_opr) => logic_opr.husky_code(),
            BinaryPunctuation::ScopeResolution => "::",
            BinaryPunctuation::Curry => "->",
            BinaryPunctuation::As => todo!(),
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryPunctuation::PureClosed(pure_binary_opr) => pure_binary_opr.spaced_husky_code(),
            BinaryPunctuation::Comparison(cmp_opr) => cmp_opr.spaced_husky_code(),
            BinaryPunctuation::ShortcuitLogic(logic_opr) => logic_opr.spaced_husky_code(),
            BinaryPunctuation::Assign(opt_binary_opr) => {
                if let Some(binary_opr) = opt_binary_opr {
                    match binary_opr {
                        BinaryPureClosedPunctuation::Add => " += ",
                        BinaryPureClosedPunctuation::BitAnd => " &= ",
                        BinaryPureClosedPunctuation::BitOr => " |= ",
                        BinaryPureClosedPunctuation::BitXor => " ^= ",
                        BinaryPureClosedPunctuation::Div => " /= ",
                        BinaryPureClosedPunctuation::Mul => " *= ",
                        BinaryPureClosedPunctuation::RemEuclid => " %= ",
                        BinaryPureClosedPunctuation::Power => " **= ",
                        BinaryPureClosedPunctuation::Shl => " <<= ",
                        BinaryPureClosedPunctuation::Shr => " >>= ",
                        BinaryPureClosedPunctuation::Sub => " -= ",
                    }
                } else {
                    " = "
                }
            }
            BinaryPunctuation::ScopeResolution => todo!(),
            BinaryPunctuation::Curry => " -> ",
            BinaryPunctuation::As => todo!(),
        }
    }
}
