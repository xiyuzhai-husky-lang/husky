use syntax_types::*;

use crate::*;

pub enum Instruction {
    PushVarInput(InputContract, u16),
    PushPrimitive(PrimitiveValue),
    Call(fn(&mut VirtualStack) -> VirtualStackResult<()>, u16),
    BuiltinArithmetic(BuiltinArithmeticOpn),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BuiltinArithmeticOpn {
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
