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
                PrimitiveValue::I32(a) => (a + ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a + ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::And => match lopd {
                PrimitiveValue::Bool(a) => (a && ropd.as_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitAnd => match lopd {
                PrimitiveValue::B32(a) => (a & ropd.as_b32()).into(),
                PrimitiveValue::B64(a) => (a & ropd.as_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitOr => match lopd {
                PrimitiveValue::B32(a) => (a | ropd.as_b32()).into(),
                PrimitiveValue::B64(a) => (a | ropd.as_b64()).into(),
                _ => {
                    p!(lopd);
                    no_such_opn!()
                }
            },
            PureBinaryOpr::BitXor => match lopd {
                PrimitiveValue::B32(a) => (a ^ ropd.as_b32()).into(),
                PrimitiveValue::B64(a) => (a ^ ropd.as_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Div => match lopd {
                PrimitiveValue::I32(a) => (a / ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a / ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Eq => match lopd {
                PrimitiveValue::I32(i) => (i == ropd.as_i32()).into(),
                PrimitiveValue::F32(f) => (f == ropd.as_f32()).into(),
                PrimitiveValue::B32(b) => (b == ropd.as_b32()).into(),
                PrimitiveValue::B64(b) => (b == ropd.as_b64()).into(),
                PrimitiveValue::Bool(b) => (b == ropd.as_bool()).into(),
                PrimitiveValue::Void => true.into(),
            },
            PureBinaryOpr::Geq => match lopd {
                PrimitiveValue::I32(a) => (a >= ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a >= ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Greater => match lopd {
                PrimitiveValue::I32(a) => (a > ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a > ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Leq => match lopd {
                PrimitiveValue::I32(a) => (a <= ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a <= ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Less => match lopd {
                PrimitiveValue::I32(a) => (a < ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a < ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Mul => match lopd {
                PrimitiveValue::I32(a) => (a * ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a * ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Neq => match lopd {
                PrimitiveValue::I32(a) => (a != ropd.as_i32()).into(),
                PrimitiveValue::F32(f) => (f != ropd.as_f32()).into(),
                PrimitiveValue::B32(b) => (b != ropd.as_b32()).into(),
                PrimitiveValue::B64(b) => (b != ropd.as_b64()).into(),
                PrimitiveValue::Bool(b) => (b != ropd.as_bool()).into(),
                PrimitiveValue::Void => false.into(),
            },
            PureBinaryOpr::Or => match lopd {
                PrimitiveValue::Bool(a) => (a || ropd.as_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Power => match lopd {
                PrimitiveValue::I32(a) => (a.pow(
                    ropd.as_i32()
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
                PrimitiveValue::I32(a) => a.rem_euclid(ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => a.rem_euclid(ropd.as_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shl => match lopd {
                PrimitiveValue::B32(a) => (a << ropd.as_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shr => match lopd {
                PrimitiveValue::B32(a) => (a >> ropd.as_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Sub => match lopd {
                PrimitiveValue::I32(a) => (a - ropd.as_i32()).into(),
                PrimitiveValue::F32(a) => (a - ropd.as_f32()).into(),
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
