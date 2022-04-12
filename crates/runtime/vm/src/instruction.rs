mod id;
mod sheet;

pub use id::{InstructionId, InstructionSource};
pub use sheet::InstructionSheet;

use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};

use crate::*;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.src.instruction_id() == other.src.instruction_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(kind: InstructionKind, src: Arc<dyn InstructionSource>) -> Self {
        Self { kind, src }
    }

    pub fn id(&self) -> InstructionId {
        self.src.instruction_id()
    }
}

impl<
        S: InstructionSource,
        T: Deref<Target = S> + std::fmt::Debug + Send + Sync + RefUnwindSafe,
    > InstructionSource for T
{
    fn instruction_id(&self) -> InstructionId {
        let this: &S = self;
        this.instruction_id()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionKind {
    Init(InitKind),
    PushVariable {
        stack_idx: StackIdx,
        contract: EagerContract,
    },
    PushPrimitiveLiteral(PrimitiveValue),
    MembAccessCompiled {
        field_access_fp: MembAccessFp,
    },
    MembAccessInterpreted {
        field_idx: u8,
        contract: EagerContract,
    },
    RoutineCallCompiled {
        fp: RoutineLinkage,
    },
    RoutineCallInterpreted {
        routine: EntityUid,
        nargs: u8,
    },
    NewVirtualStruct {
        fields: Vec<FieldContract>,
    },
    PrimitiveOpn(PrimitiveOpn),
    Loop {
        body: Arc<InstructionSheet>,
        loop_kind: VMLoopKind,
    },
    Return,
    BreakIfFalse,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Decl,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Decl => "decl",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PrimitiveOpn {
    PureBinary(PureBinaryOpr),
    Assign(Option<PureBinaryOpr>),
    Unary,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryOpr {
    Pure(PureBinaryOpr),
    Assign(Option<PureBinaryOpr>),
}

impl BinaryOpr {
    pub fn lopd_contract(self) -> EagerContract {
        todo!()
    }

    pub fn ropd_contract(self) -> EagerContract {
        todo!()
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::Pure(pure_binary_opr) => pure_binary_opr.spaced_code(),
            BinaryOpr::Assign(opt_binary_opr) => {
                if let Some(binary_opr) = opt_binary_opr {
                    match binary_opr {
                        PureBinaryOpr::Add => " += ",
                        PureBinaryOpr::And => " &&= ",
                        PureBinaryOpr::BitAnd => " &= ",
                        PureBinaryOpr::BitOr => " |= ",
                        PureBinaryOpr::BitXor => " ^= ",
                        PureBinaryOpr::Div => " /= ",
                        PureBinaryOpr::Greater => todo!(),
                        PureBinaryOpr::Leq => todo!(),
                        PureBinaryOpr::Less => todo!(),
                        PureBinaryOpr::Mul => todo!(),
                        PureBinaryOpr::Neq => todo!(),
                        PureBinaryOpr::RemEuclid => todo!(),
                        PureBinaryOpr::Or => todo!(),
                        PureBinaryOpr::Power => todo!(),
                        PureBinaryOpr::Shl => todo!(),
                        PureBinaryOpr::Shr => todo!(),
                        PureBinaryOpr::Sub => " -= ",
                        PureBinaryOpr::Eq | PureBinaryOpr::Geq => panic!(),
                    }
                } else {
                    " = "
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PureBinaryOpr {
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

impl PureBinaryOpr {
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
            PureBinaryOpr::Add => match lopd {
                PrimitiveValue::I32(a) => (a + ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a + ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::And => match lopd {
                PrimitiveValue::Bool(a) => (a && ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitAnd => match lopd {
                PrimitiveValue::B32(a) => (a & ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitOr => match lopd {
                PrimitiveValue::B32(a) => (a | ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitXor => match lopd {
                PrimitiveValue::B32(a) => (a ^ ropd.as_b32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Div => match lopd {
                PrimitiveValue::I32(a) => (a / ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a / ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Eq => match lopd {
                PrimitiveValue::I32(a) => (a == ropd.as_i32()?).into(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::Void => todo!(),
            },
            PureBinaryOpr::Geq => match lopd {
                PrimitiveValue::I32(a) => (a >= ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a >= ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Greater => match lopd {
                PrimitiveValue::I32(a) => (a > ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a > ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Leq => match lopd {
                PrimitiveValue::I32(a) => (a <= ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a <= ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Less => match lopd {
                PrimitiveValue::I32(a) => (a < ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a < ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Mul => match lopd {
                PrimitiveValue::I32(a) => (a * ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a * ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Neq => match lopd {
                PrimitiveValue::I32(a) => (a != ropd.as_i32()?).into(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::Void => todo!(),
            },
            PureBinaryOpr::Or => match lopd {
                PrimitiveValue::Bool(a) => (a || ropd.as_bool()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Power => match lopd {
                PrimitiveValue::I32(a) => (a.pow(
                    ropd.as_i32()?
                        .try_into()
                        .map_err(|_| error!("expect positive power"))?,
                ))
                .into(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(_) => todo!(),
                PrimitiveValue::Void => todo!(),
            },
            PureBinaryOpr::RemEuclid => match lopd {
                PrimitiveValue::I32(a) => a.rem_euclid(ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => a.rem_euclid(ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shl => match lopd {
                PrimitiveValue::B32(a) => (a << ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shr => match lopd {
                PrimitiveValue::B32(a) => (a >> ropd.as_i32()?).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Sub => match lopd {
                PrimitiveValue::I32(a) => (a - ropd.as_i32()?).into(),
                PrimitiveValue::F32(a) => (a - ropd.as_f32()?).into(),
                _ => no_such_opn!(),
            },
        })
    }

    pub fn code(&self) -> &'static str {
        match self {
            PureBinaryOpr::Less => "<",
            PureBinaryOpr::Leq => "<=",
            PureBinaryOpr::Greater => ">",
            PureBinaryOpr::Geq => ">=",
            PureBinaryOpr::Neq => "!=",
            PureBinaryOpr::Eq => "==",
            PureBinaryOpr::Shl => "<<",
            PureBinaryOpr::Shr => ">>",
            PureBinaryOpr::Add => "+",
            PureBinaryOpr::Sub => "-",
            PureBinaryOpr::Mul => "*",
            PureBinaryOpr::Div => "/",
            PureBinaryOpr::And => "&&",
            PureBinaryOpr::BitAnd => "&",
            PureBinaryOpr::Or => "||",
            PureBinaryOpr::Power => "**",
            PureBinaryOpr::BitXor => "^",
            PureBinaryOpr::BitOr => "|",
            PureBinaryOpr::RemEuclid => "%",
        }
    }

    pub fn spaced_code(&self) -> &'static str {
        match self {
            PureBinaryOpr::Less => " < ",
            PureBinaryOpr::Leq => " <= ",
            PureBinaryOpr::Greater => " > ",
            PureBinaryOpr::Geq => " >= ",
            PureBinaryOpr::Neq => " != ",
            PureBinaryOpr::Eq => " == ",
            PureBinaryOpr::Shl => " << ",
            PureBinaryOpr::Shr => " >>",
            PureBinaryOpr::Add => " + ",
            PureBinaryOpr::Sub => " - ",
            PureBinaryOpr::Mul => " * ",
            PureBinaryOpr::Div => " / ",
            PureBinaryOpr::And => " && ",
            PureBinaryOpr::BitAnd => " & ",
            PureBinaryOpr::Or => " || ",
            PureBinaryOpr::Power => " ** ",
            PureBinaryOpr::BitXor => " ^ ",
            PureBinaryOpr::BitOr => " | ",
            PureBinaryOpr::RemEuclid => " % ",
        }
    }
}
