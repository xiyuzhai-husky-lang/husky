use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_opn_syntax::Bracket;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExprError {
    #[error("mismatching bracket")]
    MismatchingBracket {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("missing `>`")]
    MissingRightAngleBracket { langle_token_idx: TokenIdx },
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenIdx),
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("expect `:`")]
    ExpectColon(TokenIdx),
    #[error("expect `)`")]
    ExpectRightParenthesis(TokenIdx),
    #[error("no matching bracket")]
    NoMatchingBra {
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect identifier after dot")]
    ExpectIdentifierAfterDot(TokenIdx),
    #[error("token error {0}")]
    Token(#[from] TokenError),
    #[error("no left operand for binary operator")]
    NoLeftOperandForBinaryOperator { binary_token_idx: TokenIdx },
    #[error("no right operand for binary operator")]
    NoRightOperandForBinaryOperator {
        lopd: ExprIdx,
        punctuation: BinaryOpr,
        punctuation_token_idx: TokenIdx,
    },
    #[error("no operand for prefix operator")]
    NoOperandForPrefixOperator {
        prefix: PrefixOpr,
        prefix_token_idx: TokenIdx,
    },
    #[error("missing item before `,`")]
    MissingItemBeforeComma { comma_token_idx: TokenIdx },
    #[error("missing item before `be`")]
    MissingItemBeforeBe { be_token_idx: TokenIdx },
    #[error("expect variable pattern")]
    ExpectLetVariablePattern(TokenIdx),
    #[error("expect `=`")]
    ExpectAssignToken(TokenIdx),
    #[error("missing initial value")]
    MissingInitialValue(TokenIdx),
    #[error("unexpected keyword")]
    UnexpectedKeyword(TokenIdx),
    #[error("missing result")]
    MissingResult(TokenIdx),
    #[error("missing condition")]
    MissingCondition(TokenIdx),
    #[error("expect for expr")]
    MissingForExpr(TokenIdx),
    #[error("expect be pattern")]
    ExpectBePattern(TokenIdx),
    #[error("expect paramter pattern")]
    ExpectParameterPattern(TokenIdx),
    #[error("unterminated list")]
    UnterminatedList { bra_token_idx: TokenIdx },
    #[error("expect `:` at end of line")]
    ExpectEolColon(TokenIdx),
    #[error("expect identifier after `mut`")]
    ExpectIdentifierAfterMut(TokenIdx),
    #[error("missing block")]
    MissingBlock(TokenGroupIdx),
    #[error("unexpected `$`")]
    UnexpectedSheba(TokenIdx),
    #[error("unrecognized identifier")]
    UnrecognizedIdentifier {
        token_idx: TokenIdx,
        ident: Identifier,
    },
    #[error("unrecognized identifier")]
    UnresolvedSubentity {
        token_idx: TokenIdx,
        ident: Identifier,
    },
}

pub type ExprResult<T> = Result<T, ExprError>;

impl<'a, 'b> FromAbsent<RightCurlyBraceToken, ExprParseContext<'a, 'b>> for ExprError {
    fn new_absent_error(state: <ExprParseContext<'a, 'b> as HasParseState>::State) -> Self {
        ExprError::ExpectRightCurlyBrace(state)
    }
}

// impl<'a, Context> FromAbsent<IdentifierToken, Context> for ExprError
// where
//     Context: TokenParseContext<'a>,
//
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         ExprError::ExpectIdentifier(state)
//     }
// }

impl<'a, Context> FromAbsent<ColonToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectColon(state)
    }
}

impl<'a, Context> FromAbsent<RightParenthesisToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectRightParenthesis(state)
    }
}

impl<'a, Context> FromAbsent<LetVariablePattern, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectLetVariablePattern(state)
    }
}

impl<'a, Context> FromAbsent<AssignToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectAssignToken(state)
    }
}

impl<'a, Context> FromAbsent<BePattern, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectBePattern(state)
    }
}

impl<'a, Context> FromAbsent<EolColonToken, Context> for ExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        ExprError::ExpectEolColon(state)
    }
}
