use crate::precedence::VdMirPrecedence;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum VdMirBasePrefixOpr {
    RingPos,
    RingNeg,
}

impl VdMirBasePrefixOpr {
    pub const RING_POS: Self = VdMirBasePrefixOpr::RingPos;
    pub const RING_NEG: Self = VdMirBasePrefixOpr::RingNeg;
}

impl VdMirBasePrefixOpr {
    pub fn precedence(self) -> VdMirPrecedence {
        match self {
            VdMirBasePrefixOpr::RING_POS | VdMirBasePrefixOpr::RING_NEG => VdMirPrecedence::SIGN,
            _ => todo!(),
        }
    }
}
