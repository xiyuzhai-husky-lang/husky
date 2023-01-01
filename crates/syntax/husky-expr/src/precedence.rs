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
    NextCurry = 5,
    PrevCurry = 4,
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

impl From<BinaryPunctuation> for Precedence {
    fn from(binary: BinaryPunctuation) -> Self {
        match binary {
            BinaryPunctuation::PureClosed(pure_binary) => match pure_binary {
                BinaryPureClosedPunctuation::BitAnd => Precedence::BitAnd,
                BinaryPureClosedPunctuation::BitOr => Precedence::BitOr,
                BinaryPureClosedPunctuation::BitXor => Precedence::BitXor,
                BinaryPureClosedPunctuation::Mul
                | BinaryPureClosedPunctuation::Div
                | BinaryPureClosedPunctuation::RemEuclid => Precedence::Multiplicative,
                BinaryPureClosedPunctuation::Add | BinaryPureClosedPunctuation::Sub => {
                    Precedence::Additive
                }
                BinaryPureClosedPunctuation::Shl | BinaryPureClosedPunctuation::Shr => {
                    Precedence::Shift
                }
                BinaryPureClosedPunctuation::Power => Precedence::Power,
            },
            BinaryPunctuation::Comparison(cmp_opr) => match cmp_opr {
                BinaryComparisonPunctuation::Eq | BinaryComparisonPunctuation::Neq => {
                    Precedence::EqComparison
                }
                BinaryComparisonPunctuation::Leq
                | BinaryComparisonPunctuation::Less
                | BinaryComparisonPunctuation::Geq
                | BinaryComparisonPunctuation::Greater => Precedence::OrdComparison,
            },
            BinaryPunctuation::ShortcuitLogic(logic_opr) => match logic_opr {
                BinaryShortcuitLogicPunctuation::And => Precedence::And,
                BinaryShortcuitLogicPunctuation::Or => Precedence::Or,
            },
            BinaryPunctuation::Assign(_) => Precedence::None,
            BinaryPunctuation::ScopeResolution => Precedence::ScopeResolution,
            BinaryPunctuation::Curry => todo!(),
            BinaryPunctuation::As => todo!(),
        }
    }
}
