use shifted_unsigned_int::ShiftedU32;
use smallvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValReprInterface(ShiftedU32);

impl ValReprInterface {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

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
    ExprNotReturned(ValReprInterface),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValRuntimeConstantInterface(ShiftedU32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValArgumentReprInterface {
    Simple(ValReprInterface),
    Keyed(Option<ValReprInterface>),
    Variadic(SmallVec<[ValReprInterface; 4]>),
    Branch {
        condition: Option<ValReprInterface>,
        stmts: SmallVec<[ValReprInterface; 4]>,
    },
    RuntimeConstants(SmallVec<[ValRuntimeConstantInterface; 4]>),
}
