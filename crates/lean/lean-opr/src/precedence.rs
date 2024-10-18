#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LeanPrecedence {
    Application,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LeanPrecedenceRange {
    Any,
    Greater(LeanPrecedence),
    GreaterThan(LeanPrecedence),
}

/// # constants
impl LeanPrecedenceRange {
    pub const APPLICATION_SUBEXPR: Self = LeanPrecedenceRange::Greater(LeanPrecedence::Application);
}

/// # methods
impl LeanPrecedenceRange {
    pub fn include(self, precedence: LeanPrecedence) -> bool {
        match self {
            LeanPrecedenceRange::Any => true,
            LeanPrecedenceRange::Greater(p) => precedence > p,
            LeanPrecedenceRange::GreaterThan(p) => precedence >= p,
        }
    }
}
