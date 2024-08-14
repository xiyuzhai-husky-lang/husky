#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Opr {
    Prefix(PrefixOpr),
    Binary(BinaryOpr),
    Suffix(SuffixOpr),
}

impl std::fmt::Debug for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.repr()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    EqNe,
    AddOrSub,
    MulOrDiv,
    Prefix,
    Suffix,
    TypeIs,
    ScopeResolution,
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
    pub const EQ: Self = Opr::Binary(BinaryOpr::Eq);
    pub const TYPE_IS: Self = Opr::Binary(BinaryOpr::TypeIs);
    pub const SCOPE_RESOLUTION: Self = Opr::Binary(BinaryOpr::ScopeResolution);
    /// # suffix
    pub const INCR: Self = Opr::Suffix(SuffixOpr::Incr);
    pub const DECR: Self = Opr::Suffix(SuffixOpr::Decr);
}

impl Opr {
    pub fn repr(self) -> &'static str {
        match self {
            Opr::Prefix(slf) => slf.repr(),
            Opr::Binary(slf) => slf.repr(),
            Opr::Suffix(slf) => slf.repr(),
        }
    }

    pub fn repr_short(self) -> &'static str {
        match self {
            Opr::Prefix(slf) => slf.repr_short(),
            Opr::Binary(slf) => slf.repr_short(),
            Opr::Suffix(slf) => slf.repr(),
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
    Eq,
    Ne,
    ScopeResolution,
    TypeIs,
}

impl std::fmt::Debug for BinaryOpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.repr()))
    }
}

impl BinaryOpr {
    pub fn precedence(self) -> Precedence {
        match self {
            BinaryOpr::Eq | BinaryOpr::Ne => Precedence::EqNe,
            BinaryOpr::Add | BinaryOpr::Sub => Precedence::AddOrSub,
            BinaryOpr::Mul | BinaryOpr::Div => Precedence::MulOrDiv,
            BinaryOpr::Assign => todo!(),
            BinaryOpr::ScopeResolution => Precedence::ScopeResolution,
            BinaryOpr::TypeIs => Precedence::TypeIs,
        }
    }
}

impl BinaryOpr {
    pub fn repr(self) -> &'static str {
        match self {
            BinaryOpr::Add => "+(add)",
            BinaryOpr::Sub => "-(sub)",
            BinaryOpr::Mul => "*",
            BinaryOpr::Div => "/",
            BinaryOpr::Assign => "=",
            BinaryOpr::Eq => "==",
            BinaryOpr::Ne => "!=",
            BinaryOpr::ScopeResolution => "::",
            BinaryOpr::TypeIs => ":",
        }
    }

    pub fn repr_short(self) -> &'static str {
        match self {
            BinaryOpr::Add => "+",
            BinaryOpr::Sub => "-",
            BinaryOpr::Mul => "*",
            BinaryOpr::Div => "/",
            BinaryOpr::Assign => "=",
            BinaryOpr::Eq => "==",
            BinaryOpr::Ne => "!=",
            BinaryOpr::ScopeResolution => "::",
            BinaryOpr::TypeIs => ":",
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
        f.write_fmt(format_args!("`{}`", self.repr()))
    }
}

impl PrefixOpr {
    pub fn repr(self) -> &'static str {
        match self {
            PrefixOpr::Not => "!",
            PrefixOpr::Plus => "+(plus)",
            PrefixOpr::Minus => "-(minus)",
        }
    }

    pub fn repr_short(self) -> &'static str {
        match self {
            PrefixOpr::Not => "!",
            PrefixOpr::Plus => "+",
            PrefixOpr::Minus => "-",
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
    pub fn precedence(self) -> Precedence {
        Precedence::Suffix
    }
}

impl std::fmt::Debug for SuffixOpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.repr()))
    }
}

impl SuffixOpr {
    pub fn repr(self) -> &'static str {
        match self {
            SuffixOpr::Incr => "++",
            SuffixOpr::Decr => "--",
        }
    }
}
