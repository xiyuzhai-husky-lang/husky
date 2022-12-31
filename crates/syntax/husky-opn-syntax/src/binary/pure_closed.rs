#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BinaryPureClosedPunctuation {
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

impl BinaryPureClosedPunctuation {
    pub fn rust_trait_method_name(self) -> &'static str {
        match self {
            BinaryPureClosedPunctuation::Add => "add",
            BinaryPureClosedPunctuation::BitAnd => todo!(),
            BinaryPureClosedPunctuation::BitOr => todo!(),
            BinaryPureClosedPunctuation::BitXor => todo!(),
            BinaryPureClosedPunctuation::Div => todo!(),
            BinaryPureClosedPunctuation::Mul => todo!(),
            BinaryPureClosedPunctuation::RemEuclid => todo!(),
            BinaryPureClosedPunctuation::Power => todo!(),
            BinaryPureClosedPunctuation::Shl => todo!(),
            BinaryPureClosedPunctuation::Shr => todo!(),
            BinaryPureClosedPunctuation::Sub => "sub",
        }
    }

    pub fn husky_code(&self) -> &'static str {
        match self {
            BinaryPureClosedPunctuation::Add => "+",
            BinaryPureClosedPunctuation::BitAnd => "&",
            BinaryPureClosedPunctuation::BitOr => "|",
            BinaryPureClosedPunctuation::BitXor => "^",
            BinaryPureClosedPunctuation::Div => "/",
            BinaryPureClosedPunctuation::Mul => "*",
            BinaryPureClosedPunctuation::Power => "**",
            BinaryPureClosedPunctuation::RemEuclid => "%",
            BinaryPureClosedPunctuation::Shl => "<<",
            BinaryPureClosedPunctuation::Shr => ">>",
            BinaryPureClosedPunctuation::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
        match self {
            BinaryPureClosedPunctuation::Shl => " << ",
            BinaryPureClosedPunctuation::Shr => " >> ",
            BinaryPureClosedPunctuation::Add => " + ",
            BinaryPureClosedPunctuation::Sub => " - ",
            BinaryPureClosedPunctuation::Mul => " * ",
            BinaryPureClosedPunctuation::Div => " / ",
            BinaryPureClosedPunctuation::BitAnd => " & ",
            BinaryPureClosedPunctuation::Power => " ** ",
            BinaryPureClosedPunctuation::BitXor => " ^ ",
            BinaryPureClosedPunctuation::BitOr => " | ",
            BinaryPureClosedPunctuation::RemEuclid => " % ",
        }
    }
}
