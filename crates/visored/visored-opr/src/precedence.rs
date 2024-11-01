#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VdPrecedence(u64);

impl VdPrecedence {
    pub const SPACE: Self = VdPrecedence(100);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdPrecedenceRange {
    Any,
    Greater(VdPrecedence),
    NoLess(VdPrecedence),
}

/// # constants
impl VdPrecedenceRange {
    pub const SPACE_LEFT: Self = VdPrecedenceRange::NoLess(VdPrecedence::SPACE);
}

/// # methods
impl VdPrecedenceRange {
    pub fn include(self, precedence: VdPrecedence) -> bool {
        match self {
            VdPrecedenceRange::Any => true,
            VdPrecedenceRange::Greater(p) => precedence > p,
            VdPrecedenceRange::NoLess(p) => precedence >= p,
        }
    }
}
