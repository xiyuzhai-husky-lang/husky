#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
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
    Application = 540,
    Be = 230,
    /// means `->`
    Curry = 170,
    KeyedArgument = 140,
    ListItem = 110,
    LambdaHead = 30,
    Method = 20,
    Assign = 10,
    List = 1,
    None = 0,
}

#[cfg(test)]
#[test]
fn test_precedence_order() {
    use husky_check_utils::should;

    should!(Precedence::Power < Precedence::Prefix);
}

pub trait HasPrecedence: Copy {
    fn precedence(self) -> Precedence;
}
