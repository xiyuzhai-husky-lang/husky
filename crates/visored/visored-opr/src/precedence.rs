#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VdPrecedence {
    Application,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdPrecedenceRange {
    Any,
    Greater(VdPrecedence),
    GreaterThan(VdPrecedence),
}

/// # constants
impl VdPrecedenceRange {
    pub const APPLICATION_SUBEXPR: Self = VdPrecedenceRange::Greater(VdPrecedence::Application);
}

/// # methods
impl VdPrecedenceRange {
    pub fn include(self, precedence: VdPrecedence) -> bool {
        match self {
            VdPrecedenceRange::Any => true,
            VdPrecedenceRange::Greater(p) => precedence > p,
            VdPrecedenceRange::GreaterThan(p) => precedence >= p,
        }
    }
}
