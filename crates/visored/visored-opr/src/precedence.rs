#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VdPrecedence(u64);

impl VdPrecedence {
    pub const APPLICATION: Self = VdPrecedence(0);
    pub const SPACE: Self = VdPrecedence(0);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdPrecedenceRange {
    Any,
    Greater(VdPrecedence),
    GreaterThan(VdPrecedence),
}

/// # constants
impl VdPrecedenceRange {
    pub const APPLICATION_SUBEXPR: Self = VdPrecedenceRange::Greater(VdPrecedence::APPLICATION);
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
