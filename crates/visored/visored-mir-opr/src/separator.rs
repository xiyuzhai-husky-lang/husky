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
}

impl VdMirBaseSeparator {
    pub const RING_ADD: Self = VdMirBaseSeparator::CommRingAdd;
    pub const RING_MUL: Self = VdMirBaseSeparator::CommRingMul;
    pub const EQ: Self = VdMirBaseSeparator::Eq;
    pub const NE: Self = VdMirBaseSeparator::Ne;
    pub const LT: Self = VdMirBaseSeparator::Lt;
    pub const GT: Self = VdMirBaseSeparator::Gt;
    pub const LE: Self = VdMirBaseSeparator::Le;
    pub const GE: Self = VdMirBaseSeparator::Ge;
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
