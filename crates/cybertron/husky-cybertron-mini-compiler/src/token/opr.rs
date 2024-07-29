#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Opr {
    Binary(BinaryOpr),
}

impl std::fmt::Debug for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryOpr {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

impl std::fmt::Debug for BinaryOpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}

impl BinaryOpr {
    pub fn precedence(self) -> Precedence {
        match self {
            BinaryOpr::Add | BinaryOpr::Sub => Precedence::AddOrSub,
            BinaryOpr::Mul | BinaryOpr::Div => Precedence::MulOrDiv,
            BinaryOpr::Assign => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    AddOrSub,
    MulOrDiv,
    Prefix,
    Suffix,
}

impl Opr {
    pub const ADD: Self = Opr::Binary(BinaryOpr::Add);
    pub const SUB: Self = Opr::Binary(BinaryOpr::Sub);
    pub const MUL: Self = Opr::Binary(BinaryOpr::Mul);
    pub const DIV: Self = Opr::Binary(BinaryOpr::Div);
    pub const ASSIGN: Self = Opr::Binary(BinaryOpr::Assign);
}

impl Opr {
    pub fn data(self) -> &'static str {
        match self {
            Opr::Binary(slf) => slf.data(),
        }
    }
}

impl BinaryOpr {
    pub fn data(self) -> &'static str {
        match self {
            BinaryOpr::Add => "+",
            BinaryOpr::Sub => "-",
            BinaryOpr::Mul => "*",
            BinaryOpr::Div => "/",
            BinaryOpr::Assign => "=",
        }
    }
}
