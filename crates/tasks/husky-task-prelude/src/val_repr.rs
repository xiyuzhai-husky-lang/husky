use shifted_unsigned_int::ShiftedU32;
use smallvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValReprInterface(ShiftedU32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValDomainReprInterface {
    /// everything
    Omni,
    /// those where the val repr of type bool is defined and equals true
    ConditionSatisfied(ValReprInterface),
    /// those where the val repr of type bool is defined and equals false
    ConditionNotSatisfied(ValReprInterface),
    /// those where the val repr of type ControlFlow<(), _> is defined and equals Continue(())
    StmtNotReturned(ValReprInterface),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValArgumentReprInterface {
    Ordinary(ValReprInterface),
    Keyed(Option<ValReprInterface>),
    Variadic(SmallVec<[ValReprInterface; 4]>),
    Branch {
        condition: Option<ValReprInterface>,
        stmts: SmallVec<[ValReprInterface; 4]>,
    },
}
