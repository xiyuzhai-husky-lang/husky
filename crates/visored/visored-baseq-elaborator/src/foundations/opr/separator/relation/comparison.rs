use visored_mir_opr::separator::VdMirBaseSeparator;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqComparisonOpr {
    Bound(VdBsqBoundOpr),
    Eq,
    Ne,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqBoundOpr {
    Lt,
    Gt,
    Le,
    Ge,
}

impl VdBsqComparisonOpr {
    pub const EQ: Self = VdBsqComparisonOpr::Eq;
    pub const NE: Self = VdBsqComparisonOpr::Ne;
    pub const LT: Self = VdBsqComparisonOpr::Bound(VdBsqBoundOpr::Lt);
    pub const GT: Self = VdBsqComparisonOpr::Bound(VdBsqBoundOpr::Gt);
    pub const LE: Self = VdBsqComparisonOpr::Bound(VdBsqBoundOpr::Le);
    pub const GE: Self = VdBsqComparisonOpr::Bound(VdBsqBoundOpr::Ge);
}

impl VdBsqBoundOpr {
    pub fn from_mir_base_separator(separator: VdMirBaseSeparator) -> Option<Self> {
        match separator {
            VdMirBaseSeparator::Lt => Some(Self::Lt),
            VdMirBaseSeparator::Gt => Some(Self::Gt),
            VdMirBaseSeparator::Le => Some(Self::Le),
            VdMirBaseSeparator::Ge => Some(Self::Ge),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VdBsqBoundBoundaryKind {
    Closed,
    Open,
}
