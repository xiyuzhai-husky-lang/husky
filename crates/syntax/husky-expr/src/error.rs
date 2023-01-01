use crate::*;
use husky_opn_syntax::Bracket;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ExprError {
    #[error("non matching bracket")]
    MisMatchingBracket {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenIdx),
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("expect `:`")]
    ExpectColon(TokenIdx),
    #[error("no matching bracket")]
    NoMatchingBra {
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect identifier after dot")]
    ExpectIdentifierAfterDot,
    #[error("token error {0}")]
    Token(#[from] TokenError),
    #[error("no left operand for binary operator")]
    NoLeftOperandForBinaryOperator,
    #[error("no right operand for binary operator")]
    NoRightOperandForBinaryOperator {
        lopd: ExprIdx,
        punctuation: BinaryPunctuation,
        punctuation_token_idx: TokenIdx,
    },
    #[error("no operand for prefix operator")]
    NoOperandForPrefixOperator {
        prefix: PrefixPunctuation,
        prefix_token_idx: TokenIdx,
    },
    #[error("missing item before comma")]
    MissingItemBeforeComma { comma_token_idx: TokenIdx },
}

impl<'a, 'b, 'c> FromAbsent<RightCurlyBraceToken, ExprParser<'a, 'b, 'c>> for ExprError {
    fn new_absent_error(state: <ExprParser<'a, 'b, 'c> as HasParseState>::State) -> Self {
        ExprError::ExpectRightCurlyBrace(state)
    }
}

impl<'a, Context> FromAbsent<IdentifierToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectIdentifier(state)
    }
}

impl<'a, Context> FromAbsent<ColonToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectColon(state)
    }
}
