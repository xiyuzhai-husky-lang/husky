use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Clone)]
pub enum InstructionKind {
    PushVarInput(InputContract, u16),
    PushPrimitive(PrimitiveValue),
    Call(fn(&mut VirtualStack) -> VirtualStackResult<()>, u16),
    CallInterpret(Arc<Vec<Instruction>>),
    PrimitiveOpn(PrimitiveOpn),
}

impl std::fmt::Debug for InstructionKind {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PushVarInput(arg0, arg1) => f
                .debug_tuple("PushVarInput")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::PushPrimitive(arg0) => f.debug_tuple("PushPrimitive").field(arg0).finish(),
            Self::Call(arg0, arg1) => f
                .debug_tuple("Call")
                .field(&((*arg0) as usize))
                .field(arg1)
                .finish(),
            Self::CallInterpret(arg0) => f.debug_tuple("CallInterpret").field(arg0).finish(),
            Self::PrimitiveOpn(arg0) => f.debug_tuple("PrimitiveOpn").field(arg0).finish(),
        }
    }
}

impl PartialEq for InstructionKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::PushVarInput(l0, l1), Self::PushVarInput(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::PushPrimitive(l0), Self::PushPrimitive(r0)) => l0 == r0,
            (Self::Call(l0, l1), Self::Call(r0, r1)) => {
                ((*l0) as usize) == ((*r0) as usize) && l1 == r1
            }
            (Self::CallInterpret(l0), Self::CallInterpret(r0)) => l0 == r0,
            (Self::PrimitiveOpn(l0), Self::PrimitiveOpn(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for InstructionKind {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveOpn {
    Add,
    AddAssign(u16),
    And,
    AndAssign(u16),
    BitAnd,
    BitAndAssign(u16),
    BitOr,
    BitOrAssign(u16),
    BitXor,
    BitXorAssign(u16),
    Div,
    DivAssign(u16),
    Mul,
    MulAssign(u16),
    Or,
    OrAssign(u16),
    RemEuclid,
    RemEuclidAssign(u16),
    Shl,
    ShlAssign(u16),
    Shr,
    ShrAssign(u16),
    Sub,
    SubAssign(u16),
}
