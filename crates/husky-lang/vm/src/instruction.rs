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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PrimitiveOpn {
    Binary(BinaryOpr),
    BinaryAssign { dst_idx: u16, func: BinaryOpr },
    Unary,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryOpr {
    Add,
    And,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Eq,
    Geq,
    Greater,
    Leq,
    Less,
    Mul,
    Neq,
    RemEuclid,
    Or,
    Power,
    Shl,
    Shr,
    Sub,
}

impl BinaryOpr {
    pub fn act_on_primitives(
        &self,
        lopd: PrimitiveValue,
        ropd: PrimitiveValue,
    ) -> VMResult<PrimitiveValue> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }

        Ok(match self {
            BinaryOpr::Add => match lopd {
                PrimitiveValue::I32(a) => (a + ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a + ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::And => match lopd {
                PrimitiveValue::Bool(a) => (a && ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::BitAnd => match lopd {
                PrimitiveValue::B32(a) => (a & ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::BitOr => match lopd {
                PrimitiveValue::B32(a) => (a | ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::BitXor => match lopd {
                PrimitiveValue::B32(a) => (a ^ ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Div => match lopd {
                PrimitiveValue::I32(a) => (a / ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a / ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Eq => match lopd {
                PrimitiveValue::I32(a) => (a == ropd.as_i32()?).into(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::Void => todo!(),
            },
            BinaryOpr::Geq => todo!(),
            BinaryOpr::Greater => todo!(),
            BinaryOpr::Leq => todo!(),
            BinaryOpr::Less => todo!(),
            BinaryOpr::Mul => match lopd {
                PrimitiveValue::I32(a) => (a * ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a * ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Neq => todo!(),
            BinaryOpr::Or => match lopd {
                PrimitiveValue::Bool(a) => (a || ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Power => todo!(),
            BinaryOpr::RemEuclid => match lopd {
                PrimitiveValue::I32(a) => a.rem_euclid(ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => a.rem_euclid(ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Shl => match lopd {
                PrimitiveValue::B32(a) => (a << ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Shr => match lopd {
                PrimitiveValue::B32(a) => (a >> ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            BinaryOpr::Sub => match lopd {
                PrimitiveValue::I32(a) => (a - ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a - ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
        })
    }

    pub fn code(&self) -> &'static str {
        match self {
            BinaryOpr::Less => "<",
            BinaryOpr::Leq => "<=",
            BinaryOpr::Greater => ">",
            BinaryOpr::Geq => ">=",
            BinaryOpr::Neq => "!=",
            BinaryOpr::Eq => "==",
            BinaryOpr::Shl => "<<",
            BinaryOpr::Shr => ">>",
            BinaryOpr::Add => "+",
            BinaryOpr::Sub => "-",
            BinaryOpr::Mul => "*",
            BinaryOpr::Div => "/",
            BinaryOpr::And => "&&",
            BinaryOpr::BitAnd => "&",
            BinaryOpr::Or => "||",
            BinaryOpr::Power => "**",
            BinaryOpr::BitXor => "^",
            BinaryOpr::BitOr => "|",
            BinaryOpr::RemEuclid => "%",
        }
    }

    pub fn spaced_code(&self) -> &'static str {
        match self {
            BinaryOpr::Less => " < ",
            BinaryOpr::Leq => " <= ",
            BinaryOpr::Greater => " > ",
            BinaryOpr::Geq => " >= ",
            BinaryOpr::Neq => " != ",
            BinaryOpr::Eq => " == ",
            BinaryOpr::Shl => " << ",
            BinaryOpr::Shr => " >>",
            BinaryOpr::Add => " + ",
            BinaryOpr::Sub => " - ",
            BinaryOpr::Mul => " * ",
            BinaryOpr::Div => " / ",
            BinaryOpr::And => " && ",
            BinaryOpr::BitAnd => " & ",
            BinaryOpr::Or => " || ",
            BinaryOpr::Power => " ** ",
            BinaryOpr::BitXor => " ^ ",
            BinaryOpr::BitOr => " | ",
            BinaryOpr::RemEuclid => " % ",
        }
    }
}
