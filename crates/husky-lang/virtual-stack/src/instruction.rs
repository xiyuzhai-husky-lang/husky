use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionKind {
    PushVarInput(InputContract, u16),
    PushPrimitive(PrimitiveValue),
    Call { compiled: Compiled, nargs: u16 },
    CallInterpret(Arc<Vec<Instruction>>),
    PrimitiveOpn(PrimitiveOpn),
    Return,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveOpn {
    Add,
    AddAssign { dst_idx: u16 },
    And,
    AndAssign { dst_idx: u16 },
    BitAnd,
    BitAndAssign { dst_idx: u16 },
    BitOr,
    BitOrAssign { dst_idx: u16 },
    BitXor,
    BitXorAssign { dst_idx: u16 },
    Div,
    DivAssign { dst_idx: u16 },
    Mul,
    MulAssign { dst_idx: u16 },
    Or,
    OrAssign { dst_idx: u16 },
    RemEuclid,
    RemEuclidAssign { dst_idx: u16 },
    Shl,
    ShlAssign { dst_idx: u16 },
    Shr,
    ShrAssign { dst_idx: u16 },
    Sub,
    SubAssign { dst_idx: u16 },
}
