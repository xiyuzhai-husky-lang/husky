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
    As = 1003,
    Is = 1002,
    Application = 54,
    Be = 23,
    /// means `->`
    Curry = 17,
    KeyedArgument = 14,
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
            BinaryOpr::Closed(opr) => match opr {
                BinaryClosedOpr::BitAnd => Precedence::BitAnd,
                BinaryClosedOpr::BitOr => Precedence::BitOr,
                BinaryClosedOpr::BitXor => Precedence::BitXor,
                BinaryClosedOpr::Mul | BinaryClosedOpr::Div | BinaryClosedOpr::RemEuclid => {
                    Precedence::Multiplicative
                }
                BinaryClosedOpr::Add | BinaryClosedOpr::Sub => Precedence::Additive,
                BinaryClosedOpr::Power => Precedence::Power,
            },
            BinaryOpr::Shift(opr) => match opr {
                BinaryShiftOpr::Shl | BinaryShiftOpr::Shr => Precedence::Shift,
            },
            BinaryOpr::Comparison(cmp_opr) => match cmp_opr {
                BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => Precedence::EqComparison,
                BinaryComparisonOpr::Leq
                | BinaryComparisonOpr::Less
                | BinaryComparisonOpr::Geq
                | BinaryComparisonOpr::Greater => Precedence::OrdComparison,
            },
            BinaryOpr::ShortCircuitLogic(logic_opr) => match logic_opr {
                BinaryShortcuitLogicOpr::And => Precedence::And,
                BinaryShortcuitLogicOpr::Or => Precedence::Or,
            },
            BinaryOpr::Assign | BinaryOpr::AssignClosed(_) | BinaryOpr::AssignShift(_) => {
                Precedence::None
            }
            BinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            BinaryOpr::Curry => Precedence::Curry,
            BinaryOpr::As => Precedence::As,
            BinaryOpr::Ins => Precedence::Is,
            BinaryOpr::In => todo!(),
        }
    }
}
