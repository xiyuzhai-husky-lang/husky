use common::*;

use crate::*;

use atom::BinaryOpr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    Prefix = 18,
    Power = 17,
    Multiplicative = 16,
    Additive = 15,
    Shift = 14,
    Compare = 13,
    Equal = 12,
    BitAnd = 11,
    BitXor = 10,
    BitOr = 9,
    And = 8,
    Or = 7,
    LambdaHead = 2,
    ListItem = 1,
    None = 0,
}

#[cfg(test)]
#[test]
fn test_precedence_order() {
    should!(Precedence::Power < Precedence::Prefix);
}

impl From<BinaryOpr> for Precedence {
    fn from(binary: BinaryOpr) -> Self {
        match binary {
            BinaryOpr::Eq | BinaryOpr::Neq => Precedence::Equal,
            BinaryOpr::And => Precedence::And,
            BinaryOpr::BitAnd => Precedence::BitAnd,
            BinaryOpr::Or => Precedence::Or,
            BinaryOpr::BitOr => Precedence::BitOr,
            BinaryOpr::BitXor => Precedence::BitXor,
            BinaryOpr::Mult | BinaryOpr::Div | BinaryOpr::Modulo => Precedence::Multiplicative,
            BinaryOpr::Add | BinaryOpr::Sub => Precedence::Additive,
            BinaryOpr::LShift | BinaryOpr::RShift => Precedence::Shift,
            BinaryOpr::Leq | BinaryOpr::Less | BinaryOpr::Geq | BinaryOpr::Greater => {
                Precedence::Compare
            }
            BinaryOpr::Power => Precedence::Power,
        }
    }
}
