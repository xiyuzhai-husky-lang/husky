use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdSetPath {
    Prelude(VdPreludeSetPath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdPreludeSetPath {
    NaturalNumber,
    RationalNumber,
    Integer,
    RealNumber,
    ComplexNumber,
}

impl VdSetPath {
    pub const NATURAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::NaturalNumber);
    pub const RATIONAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::RationalNumber);
    pub const INTEGER: Self = Self::Prelude(VdPreludeSetPath::Integer);
    pub const REAL_NUMBER: Self = Self::Prelude(VdPreludeSetPath::RealNumber);
    pub const COMPLEX_NUMBER: Self = Self::Prelude(VdPreludeSetPath::ComplexNumber);
}

impl salsa::DisplayWithDb for VdSetPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        match self {
            VdSetPath::Prelude(path) => path.display_fmt_with_db(f, db),
        }
    }
}

impl salsa::DisplayWithDb for VdPreludeSetPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        match self {
            VdPreludeSetPath::NaturalNumber => write!(f, "ℕ"),
            VdPreludeSetPath::RationalNumber => write!(f, "ℚ"),
            VdPreludeSetPath::Integer => write!(f, "ℤ"),
            VdPreludeSetPath::RealNumber => write!(f, "ℝ"),
            VdPreludeSetPath::ComplexNumber => write!(f, "ℂ"),
        }
    }
}
