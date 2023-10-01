mod convexity;
pub mod db;
mod error;
mod keyword;
mod literal;
mod punctuation;
mod wordopr;

pub use self::convexity::*;
pub use self::error::*;
pub use self::keyword::*;
pub use self::literal::*;
pub use self::punctuation::*;
pub use self::wordopr::*;

use self::db::*;
use husky_coword::*;
#[cfg(feature = "semantic_token_support")]
use husky_semantic_token_kind::SemanticTokenKind;
use husky_term_prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDataDb)]
#[enum_class::from_variants]
pub enum TokenData {
    Keyword(Keyword),
    Ident(Ident),
    Label(Label),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(LiteralData),
    Error(TokenDataError),
}

#[cfg(feature = "semantic_token_support")]
impl TokenData {
    // overridable given more information
    pub fn default_semantic_token_kind(self) -> SemanticTokenKind {
        match self {
            TokenData::Keyword(kw) => SemanticTokenKind::Keyword(kw.kind()),
            TokenData::Punctuation(_) => SemanticTokenKind::Special,
            TokenData::WordOpr(_) => SemanticTokenKind::WordOpr,
            TokenData::Literal(_) => SemanticTokenKind::Literal,
            TokenData::Ident(_) => SemanticTokenKind::Ident,
            TokenData::Label(_) => SemanticTokenKind::Label,
            TokenData::Error(_) => SemanticTokenKind::Error,
        }
    }
}

impl std::hash::Hash for TokenData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
