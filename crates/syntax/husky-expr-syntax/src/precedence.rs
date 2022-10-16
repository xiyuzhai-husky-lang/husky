use crate::*;
use husky_opn_syntax::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum Precedence {
    ScopeResolution = 19,
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
    use husky_check_utils::should;

    should!(Precedence::Power < Precedence::Prefix);
}

impl From<BinaryOpr> for Precedence {
    fn from(binary: BinaryOpr) -> Self {
        match binary {
            BinaryOpr::Pure(pure_binary) => match pure_binary {
                PureBinaryOpr::Eq | PureBinaryOpr::Neq => Precedence::Equal,
                PureBinaryOpr::And => Precedence::And,
                PureBinaryOpr::BitAnd => Precedence::BitAnd,
                PureBinaryOpr::Or => Precedence::Or,
                PureBinaryOpr::BitOr => Precedence::BitOr,
                PureBinaryOpr::BitXor => Precedence::BitXor,
                PureBinaryOpr::Mul | PureBinaryOpr::Div | PureBinaryOpr::RemEuclid => {
                    Precedence::Multiplicative
                }
                PureBinaryOpr::Add | PureBinaryOpr::Sub => Precedence::Additive,
                PureBinaryOpr::Shl | PureBinaryOpr::Shr => Precedence::Shift,
                PureBinaryOpr::Leq
                | PureBinaryOpr::Less
                | PureBinaryOpr::Geq
                | PureBinaryOpr::Greater => Precedence::Compare,
                PureBinaryOpr::Power => Precedence::Power,
            },
            BinaryOpr::Assign(_) => Precedence::None,
            BinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            BinaryOpr::Curry => todo!(),
        }
    }
}
