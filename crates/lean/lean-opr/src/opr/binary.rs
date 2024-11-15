use crate::precedence::{LnPrecedence, LnPrecedenceRange};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LnBinaryOpr {
    Add,
    Mul,
    Eq,
    Le,
    Ge,
}

impl LnBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            LnBinaryOpr::Add => "+",
            LnBinaryOpr::Mul => "*",
            LnBinaryOpr::Eq => "=",
            LnBinaryOpr::Le => "≤",
            LnBinaryOpr::Ge => "≥",
        }
    }

    pub fn left_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::NoLess(LnPrecedence::AddSub),
            LnBinaryOpr::Mul => LnPrecedenceRange::NoLess(LnPrecedence::MulDiv),
            LnBinaryOpr::Eq => LnPrecedenceRange::NoLess(LnPrecedence::EqNe),
            LnBinaryOpr::Le => LnPrecedenceRange::NoLess(LnPrecedence::EqNe),
            LnBinaryOpr::Ge => LnPrecedenceRange::NoLess(LnPrecedence::EqNe),
        }
    }

    pub fn right_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::Greater(LnPrecedence::AddSub),
            LnBinaryOpr::Mul => LnPrecedenceRange::Greater(LnPrecedence::MulDiv),
            LnBinaryOpr::Eq => LnPrecedenceRange::Greater(LnPrecedence::EqNe),
            LnBinaryOpr::Le => LnPrecedenceRange::Greater(LnPrecedence::EqNe),
            LnBinaryOpr::Ge => LnPrecedenceRange::Greater(LnPrecedence::EqNe),
        }
    }

    pub fn outer_precedence(self) -> LnPrecedence {
        match self {
            LnBinaryOpr::Add => LnPrecedence::AddSub,
            LnBinaryOpr::Mul => LnPrecedence::MulDiv,
            LnBinaryOpr::Eq => LnPrecedence::EqNe,
            LnBinaryOpr::Le => LnPrecedence::EqNe,
            LnBinaryOpr::Ge => LnPrecedence::EqNe,
        }
    }
}

impl std::fmt::Display for LnBinaryOpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_str())
    }
}
