use husky_print_utils::p;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOpr {
    Pure(PureBinaryOpr),
    Assign(Option<PureBinaryOpr>),
}

impl Into<RawOpnVariant> for BinaryOpr {
    fn into(self) -> RawOpnVariant {
        RawOpnVariant::Binary(self)
    }
}

impl BinaryOpr {
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
        lopd: PrimitiveValueData,
        ropd: PrimitiveValueData,
    ) -> __VMResult<PrimitiveValueData> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }

        Ok(match self {
            PureBinaryOpr::Add => match lopd {
                PrimitiveValueData::I32(a) => (a + ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a + ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::And => match lopd {
                PrimitiveValueData::Bool(a) => (a && ropd.take_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitAnd => match lopd {
                PrimitiveValueData::B32(a) => (a & ropd.take_b32()).into(),
                PrimitiveValueData::B64(a) => (a & ropd.take_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::BitOr => match lopd {
                PrimitiveValueData::B32(a) => (a | ropd.take_b32()).into(),
                PrimitiveValueData::B64(a) => (a | ropd.take_b64()).into(),
                _ => {
                    p!(lopd);
                    no_such_opn!()
                }
            },
            PureBinaryOpr::BitXor => match lopd {
                PrimitiveValueData::B32(a) => (a ^ ropd.take_b32()).into(),
                PrimitiveValueData::B64(a) => (a ^ ropd.take_b64()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Div => match lopd {
                PrimitiveValueData::I32(a) => (a / ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a / ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Eq => (lopd == ropd).into(),
            PureBinaryOpr::Geq => match lopd {
                PrimitiveValueData::I32(a) => (a >= ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a >= ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Greater => match lopd {
                PrimitiveValueData::I32(a) => (a > ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a > ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Leq => match lopd {
                PrimitiveValueData::I32(a) => (a <= ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a <= ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Less => match lopd {
                PrimitiveValueData::I32(a) => (a < ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a < ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Mul => match lopd {
                PrimitiveValueData::I32(a) => (a * ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a * ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Neq => match lopd {
                PrimitiveValueData::I32(a) => (a != ropd.take_i32()).into(),
                PrimitiveValueData::F32(f) => (f != ropd.take_f32()).into(),
                PrimitiveValueData::B32(b) => (b != ropd.take_b32()).into(),
                PrimitiveValueData::B64(b) => (b != ropd.take_b64()).into(),
                PrimitiveValueData::Bool(b) => (b != ropd.take_bool()).into(),
                PrimitiveValueData::Void(_) => false.into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Or => match lopd {
                PrimitiveValueData::Bool(a) => (a || ropd.take_bool()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Power => match lopd {
                PrimitiveValueData::I32(a) => (a.pow(
                    ropd.take_i32()
                        .try_into()
                        .map_err(|_| vm_runtime_error!("expect positive power"))?,
                ))
                .into(),
                PrimitiveValueData::F32(_) => todo!(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::RemEuclid => match lopd {
                PrimitiveValueData::I32(a) => a.rem_euclid(ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => a.rem_euclid(ropd.take_f32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shl => match lopd {
                PrimitiveValueData::B32(a) => (a << ropd.take_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Shr => match lopd {
                PrimitiveValueData::B32(a) => (a >> ropd.take_i32()).into(),
                _ => no_such_opn!(),
            },
            PureBinaryOpr::Sub => match lopd {
                PrimitiveValueData::I32(a) => (a - ropd.take_i32()).into(),
                PrimitiveValueData::F32(a) => (a - ropd.take_f32()).into(),
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
