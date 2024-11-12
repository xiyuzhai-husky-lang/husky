use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemPrefixOpr {
    Base(LxTokenIdxRange, VdBasePrefixOpr),
    Composite(VdSemExprIdx, VdCompositePrefixOpr),
}
