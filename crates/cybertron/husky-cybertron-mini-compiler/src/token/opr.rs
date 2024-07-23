#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opr {
    Binary(BinaryOpr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOpr {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
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
