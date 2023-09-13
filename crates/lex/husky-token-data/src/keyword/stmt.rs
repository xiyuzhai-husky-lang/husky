use std::ops::Deref;

use crate::{Keyword, TokenData};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    If,
    Elif,
    Else,
    Match,
    /// `for` token not in an impl decl
    NonImplFor,
    /// `forext`
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
    Require,
}

impl StmtKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Match => "match",
            StmtKeyword::NonImplFor => "for",
            StmtKeyword::ForExt => "forext",
            StmtKeyword::While => "while",
            StmtKeyword::Do => "do",
            StmtKeyword::Break => "break",
            StmtKeyword::Return => "return",
            StmtKeyword::Assert => "assert",
            StmtKeyword::Require => "require",
        }
    }
}

impl Deref for StmtKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<StmtKeyword> for TokenData {
    fn from(val: StmtKeyword) -> Self {
        TokenData::Keyword(val.into())
    }
}
