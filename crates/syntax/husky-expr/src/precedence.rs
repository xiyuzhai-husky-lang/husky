use husky_opn_syntax::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum Precedence {
    ScopeResolution = 19,
    Prefix = 18,
    Power = 17,
    Multiplicative = 16,
    Additive = 15,
    Shift = 14,
    OrdComparison = 13,
    EqComparison = 12,
    BitAnd = 11,
    BitXor = 10,
    BitOr = 9,
    And = 8,
    Or = 7,
    Dot = 3,
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
            BinaryOpr::PureClosed(pure_binary) => match pure_binary {
                BinaryPureClosedOpr::BitAnd => Precedence::BitAnd,
                BinaryPureClosedOpr::BitOr => Precedence::BitOr,
                BinaryPureClosedOpr::BitXor => Precedence::BitXor,
                BinaryPureClosedOpr::Mul
                | BinaryPureClosedOpr::Div
                | BinaryPureClosedOpr::RemEuclid => Precedence::Multiplicative,
                BinaryPureClosedOpr::Add | BinaryPureClosedOpr::Sub => Precedence::Additive,
                BinaryPureClosedOpr::Shl | BinaryPureClosedOpr::Shr => Precedence::Shift,
                BinaryPureClosedOpr::Power => Precedence::Power,
            },
            BinaryOpr::Comparison(cmp_opr) => match cmp_opr {
                BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => Precedence::EqComparison,
                BinaryComparisonOpr::Leq
                | BinaryComparisonOpr::Less
                | BinaryComparisonOpr::Geq
                | BinaryComparisonOpr::Greater => Precedence::OrdComparison,
            },
            BinaryOpr::ShortcuitLogic(logic_opr) => match logic_opr {
                BinaryShortcuitLogicOpr::And => Precedence::And,
                BinaryShortcuitLogicOpr::Or => Precedence::Or,
            },
            BinaryOpr::Assign(_) => Precedence::None,
            BinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
        }
    }
}
