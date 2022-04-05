use entity_syntax::RawTyKind;
use token::{Special, Token, TokenKind};
use word::{Keyword, TyKeyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawEntityKind {
    Module,
    Type(RawTyKind),
    Trait,
    Routine,
    Feature,
    Pattern,
    Literal,
}

impl RawEntityKind {
    pub fn new(keyword: Keyword, third_token: &Token) -> Option<RawEntityKind> {
        match keyword {
            Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) => None,
            Keyword::Mod => Some(RawEntityKind::Module),
            Keyword::Routine(_) => Some(RawEntityKind::Routine),
            Keyword::Type(keyword) => Some(RawEntityKind::Type(keyword.into())),
            Keyword::Def => Some(match third_token.kind {
                TokenKind::Special(Special::LCurl) => RawEntityKind::Pattern,
                _ => RawEntityKind::Feature,
            }),
            Keyword::Main => todo!(),
        }
    }
}
