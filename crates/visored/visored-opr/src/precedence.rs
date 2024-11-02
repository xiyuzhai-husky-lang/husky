#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VdPrecedence(u64);

impl VdPrecedence {
    pub const EQ: Self = VdPrecedence(100);
    pub const ADD: Self = VdPrecedence(200);
    pub const MUL: Self = VdPrecedence(300);
    pub const SPACE: Self = VdPrecedence(1000);
}

#[test]
fn vd_precedence_works() {
    // a=a+b
    assert!(VdPrecedence::EQ < VdPrecedence::ADD);
    // a+a\times b
    assert!(VdPrecedence::ADD < VdPrecedence::MUL);
    // a\times b c
    assert!(VdPrecedence::MUL < VdPrecedence::SPACE);
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
    pub const ADD_LEFT: Self = VdPrecedenceRange::NoLess(VdPrecedence::ADD);
    pub const MUL_LEFT: Self = VdPrecedenceRange::NoLess(VdPrecedence::MUL);
    pub const EQ_LEFT: Self = VdPrecedenceRange::NoLess(VdPrecedence::EQ);
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
