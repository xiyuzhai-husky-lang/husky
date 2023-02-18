use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_opn_syntax::Bracket;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExprError {
    #[error("original {0}")]
    Original(#[from] OriginalExprError),
    #[error("derived {0}")]
    Derived(#[from] DerivedExprError),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum OriginalExprError {
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
    #[error("missing let variables type")]
    MissingLetVariablesType(TokenIdx),
    #[error("missing field type")]
    MissingFieldType(TokenIdx),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DerivedExprError {
    #[error("token error")]
    Token,
}

pub type ExprResult<T> = Result<T, OriginalExprError>;

impl<'a, 'b> FromAbsent<RightCurlyBraceToken, ExprParseContext<'a, 'b>> for OriginalExprError {
    fn new_absent_error(state: <ExprParseContext<'a, 'b> as HasParseState>::State) -> Self {
        OriginalExprError::ExpectRightCurlyBrace(state)
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

impl<'a, Context> FromAbsent<ColonToken, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectColon(state)
    }
}

impl<'a, Context> FromAbsent<RightParenthesisToken, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectRightParenthesis(state)
    }
}

impl<'a, Context> FromAbsent<LetVariablesPattern, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectLetVariablePattern(state)
    }
}

impl<'a, Context> FromAbsent<AssignToken, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectAssignToken(state)
    }
}

impl<'a, Context> FromAbsent<BeVariableDeclPattern, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectBePattern(state)
    }
}

impl<'a, Context> FromAbsent<EolColonToken, Context> for OriginalExprError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        OriginalExprError::ExpectEolColon(state)
    }
}
