use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemLeftDelimiter {
    Base(LxTokenIdxRange, VdBaseLeftDelimiter),
    Composite(VdSemExprIdx, VdCompositeLeftDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemRightDelimiter {
    Base(LxTokenIdxRange, VdBaseRightDelimiter),
    Composite(VdSemExprIdx, VdCompositeRightDelimiter),
}
