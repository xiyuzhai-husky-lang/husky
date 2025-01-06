use crate::precedence::{VdMirPrecedence, VdMirPrecedenceRange};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum VdMirBaseSeparator {
    CommRingAdd,
    CommRingMul,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Subset,
    Supset,
    Subseteq,
    Supseteq,
    Subseteqq,
    Supseteqq,
    Subsetneq,
    Supsetneq,
    In,
    Notin,
}

impl VdMirBaseSeparator {
    pub const COMM_RING_ADD: Self = VdMirBaseSeparator::CommRingAdd;
    pub const COMM_RING_MUL: Self = VdMirBaseSeparator::CommRingMul;
    pub const EQ: Self = VdMirBaseSeparator::Eq;
    pub const NE: Self = VdMirBaseSeparator::Ne;
    pub const LT: Self = VdMirBaseSeparator::Lt;
    pub const GT: Self = VdMirBaseSeparator::Gt;
    pub const LE: Self = VdMirBaseSeparator::Le;
    pub const GE: Self = VdMirBaseSeparator::Ge;
    pub const SUBSET: Self = VdMirBaseSeparator::Subset;
    pub const SUPSET: Self = VdMirBaseSeparator::Supset;
    pub const SUBSETEQ: Self = VdMirBaseSeparator::Subseteq;
    pub const SUPSETEQ: Self = VdMirBaseSeparator::Supseteq;
    pub const SUBSETEQQ: Self = VdMirBaseSeparator::Subseteqq;
    pub const SUPSETEQQ: Self = VdMirBaseSeparator::Supseteqq;
    pub const SUBSETNEQ: Self = VdMirBaseSeparator::Subsetneq;
    pub const SUPSETNEQ: Self = VdMirBaseSeparator::Supsetneq;
}

impl VdMirBaseSeparator {
    pub fn precedence(self) -> VdMirPrecedence {
        todo!()
    }

    pub fn left_precedence_range(self) -> VdMirPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> VdMirPrecedenceRange {
        todo!()
    }
}

impl VdMirBaseSeparator {
    pub fn unicode(self) -> &'static str {
        todo!()
    }

    pub fn show_fmt(self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        todo!()
    }
}
