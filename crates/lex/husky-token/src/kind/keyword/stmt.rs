use std::ops::Deref;

use crate::{Keyword, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
    Match,
    For,
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
    Require,
}

impl StmtKeyword {
    pub const fn as_str(&self) -> &'static str {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Match => "match",
            StmtKeyword::For => "for",
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
        self.as_str()
    }
}

impl const From<StmtKeyword> for Keyword {
    fn from(stmt: StmtKeyword) -> Self {
        Keyword::Stmt(stmt)
    }
}

impl const Into<TokenKind> for StmtKeyword {
    fn into(self) -> TokenKind {
        TokenKind::Keyword(self.into())
    }
}
