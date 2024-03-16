use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryComparisonOpr {
    Eq,
    Neq,
    Geq,
    Greater,
    Leq,
    Less,
}

impl BinaryComparisonOpr {
    pub fn husky_code(self) -> &'static str {
        match self {
            BinaryComparisonOpr::Eq => "==",
            BinaryComparisonOpr::Neq => "!=",
            BinaryComparisonOpr::Greater => ">",
            BinaryComparisonOpr::Geq => ">=",
            BinaryComparisonOpr::Less => "<",
            BinaryComparisonOpr::Leq => "<=",
        }
    }
    pub fn spaced_husky_code(self) -> &'static str {
        match self {
            BinaryComparisonOpr::Greater => " > ",
            BinaryComparisonOpr::Geq => " >= ",
            BinaryComparisonOpr::Less => " < ",
            BinaryComparisonOpr::Leq => " <= ",
            BinaryComparisonOpr::Eq => " == ",
            BinaryComparisonOpr::Neq => " != ",
        }
    }
}

impl HasPrecedence for BinaryComparisonOpr {
    #[inline(always)]
    fn precedence(self) -> Precedence {
        match self {
            BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => Precedence::EqComparison,
            BinaryComparisonOpr::Leq
            | BinaryComparisonOpr::Less
            | BinaryComparisonOpr::Geq
            | BinaryComparisonOpr::Greater => Precedence::OrdComparison,
        }
    }
}
