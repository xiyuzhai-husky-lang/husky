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
    Primitive(PrimitiveOpn),
    Return,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveOpn {
    Binary(PrimitiveBinaryFunc),
    BinaryAssign {
        dst_idx: u16,
        func: PrimitiveBinaryFunc,
    },
    Unary,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveBinaryFunc {
    Add,
    And,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Mul,
    RemEuclid,
    Or,
    Shl,
    Shr,
    Sub,
}

impl PrimitiveBinaryFunc {
    pub fn call(
        &self,
        lopd: PrimitiveValue,
        ropd: PrimitiveValue,
    ) -> InterpretResult<PrimitiveValue> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }

        Ok(match self {
            PrimitiveBinaryFunc::Add => match lopd {
                PrimitiveValue::I32(a) => (a + ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a + ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::And => match lopd {
                PrimitiveValue::Bool(a) => (a && ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::BitAnd => match lopd {
                PrimitiveValue::B32(a) => (a & ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::BitOr => match lopd {
                PrimitiveValue::B32(a) => (a | ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::BitXor => match lopd {
                PrimitiveValue::B32(a) => (a ^ ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Div => match lopd {
                PrimitiveValue::I32(a) => (a / ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a / ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Mul => match lopd {
                PrimitiveValue::I32(a) => (a * ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a * ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Or => match lopd {
                PrimitiveValue::Bool(a) => (a || ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::RemEuclid => match lopd {
                PrimitiveValue::I32(a) => a.rem_euclid(ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => a.rem_euclid(ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Shl => match lopd {
                PrimitiveValue::B32(a) => (a << ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Shr => match lopd {
                PrimitiveValue::B32(a) => (a >> ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            PrimitiveBinaryFunc::Sub => match lopd {
                PrimitiveValue::I32(a) => (a - ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a - ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
        })
    }
}

// PrimitiveBinaryFunc::Add => match lopd {
//     PrimitiveValue::I32(a) => (a + ropd.as_i32()?).into(),
//     PrimitiveValue::F32(a) => (a + ropd.as_f32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::AddAssign { dst_idx } => {
//     match self.var(dst_idx.into()?? {
//         PrimitiveValue::I32(a) => {
//             self.binary_assign(dst_idx, (a + ropd.as_i32()?).into()
//         }
//         PrimitiveValue::F32(a) => {
//             self.binary_assign(dst_idx, (a + ropd.as_f32()?).into()
//         }
//         _ => no_such_opn!(),
//     }
// }
// PrimitiveBinaryFunc::And => match lopd {
//     PrimitiveValue::Bool(a) => (a && ropd.as_bool()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::AndAssign { dst_idx } => match lopd {
//     PrimitiveValue::Bool(a) => {
//         self.binary_assign(dst_idx, (a && ropd.as_bool()?).into()
//     }
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitAnd => match lopd {
//     PrimitiveValue::B32(a) => (a & self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitAndAssign { dst_idx } => match lopd {
//     PrimitiveValue::B32(a) => self.binary_assign(dst_idx, (a & self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitOr => match lopd {
//     PrimitiveValue::B32(a) => (a | self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitOrAssign { dst_idx } => match lopd {
//     PrimitiveValue::B32(a) => self.binary_assign(dst_idx, (a | self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitXor => match lopd {
//     PrimitiveValue::B32(a) => (a ^ self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::BitXorAssign { dst_idx } => match lopd {
//     PrimitiveValue::B32(a) => self.binary_assign(dst_idx, (a ^ self.as_u32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::Div => match lopd {
//     PrimitiveValue::I32(a) => (a / ropd.as_i32()?).into(),
//     PrimitiveValue::F32(a) => (a / ropd.as_f32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::DivAssign { dst_idx } => {
//     match self.var(dst_idx.into()?? {
//         PrimitiveValue::I32(a) => {
//             self.binary_assign(dst_idx, (a / ropd.as_i32()?).into()
//         }
//         PrimitiveValue::F32(a) => {
//             self.binary_assign(dst_idx, (a / ropd.as_f32()?).into()
//         }
//         _ => no_such_opn!(),
//     }
// }
// PrimitiveBinaryFunc::Mul => match lopd {
//     PrimitiveValue::I32(a) => (a * ropd.as_i32()?).into(),
//     PrimitiveValue::F32(a) => (a * ropd.as_f32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::MulAssign { dst_idx } => {
//     match self.var(dst_idx.into()?? {
//         PrimitiveValue::I32(a) => {
//             self.binary_assign(dst_idx, (a * ropd.as_i32()?).into()
//         }
//         PrimitiveValue::F32(a) => {
//             self.binary_assign(dst_idx, (a * ropd.as_f32()?).into()
//         }
//         _ => no_such_opn!(),
//     }
// }
// PrimitiveBinaryFunc::Or => match lopd {
//     PrimitiveValue::Bool(a) => (a || ropd.as_bool()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::OrAssign { dst_idx } => match lopd {
//     PrimitiveValue::Bool(a) => {
//         self.binary_assign(dst_idx, (a || ropd.as_bool()?).into()
//     }
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::RemEuclid => match lopd {
//     PrimitiveValue::I32(a) => a.rem_euclid(ropd.as_i32()?).into(),
//     PrimitiveValue::F32(a) => a.rem_euclid(ropd.as_f32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::RemEuclidAssign { dst_idx } => {
//     match self.var(dst_idx.into()?? {
//         PrimitiveValue::I32(a) => {
//             self.binary_assign(dst_idx, a.rem_euclid(ropd.as_i32()?).into()
//         }
//         PrimitiveValue::F32(a) => {
//             self.binary_assign(dst_idx, a.rem_euclid(ropd.as_f32()?).into()
//         }
//         _ => no_such_opn!(),
//     }
// }
// PrimitiveBinaryFunc::Shl => match lopd {
//     PrimitiveValue::B32(a) => (a << ropd.as_i32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::ShlAssign { dst_idx } => match lopd {
//     PrimitiveValue::B32(a) => {
//         self.binary_assign(dst_idx, (a << ropd.as_i32()?).into()
//     }
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::Shr => match lopd {
//     PrimitiveValue::B32(a) => (a >> ropd.as_i32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::ShrAssign { dst_idx } => match lopd {
//     PrimitiveValue::B32(a) => {
//         self.binary_assign(dst_idx, (a >> ropd.as_i32()?).into()
//     }
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::Sub => match lopd {
//     PrimitiveValue::I32(a) => (a - ropd.as_i32()?).into(),
//     PrimitiveValue::F32(a) => (a - ropd.as_f32()?).into(),
//     _ => no_such_opn!(),
// },
// PrimitiveBinaryFunc::SubAssign { dst_idx } => {
//     match self.var(dst_idx.into()?? {
//         PrimitiveValue::I32(a) => {
//             self.binary_assign(dst_idx, (a - ropd.as_i32()?).into()
//         }
//         PrimitiveValue::F32(a) => {
//             self.binary_assign(dst_idx, (a - ropd.as_f32()?).into()
//         }
//         _ => no_such_opn!(),
//     }
// }
