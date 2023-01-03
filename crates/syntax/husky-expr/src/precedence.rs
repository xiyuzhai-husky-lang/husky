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
            BinaryPunctuation::Curry => Precedence::Curry,
            BinaryPunctuation::As => todo!(),
        }
    }
}
