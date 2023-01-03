use husky_opn_syntax::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum Precedence {
    ScopeResolution = 1019,
    Prefix = 1018,
    Power = 1017,
    Multiplicative = 1016,
    Additive = 1015,
    Shift = 1014,
    OrdComparison = 1013,
    EqComparison = 1012,
    BitAnd = 1011,
    BitXor = 1010,
    BitOr = 1009,
    And = 1008,
    Or = 1007,
    Curry = 105,
    Application = 54,
    ListItem = 11,
    LambdaHead = 3,
    Method = 2,
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
                BinaryComparisonPunctuation::Eq | BinaryComparisonPunctuation::Neq => {
                    Precedence::EqComparison
                }
                BinaryComparisonPunctuation::Leq
                | BinaryComparisonPunctuation::Less
                | BinaryComparisonPunctuation::Geq
                | BinaryComparisonPunctuation::Greater => Precedence::OrdComparison,
            },
            BinaryOpr::ShortcuitLogic(logic_opr) => match logic_opr {
                BinaryShortcuitLogicPunctuation::And => Precedence::And,
                BinaryShortcuitLogicPunctuation::Or => Precedence::Or,
            },
            BinaryOpr::Assign(_) => Precedence::None,
            BinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            BinaryOpr::Curry => Precedence::Curry,
            BinaryOpr::As => todo!(),
        }
    }
}
