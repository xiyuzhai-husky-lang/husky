use token::{Special, Token, TokenKind};
use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Type,
    Trait,
    Routine,
    Feature,
    Pattern,
    Literal,
}

impl ScopeKind {
    pub fn new(keyword: Keyword, third_token: &Token) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) => None,
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Routine(_) => Some(ScopeKind::Routine),
            Keyword::Type(_) => Some(ScopeKind::Type),
            Keyword::Def => Some(match third_token.kind {
                TokenKind::Special(Special::LCurl) => ScopeKind::Pattern,
                _ => ScopeKind::Feature,
            }),
        }
    }
}
