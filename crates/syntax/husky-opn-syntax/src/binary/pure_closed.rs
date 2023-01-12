#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PureClosedBinaryOpr {
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

impl PureClosedBinaryOpr {
    pub fn rust_trait_method_name(self) -> &'static str {
        match self {
            PureClosedBinaryOpr::Add => "add",
            PureClosedBinaryOpr::BitAnd => todo!(),
            PureClosedBinaryOpr::BitOr => todo!(),
            PureClosedBinaryOpr::BitXor => todo!(),
            PureClosedBinaryOpr::Div => todo!(),
            PureClosedBinaryOpr::Mul => todo!(),
            PureClosedBinaryOpr::RemEuclid => todo!(),
            PureClosedBinaryOpr::Power => todo!(),
            PureClosedBinaryOpr::Shl => todo!(),
            PureClosedBinaryOpr::Shr => todo!(),
            PureClosedBinaryOpr::Sub => "sub",
        }
    }

    pub fn husky_code(&self) -> &'static str {
        match self {
            PureClosedBinaryOpr::Add => "+",
            PureClosedBinaryOpr::BitAnd => "&",
            PureClosedBinaryOpr::BitOr => "|",
            PureClosedBinaryOpr::BitXor => "^",
            PureClosedBinaryOpr::Div => "/",
            PureClosedBinaryOpr::Mul => "*",
            PureClosedBinaryOpr::Power => "**",
            PureClosedBinaryOpr::RemEuclid => "%",
            PureClosedBinaryOpr::Shl => "<<",
            PureClosedBinaryOpr::Shr => ">>",
            PureClosedBinaryOpr::Sub => "-",
        }
    }

    pub fn spaced_husky_code(&self) -> &'static str {
        match self {
            PureClosedBinaryOpr::Shl => " << ",
            PureClosedBinaryOpr::Shr => " >> ",
            PureClosedBinaryOpr::Add => " + ",
            PureClosedBinaryOpr::Sub => " - ",
            PureClosedBinaryOpr::Mul => " * ",
            PureClosedBinaryOpr::Div => " / ",
            PureClosedBinaryOpr::BitAnd => " & ",
            PureClosedBinaryOpr::Power => " ** ",
            PureClosedBinaryOpr::BitXor => " ^ ",
            PureClosedBinaryOpr::BitOr => " | ",
            PureClosedBinaryOpr::RemEuclid => " % ",
        }
    }
}
