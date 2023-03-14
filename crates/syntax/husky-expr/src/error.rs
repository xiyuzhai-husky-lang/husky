use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_opn_syntax::Bracket;
use husky_token::*;
use parsec::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum ExprError {
    #[error("original {0}")]
    Original(#[from] OriginalExprError),
    #[error("derived {0}")]
    Derived(#[from] DerivedExprError),
}

impl From<TokenError> for ExprError {
    fn from(value: TokenError) -> Self {
        ExprError::Derived(value.into())
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum OriginalExprError {
    #[error("mismatching bracket")]
    MismatchingBracket {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect `>`")]
    ExpectRightAngleBracket { langle_token_idx: TokenIdx },
    #[error("expect `}}`")]
    ExpectRightCurlyBrace(TokenIdx),
    #[error("expect identifier")]
    ExpectIdent(TokenIdx),
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
    ExpectIdentAfterDot(TokenIdx),
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
    #[error("expect item before `,`")]
    ExpectItemBeforeComma { comma_token_idx: TokenIdx },
    #[error("expect item before `be`")]
    ExpectItemBeforeBe { be_token_idx: TokenIdx },
    #[error("expect pattern expression after `be`")]
    ExpectedPatternExprAfterBe(TokenIdx),
    #[error("expect variable pattern")]
    ExpectLetVariablePattern(TokenIdx),
    #[error("expect `=`")]
    ExpectAssign(TokenIdx),
    #[error("expect initial value")]
    ExpectInitialValue(TokenIdx),
    #[error("unexpected keyword")]
    UnexpectedKeyword(TokenIdx),
    #[error("expect result")]
    ExpectResult(TokenIdx),
    #[error("expect condition")]
    ExpectCondition(TokenIdx),
    #[error("expect for expr")]
    ExpectForExpr(TokenIdx),
    #[error("expect be pattern")]
    ExpectBePattern(TokenIdx),
    #[error("expect paramter pattern")]
    ExpectParameterPattern(TokenIdx),
    #[error("unterminated list")]
    UnterminatedList { bra_token_idx: TokenIdx },
    #[error("expect `:` at end of line")]
    ExpectEolColon(TokenIdx),
    #[error("expect identifier after `mut`")]
    ExpectIdentAfterMut(TokenIdx),
    #[error("expect block")]
    ExpectBlock(TokenGroupIdx),
    #[error("unexpected `$`")]
    UnexpectedSheba(TokenIdx),
    #[error("unrecognized identifier")]
    UnrecognizedIdent { token_idx: TokenIdx, ident: Ident },
    #[error("unrecognized identifier")]
    UnresolvedSubentity { token_idx: TokenIdx, ident: Ident },
    #[error("expect let variables type")]
    ExpectedLetVariablesType(TokenIdx),
    #[error("expect field type")]
    ExpectedFieldType(TokenIdx),
    #[error("expect parameter type")]
    ExpectParameterType(TokenIdx),
    #[error("SelfTypeNotAllowed")]
    SelfTypeNotAllowed(TokenIdx),
    #[error("SelfValueNotAllowed")]
    SelfValueNotAllowed(TokenIdx),
}

impl OriginalError for OriginalExprError {
    type Error = ExprError;
}

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum DerivedExprError {
    #[error("token error {0}")]
    Token(#[from] TokenError),
}

pub type ExprResult<T> = Result<T, ExprError>;
pub type ExprResultRef<'a, T> = Result<T, &'a ExprError>;

// impl<'a, 'b> FromAbsent<RightCurlyBraceToken, ExprParseContext<'a, 'b>> for OriginalExprError {
//     fn new_absent_error(state: <ExprParseContext<'a, 'b> as HasParseState>::State) -> Self {
//         OriginalExprError::ExpectRightCurlyBrace(state)
//     }
// }

// // impl<'a, Context> FromAbsent<IdentToken, Context> for ExprError
// // where
// //     Context: TokenParseContext<'a>,
// //
// // {
// //     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
// //         ExprError::ExpectIdent(state)
// //     }
// // }

// impl<'a, Context> FromAbsent<ColonToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectColon(state)
//     }
// }

// impl<'a, Context> FromAbsent<RightParenthesisToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectRightParenthesis(state)
//     }
// }

// impl<'a, Context> FromAbsent<LetVariablesPattern, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectLetVariablePattern(state)
//     }
// }

// impl<'a, Context> FromAbsent<AssignToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectAssignToken(state)
//     }
// }

// impl<'a, Context> FromAbsent<BeVariableDeclPattern, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectBePattern(state)
//     }
// }

// impl<'a, Context> FromAbsent<EolColonToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectEolColon(state)
//     }
// }
