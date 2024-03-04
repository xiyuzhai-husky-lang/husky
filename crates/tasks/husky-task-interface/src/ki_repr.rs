use shifted_unsigned_int::ShiftedU32;
use smallvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KiReprInterface(ShiftedU32);

impl KiReprInterface {
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
    ConditionSatisfied(KiReprInterface),
    /// those where the val repr of type bool is defined and equals false
    ConditionNotSatisfied(KiReprInterface),
    /// those where the val repr of type ControlFlow<(), _> is defined and equals Continue(())
    StmtNotReturned(KiReprInterface),
    ExprNotReturned(KiReprInterface),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KiRuntimeConstantInterface(ShiftedU32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KiArgumentReprInterface {
    Simple(KiReprInterface),
    Keyed(Option<KiReprInterface>),
    Variadic(SmallVec<[KiReprInterface; 4]>),
    Branch {
        condition: Option<KiReprInterface>,
        stmts: SmallVec<[KiReprInterface; 4]>,
    },
    RuntimeConstants(SmallVec<[KiRuntimeConstantInterface; 4]>),
}
