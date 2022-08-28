pub use crate::*;
use husky_word::{Decorator, Identifier, Keyword, WordOpr, WordPattern, WordPtr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HuskyTokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    WordPattern(WordPattern),
    PrimitiveLiteral(PrimitiveLiteralData),
    Unrecognized(char),
    IllFormedLiteral(PrimitiveLiteralData),
}

impl std::fmt::Display for HuskyTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HuskyTokenKind::Decorator(decorator) => write!(f, "`{}`", decorator.as_str()),
            HuskyTokenKind::Keyword(keyword) => write!(f, "`{}`", keyword.as_str()),
            HuskyTokenKind::Identifier(ident) => write!(f, "`{}`", ident.as_str()),
            HuskyTokenKind::Special(special_token) => write!(f, "`{}`", special_token.code()),
            HuskyTokenKind::WordOpr(opr) => write!(f, "`{}`", opr.as_str()),
            HuskyTokenKind::WordPattern(patt) => write!(f, "`{}`", patt.as_str()),
            HuskyTokenKind::PrimitiveLiteral(lit) => write!(f, "`{}`", lit),
            HuskyTokenKind::Unrecognized(_) => todo!(),
            HuskyTokenKind::IllFormedLiteral(_) => todo!(),
        }
    }
}

impl HuskyTokenKind {
    pub fn left_convexity(self) -> Option<Convexity> {
        match self {
            HuskyTokenKind::Decorator(_) => todo!(),
            HuskyTokenKind::Keyword(_) => todo!(),
            HuskyTokenKind::Identifier(_) => Some(Convexity::Convex),
            HuskyTokenKind::Special(_) => todo!(),
            HuskyTokenKind::WordOpr(_) => todo!(),
            HuskyTokenKind::PrimitiveLiteral(_) => Some(Convexity::Convex),
            HuskyTokenKind::Unrecognized(_) => None,
            HuskyTokenKind::IllFormedLiteral(_) => Some(Convexity::Convex),
            HuskyTokenKind::WordPattern(_) => Some(Convexity::Convex),
        }
    }
}

impl From<SpecialToken> for HuskyTokenKind {
    fn from(special: SpecialToken) -> Self {
        HuskyTokenKind::Special(special)
    }
}
impl std::hash::Hash for HuskyTokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl From<WordPtr> for HuskyTokenKind {
    fn from(word: WordPtr) -> Self {
        match word {
            WordPtr::Keyword(keyword) => HuskyTokenKind::Keyword(keyword),
            WordPtr::Identifier(ident) => HuskyTokenKind::Identifier(ident),
            WordPtr::Opr(word_opr) => HuskyTokenKind::WordOpr(word_opr),
            WordPtr::Decorator(decorator) => HuskyTokenKind::Decorator(decorator),
            WordPtr::Pattern(patt) => HuskyTokenKind::WordPattern(patt),
        }
    }
}

impl From<Keyword> for HuskyTokenKind {
    fn from(keyword: Keyword) -> Self {
        HuskyTokenKind::Keyword(keyword)
    }
}
