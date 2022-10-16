use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOpr {
    Pure(PureBinaryOpr),
    Assign(Option<PureBinaryOpr>),
    ScopeResolution, // ::
    Curry,           // ->
    As,              // as
}

impl Into<RawOpnVariant> for BinaryOpr {
    fn into(self) -> RawOpnVariant {
        RawOpnVariant::Binary(self)
    }
}

impl BinaryOpr {
    pub fn code(self) -> &'static str {
        match self {
            BinaryOpr::Pure(pure_opr) => pure_opr.husky_code(),
            BinaryOpr::Assign(None) => "=",
            BinaryOpr::Assign(Some(pure_opr)) => match pure_opr {
                PureBinaryOpr::Add => "+=",
                PureBinaryOpr::And => unreachable!(),
                PureBinaryOpr::BitAnd => "&=",
                PureBinaryOpr::BitOr => "|=",
                PureBinaryOpr::BitXor => "^=",
                PureBinaryOpr::Div => "/=",
                PureBinaryOpr::Eq => unreachable!(),
                PureBinaryOpr::Geq => unreachable!(),
                PureBinaryOpr::Greater => unreachable!(),
                PureBinaryOpr::Leq => unreachable!(),
                PureBinaryOpr::Less => unreachable!(),
                PureBinaryOpr::Mul => "*=",
                PureBinaryOpr::Neq => "!=",
                PureBinaryOpr::Or => "||=",
                PureBinaryOpr::RemEuclid => "%=",
                PureBinaryOpr::Power => "**=",
                PureBinaryOpr::Shl => "<<=",
                PureBinaryOpr::Shr => ">>=",
                PureBinaryOpr::Sub => "-=",
            },
            BinaryOpr::ScopeResolution => "::",
            BinaryOpr::Curry => "->",
            BinaryOpr::As => todo!(),
        }
    }

    pub fn spaced_code(self) -> &'static str {
        match self {
            BinaryOpr::Pure(pure_binary_opr) => pure_binary_opr.spaced_husky_code(),
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
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => " -> ",
            BinaryOpr::As => todo!(),
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
    Or,
    RemEuclid,
    Power,
    Shl,
    Shr,
    Sub,
}

impl PureBinaryOpr {
    pub fn rust_trait_method_name(self) -> &'static str {
        match self {
            PureBinaryOpr::Add => "add",
            PureBinaryOpr::And => todo!(),
            PureBinaryOpr::BitAnd => todo!(),
            PureBinaryOpr::BitOr => todo!(),
            PureBinaryOpr::BitXor => todo!(),
            PureBinaryOpr::Div => todo!(),
            PureBinaryOpr::Eq => "eq",
            PureBinaryOpr::Geq => "ge",
            PureBinaryOpr::Greater => "gt",
            PureBinaryOpr::Leq => "le",
            PureBinaryOpr::Less => "lt",
            PureBinaryOpr::Mul => todo!(),
            PureBinaryOpr::Neq => "ne",
            PureBinaryOpr::RemEuclid => todo!(),
            PureBinaryOpr::Or => todo!(),
            PureBinaryOpr::Power => todo!(),
            PureBinaryOpr::Shl => todo!(),
            PureBinaryOpr::Shr => todo!(),
            PureBinaryOpr::Sub => "sub",
        }
    }

    pub fn husky_code(&self) -> &'static str {
        match self {
            PureBinaryOpr::Add => "+",
            PureBinaryOpr::And => "&&",
            PureBinaryOpr::BitAnd => "&",
            PureBinaryOpr::BitOr => "|",
            PureBinaryOpr::BitXor => "^",
            PureBinaryOpr::Div => "/",
            PureBinaryOpr::Eq => "==",
            PureBinaryOpr::Greater => ">",
            PureBinaryOpr::Geq => ">=",
            PureBinaryOpr::Less => "<",
            PureBinaryOpr::Leq => "<=",
            PureBinaryOpr::Mul => "*",
            PureBinaryOpr::Neq => "!=",
            PureBinaryOpr::Or => "||",
            PureBinaryOpr::Power => "**",
            PureBinaryOpr::RemEuclid => "%",
            PureBinaryOpr::Shl => "<<",
            PureBinaryOpr::Shr => ">>",
            PureBinaryOpr::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
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
