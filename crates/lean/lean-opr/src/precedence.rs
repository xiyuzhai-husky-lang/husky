#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LnPrecedence {
    Application,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LnPrecedenceRange {
    Any,
    Greater(LnPrecedence),
    GreaterThan(LnPrecedence),
}

/// # constants
impl LnPrecedenceRange {
    pub const APPLICATION_SUBEXPR: Self = LnPrecedenceRange::Greater(LnPrecedence::Application);
}

/// # methods
impl LnPrecedenceRange {
    pub fn include(self, precedence: LnPrecedence) -> bool {
        match self {
            LnPrecedenceRange::Any => true,
            LnPrecedenceRange::Greater(p) => precedence > p,
            LnPrecedenceRange::GreaterThan(p) => precedence >= p,
        }
    }
}
