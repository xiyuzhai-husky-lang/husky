use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryClosedOpr {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Mul,
    RemEuclid,
    Power,
    Sub,
}

impl BinaryClosedOpr {
    pub fn rust_trait_method_name(self) -> &'static str {
        match self {
            BinaryClosedOpr::Add => "add",
            BinaryClosedOpr::BitAnd => todo!(),
            BinaryClosedOpr::BitOr => todo!(),
            BinaryClosedOpr::BitXor => todo!(),
            BinaryClosedOpr::Div => todo!(),
            BinaryClosedOpr::Mul => todo!(),
            BinaryClosedOpr::RemEuclid => todo!(),
            BinaryClosedOpr::Power => todo!(),
            BinaryClosedOpr::Sub => "sub",
        }
    }

    pub fn husky_code(&self) -> &'static str {
        match self {
            BinaryClosedOpr::Add => "+",
            BinaryClosedOpr::BitAnd => "&",
            BinaryClosedOpr::BitOr => "|",
            BinaryClosedOpr::BitXor => "^",
            BinaryClosedOpr::Div => "/",
            BinaryClosedOpr::Mul => "*",
            BinaryClosedOpr::Power => "**",
            BinaryClosedOpr::RemEuclid => "%",
            BinaryClosedOpr::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
        match self {
            BinaryClosedOpr::Add => " + ",
            BinaryClosedOpr::Sub => " - ",
            BinaryClosedOpr::Mul => " * ",
            BinaryClosedOpr::Div => " / ",
            BinaryClosedOpr::BitAnd => " & ",
            BinaryClosedOpr::Power => " ** ",
            BinaryClosedOpr::BitXor => " ^ ",
            BinaryClosedOpr::BitOr => " | ",
            BinaryClosedOpr::RemEuclid => " % ",
        }
    }
}

impl HasPrecedence for BinaryClosedOpr {
    #[inline(always)]
    fn precedence(self) -> Precedence {
        match self {
            BinaryClosedOpr::BitAnd => Precedence::BitAnd,
            BinaryClosedOpr::BitOr => Precedence::BitOr,
            BinaryClosedOpr::BitXor => Precedence::BitXor,
            BinaryClosedOpr::Mul | BinaryClosedOpr::Div | BinaryClosedOpr::RemEuclid => {
                Precedence::Multiplicative
            }
            BinaryClosedOpr::Add | BinaryClosedOpr::Sub => Precedence::Additive,
            BinaryClosedOpr::Power => Precedence::Power,
        }
    }
}

// impl From<SynBinaryOpr> for Precedence {
//     fn from(binary: SynBinaryOpr) -> Self {
//         match binary {
//             SynBinaryOpr::Closed(opr) => todo!(),
//             SynBinaryOpr::Shift(opr) => match opr {
//                 BinaryShiftOpr::Shl | BinaryShiftOpr::Shr => Precedence::Shift,
//             },
//             SynBinaryOpr::Comparison(cmp_opr) =>
//             SynBinaryOpr::Assign | SynBinaryOpr::AssignClosed(_) | SynBinaryOpr::AssignShift(_) => {
//                 Precedence::Assign
//             }
//             SynBinaryOpr::ScopeResolution => Precedence::ScopeResolution,
//             SynBinaryOpr::Curry => Precedence::Curry,
//             SynBinaryOpr::As => Precedence::As,
//             SynBinaryOpr::Ins => Precedence::Is,
//             SynBinaryOpr::In => todo!(),
//         }
//     }
// }
