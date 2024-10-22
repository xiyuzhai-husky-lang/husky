use crate::precedence::{LnPrecedence, LnPrecedenceRange};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LnBinaryOpr {
    Add,
}

impl LnBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            LnBinaryOpr::Add => "+",
        }
    }

    pub fn left_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::Greater(LnPrecedence::AddSub),
        }
    }

    pub fn right_precedence_range(self) -> LnPrecedenceRange {
        match self {
            LnBinaryOpr::Add => LnPrecedenceRange::NoLess(LnPrecedence::AddSub),
        }
    }

    pub fn outer_precedence(self) -> LnPrecedence {
        match self {
            LnBinaryOpr::Add => LnPrecedence::AddSub,
        }
    }
}
