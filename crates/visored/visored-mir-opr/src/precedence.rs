#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VdMirPrecedence(u64);

impl VdMirPrecedence {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Debug for VdMirPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MIN => write!(f, "VdPrecedence::MIN"),
            Self::SEMICOLON => write!(f, "VdPrecedence::SEMICOLON"),
            Self::COMMA => write!(f, "VdPrecedence::COMMA"),
            Self::RELATION => write!(f, "VdPrecedence::EQ"),
            Self::ADD_SUB => write!(f, "VdPrecedence::ADD"),
            Self::MUL_DIV => write!(f, "VdPrecedence::MUL"),
            Self::SPACE => write!(f, "VdPrecedence::SPACE"),
            Self::ATOM => write!(f, "VdPrecedence::ATOM"),
            Self::MAX => write!(f, "VdPrecedence::MAX"),
            _ => write!(f, "VdPrecedence({})", self.raw()),
        }
    }
}

impl std::fmt::Display for VdMirPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MIN => write!(f, "VdPrecedence::MIN"),
            Self::SEMICOLON => write!(f, "VdPrecedence::SEMICOLON"),
            Self::COMMA => write!(f, "VdPrecedence::COMMA"),
            Self::RELATION => write!(f, "VdPrecedence::EQ"),
            Self::ADD_SUB => write!(f, "VdPrecedence::ADD"),
            Self::MUL_DIV => write!(f, "VdPrecedence::MUL"),
            Self::SPACE => write!(f, "VdPrecedence::SPACE"),
            Self::ATOM => write!(f, "VdPrecedence::ATOM"),
            Self::MAX => write!(f, "VdPrecedence::MAX"),
            _ => write!(f, "VdPrecedence({})", self.raw()),
        }
    }
}

impl VdMirPrecedence {
    pub const MIN: Self = VdMirPrecedence(0);
    pub const INCOMPLTE_DELIMITED: Self = VdMirPrecedence(10);
    pub const RELATION: Self = VdMirPrecedence(500);
    pub const SEMICOLON: Self = VdMirPrecedence(1000);
    pub const COMMA: Self = VdMirPrecedence(5000);
    pub const ADD_SUB: Self = VdMirPrecedence(20000);
    pub const SIGN: Self = VdMirPrecedence(25000);
    /// `reduce` is from `map reduce`, examples are like sum/integral/prod
    pub const REDUCE_PREFIX: Self = VdMirPrecedence(25000);
    pub const DIFFERENTIAL: Self = VdMirPrecedence(25000);
    pub const MUL_DIV: Self = VdMirPrecedence(30000);
    pub const SPACE: Self = VdMirPrecedence(100000);
    pub const ATOM: Self = VdMirPrecedence(u64::MAX - 1);
    pub const MAX: Self = VdMirPrecedence(u64::MAX);
}

#[test]
fn vd_precedence_works() {
    // (a; b
    assert!(VdMirPrecedence::INCOMPLTE_DELIMITED < VdMirPrecedence::SEMICOLON);
    // a, b < 1
    assert!(VdMirPrecedence::RELATION < VdMirPrecedence::COMMA);
    // \sum a+ b
    assert!(VdMirPrecedence::ADD_SUB < VdMirPrecedence::REDUCE_PREFIX);
    // \sum a b
    assert!(VdMirPrecedence::REDUCE_PREFIX < VdMirPrecedence::MUL_DIV);
    // a+a\times b
    assert!(VdMirPrecedence::ADD_SUB < VdMirPrecedence::MUL_DIV);
    // a\times b c
    assert!(VdMirPrecedence::SIGN < VdMirPrecedence::SPACE);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdMirPrecedenceRange {
    Any,
    Greater(VdMirPrecedence),
    NoLess(VdMirPrecedence),
}

/// # constants
impl VdMirPrecedenceRange {
    pub const ANY: Self = VdMirPrecedenceRange::Any;
    pub const RIGHT_DELIMITER_LEFT: Self =
        VdMirPrecedenceRange::Greater(VdMirPrecedence::INCOMPLTE_DELIMITED);
    pub const SPACE_LEFT: Self = VdMirPrecedenceRange::NoLess(VdMirPrecedence::SPACE);
    pub const ADD_SUB_LEFT: Self = VdMirPrecedenceRange::NoLess(VdMirPrecedence::ADD_SUB);
    pub const ADD_SUB_RIGHT: Self = VdMirPrecedenceRange::Greater(VdMirPrecedence::ADD_SUB);
    pub const MUL_DIV_LEFT: Self = VdMirPrecedenceRange::NoLess(VdMirPrecedence::MUL_DIV);
    pub const COMPARISON_LEFT: Self = VdMirPrecedenceRange::NoLess(VdMirPrecedence::RELATION);
    pub const ATOM: Self = VdMirPrecedenceRange::NoLess(VdMirPrecedence::ATOM);
}

/// # methods
impl VdMirPrecedenceRange {
    pub fn contains(self, precedence: VdMirPrecedence) -> bool {
        match self {
            VdMirPrecedenceRange::Any => true,
            VdMirPrecedenceRange::Greater(p) => precedence > p,
            VdMirPrecedenceRange::NoLess(p) => precedence >= p,
        }
    }
}

#[test]
fn vd_precedence_range_works() {
    assert!(VdMirPrecedenceRange::SPACE_LEFT.contains(VdMirPrecedence::SPACE));
    assert!(!VdMirPrecedenceRange::SPACE_LEFT.contains(VdMirPrecedence::ADD_SUB));
}
