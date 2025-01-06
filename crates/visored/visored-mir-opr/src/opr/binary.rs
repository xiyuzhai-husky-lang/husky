use crate::precedence::{VdMirPrecedence, VdMirPrecedenceRange};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum VdMirBaseBinaryOpr {
    CommRingSub,
    CommFieldDiv,
}

impl VdMirBaseBinaryOpr {
    pub const COMM_RING_SUB: Self = VdMirBaseBinaryOpr::CommRingSub;
    pub const COMM_FIELD_DIV: Self = VdMirBaseBinaryOpr::CommFieldDiv;
}

impl VdMirBaseBinaryOpr {
    pub fn precedence(self) -> VdMirPrecedence {
        todo!()
    }

    pub fn left_precedence_range(self) -> VdMirPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> VdMirPrecedenceRange {
        todo!()
    }

    pub fn unicode(self) -> &'static str {
        todo!()
    }
}
