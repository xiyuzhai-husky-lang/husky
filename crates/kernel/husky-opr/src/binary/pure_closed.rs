use crate::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryClosedOpr {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Mul,
    RemEuclid,
    Power,
    Sub,
}

impl BinaryClosedOpr {
    pub fn husky_code(&self) -> &'static str {
        match self {
            BinaryClosedOpr::Add => "+",
            BinaryClosedOpr::BitAnd => "&",
            BinaryClosedOpr::BitOr => "|",
            BinaryClosedOpr::BitXor => "^",
            BinaryClosedOpr::Div => "/",
            BinaryClosedOpr::Mul => "*",
            BinaryClosedOpr::Power => "**",
            BinaryClosedOpr::RemEuclid => "%",
            BinaryClosedOpr::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
        match self {
            BinaryClosedOpr::Add => " + ",
            BinaryClosedOpr::Sub => " - ",
            BinaryClosedOpr::Mul => " * ",
            BinaryClosedOpr::Div => " / ",
            BinaryClosedOpr::BitAnd => " & ",
            BinaryClosedOpr::Power => " ** ",
            BinaryClosedOpr::BitXor => " ^ ",
            BinaryClosedOpr::BitOr => " | ",
            BinaryClosedOpr::RemEuclid => " % ",
        }
    }
}

impl HasPrecedence for BinaryClosedOpr {
    #[inline(always)]
    fn precedence(self) -> Precedence {
        match self {
            BinaryClosedOpr::BitAnd => Precedence::BitAnd,
            BinaryClosedOpr::BitOr => Precedence::BitOr,
            BinaryClosedOpr::BitXor => Precedence::BitXor,
            BinaryClosedOpr::Mul | BinaryClosedOpr::Div | BinaryClosedOpr::RemEuclid => {
                Precedence::Multiplicative
            }
            BinaryClosedOpr::Add | BinaryClosedOpr::Sub => Precedence::Additive,
            BinaryClosedOpr::Power => Precedence::Power,
        }
    }
}
