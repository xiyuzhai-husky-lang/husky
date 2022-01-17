use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Func(FuncKeyword),
    Type(TypeKeyword),
    Stmt(StmtKeyword),
    Use,
    Mod,
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Keyword::Config(keyword) => keyword.deref(),
            Keyword::Func(keyword) => keyword.deref(),
            Keyword::Type(keyword) => keyword.deref(),
            Keyword::Stmt(keyword) => keyword.deref(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
        }
    }
}

impl From<ConfigKeyword> for Keyword {
    fn from(kw: ConfigKeyword) -> Self {
        Self::Config(kw)
    }
}

impl From<FuncKeyword> for Keyword {
    fn from(kw: FuncKeyword) -> Self {
        Keyword::Func(kw)
    }
}

impl From<TypeKeyword> for Keyword {
    fn from(kw: TypeKeyword) -> Self {
        Keyword::Type(kw)
    }
}

impl From<StmtKeyword> for Keyword {
    fn from(stmt: StmtKeyword) -> Self {
        Keyword::Stmt(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Dataset,
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            ConfigKeyword::Dataset => "dataset",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FuncKeyword {
    Main,
    Test,
    Proc,
    Func,
    Def,
}

impl Deref for FuncKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            FuncKeyword::Main => "main",
            FuncKeyword::Test => "test",
            FuncKeyword::Proc => "proc",
            FuncKeyword::Func => "func",
            FuncKeyword::Def => "def",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeKeyword {
    Struct,
    Rename,
    Enum,
    Props,
}

impl Deref for TypeKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            TypeKeyword::Struct => "struct",
            TypeKeyword::Rename => "rename",
            TypeKeyword::Enum => "enum",
            TypeKeyword::Props => "props",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
    Switch,
    Match,
    Case,
    DeFault,
    For,
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
}

impl Deref for StmtKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Switch => "switch",
            StmtKeyword::Match => "match",
            StmtKeyword::Case => "case",
            StmtKeyword::DeFault => "default",
            StmtKeyword::For => "for",
            StmtKeyword::ForExt => "forext",
            StmtKeyword::While => "while",
            StmtKeyword::Do => "do",
            StmtKeyword::Break => "break",
            StmtKeyword::Return => "return",
            StmtKeyword::Assert => "assert",
        }
    }
}
