#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VdPrecedence(u64);

impl VdPrecedence {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Debug for VdPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MIN => write!(f, "VdPrecedence::MIN"),
            Self::SEMICOLON => write!(f, "VdPrecedence::SEMICOLON"),
            Self::COMMA => write!(f, "VdPrecedence::COMMA"),
            Self::EQ => write!(f, "VdPrecedence::EQ"),
            Self::ADD => write!(f, "VdPrecedence::ADD"),
            Self::MUL => write!(f, "VdPrecedence::MUL"),
            Self::SPACE => write!(f, "VdPrecedence::SPACE"),
            Self::ATOM => write!(f, "VdPrecedence::ATOM"),
            Self::MAX => write!(f, "VdPrecedence::MAX"),
            _ => write!(f, "VdPrecedence({})", self.raw()),
        }
    }
}

impl std::fmt::Display for VdPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MIN => write!(f, "VdPrecedence::MIN"),
            Self::SEMICOLON => write!(f, "VdPrecedence::SEMICOLON"),
            Self::COMMA => write!(f, "VdPrecedence::COMMA"),
            Self::EQ => write!(f, "VdPrecedence::EQ"),
            Self::ADD => write!(f, "VdPrecedence::ADD"),
            Self::MUL => write!(f, "VdPrecedence::MUL"),
            Self::SPACE => write!(f, "VdPrecedence::SPACE"),
            Self::ATOM => write!(f, "VdPrecedence::ATOM"),
            Self::MAX => write!(f, "VdPrecedence::MAX"),
            _ => write!(f, "VdPrecedence({})", self.raw()),
        }
    }
}

impl VdPrecedence {
    pub const MIN: Self = VdPrecedence(0);
    pub const SEMICOLON: Self = VdPrecedence(4000);
    pub const COMMA: Self = VdPrecedence(5000);
    pub const EQ: Self = VdPrecedence(10000);
    pub const ADD: Self = VdPrecedence(20000);
    pub const MUL: Self = VdPrecedence(30000);
    pub const SPACE: Self = VdPrecedence(100000);
    pub const ATOM: Self = VdPrecedence(u64::MAX - 1);
    pub const MAX: Self = VdPrecedence(u64::MAX);
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
    pub fn contains(self, precedence: VdPrecedence) -> bool {
        match self {
            VdPrecedenceRange::Any => true,
            VdPrecedenceRange::Greater(p) => precedence > p,
            VdPrecedenceRange::NoLess(p) => precedence >= p,
        }
    }
}

#[test]
fn vd_precedence_range_works() {
    assert!(VdPrecedenceRange::SPACE_LEFT.contains(VdPrecedence::SPACE));
    assert!(!VdPrecedenceRange::SPACE_LEFT.contains(VdPrecedence::ADD));
}
