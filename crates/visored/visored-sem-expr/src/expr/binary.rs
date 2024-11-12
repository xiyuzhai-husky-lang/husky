use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemBinaryOpr {
    Base(LxTokenIdxRange, VdBaseBinaryOpr),
    Composite(VdSemExprIdx, VdCompositeBinaryOpr),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemBinaryDispatch {
    IntAdd,
    TrivialEq,
}
