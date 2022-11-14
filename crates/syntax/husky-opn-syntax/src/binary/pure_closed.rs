#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryPureClosedOpr {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    Mul,
    RemEuclid,
    Power,
    Shl,
    Shr,
    Sub,
}

impl BinaryPureClosedOpr {
    pub fn rust_trait_method_name(self) -> &'static str {
        match self {
            BinaryPureClosedOpr::Add => "add",
            BinaryPureClosedOpr::BitAnd => todo!(),
            BinaryPureClosedOpr::BitOr => todo!(),
            BinaryPureClosedOpr::BitXor => todo!(),
            BinaryPureClosedOpr::Div => todo!(),
            BinaryPureClosedOpr::Mul => todo!(),
            BinaryPureClosedOpr::RemEuclid => todo!(),
            BinaryPureClosedOpr::Power => todo!(),
            BinaryPureClosedOpr::Shl => todo!(),
            BinaryPureClosedOpr::Shr => todo!(),
            BinaryPureClosedOpr::Sub => "sub",
        }
    }

    pub fn husky_code(&self) -> &'static str {
        match self {
            BinaryPureClosedOpr::Add => "+",
            BinaryPureClosedOpr::BitAnd => "&",
            BinaryPureClosedOpr::BitOr => "|",
            BinaryPureClosedOpr::BitXor => "^",
            BinaryPureClosedOpr::Div => "/",
            BinaryPureClosedOpr::Mul => "*",
            BinaryPureClosedOpr::Power => "**",
            BinaryPureClosedOpr::RemEuclid => "%",
            BinaryPureClosedOpr::Shl => "<<",
            BinaryPureClosedOpr::Shr => ">>",
            BinaryPureClosedOpr::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
        match self {
            BinaryPureClosedOpr::Shl => " << ",
            BinaryPureClosedOpr::Shr => " >> ",
            BinaryPureClosedOpr::Add => " + ",
            BinaryPureClosedOpr::Sub => " - ",
            BinaryPureClosedOpr::Mul => " * ",
            BinaryPureClosedOpr::Div => " / ",
            BinaryPureClosedOpr::BitAnd => " & ",
            BinaryPureClosedOpr::Power => " ** ",
            BinaryPureClosedOpr::BitXor => " ^ ",
            BinaryPureClosedOpr::BitOr => " | ",
            BinaryPureClosedOpr::RemEuclid => " % ",
        }
    }
}
