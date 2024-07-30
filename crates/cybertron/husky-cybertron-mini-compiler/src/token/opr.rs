#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Opr {
    Prefix(PrefixOpr),
    Binary(BinaryOpr),
    Suffix(SuffixOpr),
}

impl std::fmt::Debug for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
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
    /// # prefix
    pub const NOT: Self = Opr::Prefix(PrefixOpr::Not);
    pub const PLUS: Self = Opr::Prefix(PrefixOpr::Plus);
    pub const MINUS: Self = Opr::Prefix(PrefixOpr::Minus);
    /// # binary
    pub const ADD: Self = Opr::Binary(BinaryOpr::Add);
    pub const SUB: Self = Opr::Binary(BinaryOpr::Sub);
    pub const MUL: Self = Opr::Binary(BinaryOpr::Mul);
    pub const DIV: Self = Opr::Binary(BinaryOpr::Div);
    pub const ASSIGN: Self = Opr::Binary(BinaryOpr::Assign);
    /// # suffix
    pub const INCR: Self = Opr::Suffix(SuffixOpr::Incr);
    pub const DECR: Self = Opr::Suffix(SuffixOpr::Decr);
}

impl Opr {
    pub fn data(self) -> &'static str {
        match self {
            Opr::Prefix(slf) => slf.data(),
            Opr::Binary(slf) => slf.data(),
            Opr::Suffix(slf) => slf.data(),
        }
    }
}

/// # binary

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

impl BinaryOpr {
    pub fn data(self) -> &'static str {
        match self {
            BinaryOpr::Add => "+(add)",
            BinaryOpr::Sub => "-(sub)",
            BinaryOpr::Mul => "*",
            BinaryOpr::Div => "/",
            BinaryOpr::Assign => "=",
        }
    }
}

/// # prefix

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrefixOpr {
    Not,
    Minus,
    Plus,
}

impl std::fmt::Debug for PrefixOpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}

impl PrefixOpr {
    pub fn data(self) -> &'static str {
        match self {
            PrefixOpr::Not => "!",
            PrefixOpr::Plus => "+(plus)",
            PrefixOpr::Minus => "-(minus)",
        }
    }

    pub fn precedence(self) -> Precedence {
        Precedence::Prefix
    }
}

/// # suffix

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,
    Decr,
}

impl SuffixOpr {
    pub fn data(self) -> &'static str {
        match self {
            SuffixOpr::Incr => "++",
            SuffixOpr::Decr => "--",
        }
    }
}
