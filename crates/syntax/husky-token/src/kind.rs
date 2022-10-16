pub use crate::*;
use husky_word::{Decorator, Identifier, Keyword, WordOpr, WordPattern, WordPtr};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    WordPattern(WordPattern),
    Literal(RawLiteralData),
    Unrecognized(char),
    IllFormedLiteral(RawLiteralData),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Decorator(decorator) => write!(f, "\"{}\"", decorator.as_str()),
            TokenKind::Keyword(keyword) => write!(f, "\"{}\"", keyword.as_str()),
            TokenKind::Identifier(ident) => write!(f, "\"{}\"", ident.as_str()),
            TokenKind::Special(special_token) => write!(f, "\"{}\"", special_token.code()),
            TokenKind::WordOpr(opr) => write!(f, "\"{}\"", opr.as_str()),
            TokenKind::WordPattern(patt) => write!(f, "\"{}\"", patt.as_str()),
            TokenKind::Literal(lit) => write!(f, "\"{}\"", lit),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // mom
        match self {
            TokenKind::Decorator(decorator) => write!(f, "\"{}\"", decorator.as_str()),
            TokenKind::Keyword(keyword) => write!(f, "\"{}\"", keyword.as_str()),
            TokenKind::Identifier(ident) => write!(f, "\"{}\"", ident.as_str()),
            TokenKind::Special(special_token) => write!(f, "\"{}\"", special_token.code()),
            TokenKind::WordOpr(opr) => write!(f, "\"{}\"", opr.as_str()),
            TokenKind::WordPattern(patt) => write!(f, "\"{}\"", patt.as_str()),
            TokenKind::Literal(lit) => write!(f, "\"{}\"", lit),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }
}

impl TokenKind {
    pub fn left_convexity(self) -> Option<Convexity> {
        match self {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => Some(Convexity::Convex),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Some(Convexity::Convex),
            TokenKind::Unrecognized(_) => None,
            TokenKind::IllFormedLiteral(_) => Some(Convexity::Convex),
            TokenKind::WordPattern(_) => Some(Convexity::Convex),
        }
    }

    pub fn right_convexity(self) -> Convexity {
        match self {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => Convexity::Convex,
            TokenKind::Special(special) => match special {
                SpecialToken::BinaryOpr(_) => Convexity::Concave,
                SpecialToken::LAngle => todo!(),
                SpecialToken::Leq => todo!(),
                SpecialToken::RAngle => todo!(),
                SpecialToken::Geq => todo!(),
                SpecialToken::Neq => todo!(),
                SpecialToken::DeriveAssign => todo!(),
                SpecialToken::Eq => todo!(),
                SpecialToken::Shl => todo!(),
                SpecialToken::LCurl => todo!(),
                SpecialToken::RCurl => todo!(),
                SpecialToken::LBox => todo!(),
                SpecialToken::RBox => todo!(),
                SpecialToken::LPar => todo!(),
                SpecialToken::RPar => todo!(),
                SpecialToken::Minus => todo!(),
                SpecialToken::DoubleVertical => todo!(),
                SpecialToken::BitNot => todo!(),
                SpecialToken::FieldAccess => todo!(),
                SpecialToken::Colon => todo!(),
                SpecialToken::Comma => todo!(),
                SpecialToken::Ambersand => todo!(),
                SpecialToken::Incr => todo!(),
                SpecialToken::Decr => todo!(),
                SpecialToken::Vertical => todo!(),
                SpecialToken::AddAssign => todo!(),
                SpecialToken::SubAssign => todo!(),
                SpecialToken::MulAssign => todo!(),
                SpecialToken::DivAssign => todo!(),
                SpecialToken::BitAndAssign => todo!(),
                SpecialToken::BitOrAssign => todo!(),
                SpecialToken::Exclamation => todo!(),
                SpecialToken::DoubleExclamation => todo!(),
                SpecialToken::Semicolon => todo!(),
                SpecialToken::XmlKet => todo!(),
                SpecialToken::At => todo!(),
                SpecialToken::QuestionMark => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Convexity::Convex,
            TokenKind::Unrecognized(_) => Convexity::Any,
            TokenKind::IllFormedLiteral(_) => Convexity::Convex,
            TokenKind::WordPattern(_) => Convexity::Any,
        }
    }
}

impl From<SpecialToken> for TokenKind {
    fn from(special: SpecialToken) -> Self {
        TokenKind::Special(special)
    }
}
impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl From<WordPtr> for TokenKind {
    fn from(word: WordPtr) -> Self {
        match word {
            WordPtr::Keyword(keyword) => TokenKind::Keyword(keyword),
            WordPtr::Identifier(ident) => TokenKind::Identifier(ident),
            WordPtr::Opr(word_opr) => TokenKind::WordOpr(word_opr),
            WordPtr::Decorator(decorator) => TokenKind::Decorator(decorator),
            WordPtr::Pattern(patt) => TokenKind::WordPattern(patt),
        }
    }
}

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> Self {
        TokenKind::Keyword(keyword)
    }
}
