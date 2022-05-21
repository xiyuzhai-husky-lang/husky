use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Routine(RoutineKeyword),
    Type(TyKeyword),
    Stmt(StmtKeyword),
    Def,
    Main,
    Use,
    Mod,
    Visual,
}

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::Config(keyword) => keyword.as_str(),
            Keyword::Routine(keyword) => keyword.as_str(),
            Keyword::Type(keyword) => keyword.as_str(),
            Keyword::Stmt(keyword) => keyword.as_str(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Def => "def",
            Keyword::Main => "main",
            Keyword::Visual => "visual",
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<ConfigKeyword> for Keyword {
    fn from(kw: ConfigKeyword) -> Self {
        Self::Config(kw)
    }
}

impl From<RoutineKeyword> for Keyword {
    fn from(kw: RoutineKeyword) -> Self {
        Keyword::Routine(kw)
    }
}

impl From<TyKeyword> for Keyword {
    fn from(kw: TyKeyword) -> Self {
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

impl ConfigKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKeyword::Dataset => "dataset",
        }
    }
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RoutineKeyword {
    Proc,
    Func,
}

impl RoutineKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoutineKeyword::Proc => "proc",
            RoutineKeyword::Func => "func",
        }
    }
}

impl Deref for RoutineKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TyKeyword {
    Struct,
    Rename,
    Enum,
    Props,
    Record,
}

impl TyKeyword {
    fn as_str(&self) -> &'static str {
        match self {
            TyKeyword::Struct => "struct",
            TyKeyword::Rename => "rename",
            TyKeyword::Enum => "enum",
            TyKeyword::Props => "props",
            TyKeyword::Record => "record",
        }
    }
}

impl Deref for TyKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
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

impl StmtKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
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

impl Deref for StmtKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
