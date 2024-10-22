use crate::precedence::{LnPrecedence, LnPrecedenceRange};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LnBinaryOpr {
    Add,
    Eq,
}

impl LnBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            LnBinaryOpr::Add => "+",
            LnBinaryOpr::Eq => "=",
        }
    }

    pub fn left_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::NoLess(LnPrecedence::AddSub),
            LnBinaryOpr::Eq => LnPrecedenceRange::NoLess(LnPrecedence::EqNe),
        }
    }

    pub fn right_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::Greater(LnPrecedence::AddSub),
            LnBinaryOpr::Eq => LnPrecedenceRange::Greater(LnPrecedence::EqNe),
        }
    }

    pub fn outer_precedence(self) -> LnPrecedence {
        match self {
            LnBinaryOpr::Add => LnPrecedence::AddSub,
            LnBinaryOpr::Eq => LnPrecedence::EqNe,
        }
    }
}
