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
use husky_term_prelude::*;
#[cfg(feature = "protocol_support")]
use husky_token_protocol::TokenKindProtocol;

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

#[cfg(feature = "protocol_support")]
impl TokenData {
    // overridable given more information
    pub fn default_protocol(self) -> TokenKindProtocol {
        match self {
            TokenData::Keyword(kw) => TokenKindProtocol::Keyword(kw.protocol()),
            TokenData::Punctuation(_) => TokenKindProtocol::Special,
            TokenData::WordOpr(_) => TokenKindProtocol::WordOpr,
            TokenData::Literal(_) => TokenKindProtocol::Literal,
            TokenData::Ident(_) => TokenKindProtocol::Ident,
            TokenData::Label(_) => TokenKindProtocol::Label,
            TokenData::Error(_) => TokenKindProtocol::Error,
        }
    }
}

impl std::hash::Hash for TokenData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
