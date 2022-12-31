use husky_opn_syntax::Bracket;
use husky_token::{TokenError, TokenIdx};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ExprError {
    #[error("non matching bracket")]
    MisMatchingBracket {
        bra: Bracket,
        bra_token: TokenIdx,
        ket: Bracket,
        ket_token: TokenIdx,
    },
    #[error("no matching bracket")]
    NoMatchingBra { ket: Bracket, ket_token: TokenIdx },
    #[error("expect identifier after dot")]
    ExpectIdentifierAfterDot,
    #[error("token error {0}")]
    Token(#[from] TokenError),
}

pub type ExprResult<T> = Result<T, ExprError>;
