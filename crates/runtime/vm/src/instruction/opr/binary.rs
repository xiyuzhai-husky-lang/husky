use print_utils::p;

use super::*;

impl From<BinaryOpr> for Opr {
    fn from(binary: BinaryOpr) -> Self {
        Self::Binary(binary)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
                        PureBinaryOpr::Mul => " *= ",
                        PureBinaryOpr::RemEuclid => " %= ",
                        PureBinaryOpr::Or => " ||= ",
                        PureBinaryOpr::Power => " **= ",
                        PureBinaryOpr::Shl => " <<= ",
                        PureBinaryOpr::Shr => " >>= ",
                        PureBinaryOpr::Sub => " -= ",
                        PureBinaryOpr::Leq
                        | PureBinaryOpr::Less
                        | PureBinaryOpr::Neq
                        | PureBinaryOpr::Greater
                        | PureBinaryOpr::Eq
                        | PureBinaryOpr::Geq => panic!(),
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
        lopd: CopyableValue,
        ropd: CopyableValue,
    ) -> VMRuntimeResult<CopyableValue> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }

        Ok(match self {
            PureBinaryOpr::Add => match lopd {
                CopyableValue::I32(a) => (a + ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a + ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::And => match lopd {
                CopyableValue::Bool(a) => (a && ropd.take_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitAnd => match lopd {
                CopyableValue::B32(a) => (a & ropd.take_b32()).into(),
                CopyableValue::B64(a) => (a & ropd.take_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitOr => match lopd {
                CopyableValue::B32(a) => (a | ropd.take_b32()).into(),
                CopyableValue::B64(a) => (a | ropd.take_b64()).into(),
                _ => {
                    p!(lopd);
                    no_such_opn!()
                }
            },
            PureBinaryOpr::BitXor => match lopd {
                CopyableValue::B32(a) => (a ^ ropd.take_b32()).into(),
                CopyableValue::B64(a) => (a ^ ropd.take_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Div => match lopd {
                CopyableValue::I32(a) => (a / ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a / ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Eq => match lopd {
                CopyableValue::I32(i) => (i == ropd.take_i32()).into(),
                CopyableValue::F32(f) => (f == ropd.take_f32()).into(),
                CopyableValue::B32(b) => (b == ropd.take_b32()).into(),
                CopyableValue::B64(b) => (b == ropd.take_b64()).into(),
                CopyableValue::Bool(b) => (b == ropd.take_bool()).into(),
                CopyableValue::Void => true.into(),
                CopyableValue::EnumKind(enum_kind) => {
                    (enum_kind.route == ropd.take_enum_kind().route).into()
                }
            },
            PureBinaryOpr::Geq => match lopd {
                CopyableValue::I32(a) => (a >= ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a >= ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Greater => match lopd {
                CopyableValue::I32(a) => (a > ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a > ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Leq => match lopd {
                CopyableValue::I32(a) => (a <= ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a <= ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Less => match lopd {
                CopyableValue::I32(a) => (a < ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a < ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Mul => match lopd {
                CopyableValue::I32(a) => (a * ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a * ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Neq => match lopd {
                CopyableValue::I32(a) => (a != ropd.take_i32()).into(),
                CopyableValue::F32(f) => (f != ropd.take_f32()).into(),
                CopyableValue::B32(b) => (b != ropd.take_b32()).into(),
                CopyableValue::B64(b) => (b != ropd.take_b64()).into(),
                CopyableValue::Bool(b) => (b != ropd.take_bool()).into(),
                CopyableValue::Void => false.into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Or => match lopd {
                CopyableValue::Bool(a) => (a || ropd.take_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Power => match lopd {
                CopyableValue::I32(a) => (a.pow(
                    ropd.take_i32()
                        .try_into()
                        .map_err(|_| vm_runtime_error!("expect positive power"))?,
                ))
                .into(),
                CopyableValue::F32(_) => todo!(),
                CopyableValue::B32(_) => todo!(),
                CopyableValue::B64(_) => todo!(),
                CopyableValue::Bool(_) => todo!(),
                CopyableValue::Void => todo!(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::RemEuclid => match lopd {
                CopyableValue::I32(a) => a.rem_euclid(ropd.take_i32()).into(),
                CopyableValue::F32(a) => a.rem_euclid(ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shl => match lopd {
                CopyableValue::B32(a) => (a << ropd.take_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shr => match lopd {
                CopyableValue::B32(a) => (a >> ropd.take_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Sub => match lopd {
                CopyableValue::I32(a) => (a - ropd.take_i32()).into(),
                CopyableValue::F32(a) => (a - ropd.take_f32()).into(),
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
            PureBinaryOpr::Shr => " >> ",
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
