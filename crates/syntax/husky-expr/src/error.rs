use crate::*;
use husky_entity_tree::EntityTreeError;
use husky_opn_syntax::Bracket;
use husky_token::*;
use original_error::OriginalError;
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
// #[salsa::derive_debug_with_db(db = ExprDb)]
pub enum OriginalExprError {
    #[error("expect `>`")]
    ExpectedRightAngleBracket { langle_token_idx: TokenIdx },
    #[error("expect `}}`")]
    ExpectedRightCurlyBrace(TokenStreamState),
    #[error("expect identifier")]
    ExpectedIdent(TokenStreamState),
    #[error("expect `:`")]
    ExpectedColon(TokenStreamState),
    #[error("expect `)`")]
    ExpectedRightParenthesis(TokenStreamState),
    #[error("no matching bracket")]
    NoMatchingBra {
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect item before `,`")]
    ExpectedItemBeforeComma { comma_token_idx: TokenIdx },
    #[error("expect item before `be`")]
    ExpectedItemBeforeBe { be_token_idx: TokenIdx },
    #[error("expect variable pattern")]
    ExpectedLetVariablesPattern(TokenStreamState),
    #[error("expect pattern expression after `be`")]
    ExpectedBeVariablesPattern(TokenStreamState),
    #[error("expect `=`")]
    ExpectedAssign(TokenStreamState),
    #[error("expect initial value")]
    ExpectedInitialValue(TokenStreamState),
    #[error("unexpected keyword")]
    UnexpectedKeyword(TokenIdx),
    #[error("expect result")]
    ExpectedResult(TokenStreamState),
    #[error("expect condition")]
    ExpectedCondition(TokenStreamState),
    #[error("expect for expr")]
    ExpectedForExpr(TokenIdx),
    #[error("expect be pattern")]
    ExpectedBePattern(TokenIdx),
    #[error("expect paramter pattern")]
    ExpectedParameterPattern(TokenIdx),
    #[error("ExpectedValueForFieldBindInitialization")]
    ExpectedValueForFieldBindInitialization(TokenStreamState),
    #[error("ExpectedFunctionIdentAfterOpeningHtmlBra")]
    ExpectedFunctionIdentAfterOpeningHtmlBra(TokenStreamState),
    #[error("expect identifier after `mut`")]
    ExpectedIdentAfterModifier(TokenStreamState),
    #[error("expect `:` at end of line")]
    ExpectedEolColon(TokenStreamState),
    #[error("mismatching bracket")]
    MismatchingBracket {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expect let variables type")]
    ExpectedLetVariablesType(TokenStreamState),
    #[error("expect field type")]
    ExpectedFieldType(TokenStreamState),
    #[error("expect parameter type")]
    ExpectedParameterType(TokenStreamState),
    #[error("ExpectedIdentAfterDot")]
    ExpectedIdentAfterDot { dot_token_idx: TokenIdx },
    #[error("ExpectedExprBeforeDot")]
    ExpectedExprBeforeDot { dot_token_idx: TokenIdx },
    #[error("expect block")]
    ExpectedBlock(TokenGroupIdx),
    #[error("unterminated list")]
    UnterminatedList { bra_token_idx: TokenIdx },
    #[error("no left operand for binary operator")]
    NoLeftOperandForBinaryOperator { binary_token_idx: TokenIdx },
    #[error("no right operand for binary operator")]
    NoRightOperandForBinaryOperator {
        punctuation: BinaryOpr,
        punctuation_token_idx: TokenIdx,
    },
    #[error("no operand for prefix operator")]
    NoOperandForPrefixOperator {
        prefix: PrefixOpr,
        prefix_token_idx: TokenIdx,
    },
    #[error("unexpected `$`")]
    UnexpectedSheba(TokenIdx),
    #[error("unrecognized identifier")]
    UnrecognizedIdent { token_idx: TokenIdx, ident: Ident },
    #[error("unrecognized identifier")]
    UnresolvedSubentity { token_idx: TokenIdx, ident: Ident },
    #[error("SelfTypeNotAllowed")]
    SelfTypeNotAllowed(TokenIdx),
    #[error("SelfValueNotAllowed")]
    SelfValueNotAllowed(TokenIdx),
    #[error("HtmlTodo")]
    HtmlTodo(TokenStreamState),
    #[error("UnexpectedLeftCurlyBrace")]
    UnexpectedLeftCurlyBrace(TokenIdx),
}

impl OriginalExprError {
    pub fn token_idx_range(&self) -> TokenIdxRange {
        match self {
            OriginalExprError::ExpectedLetVariablesPattern(token_idx)
            | OriginalExprError::ExpectedBeVariablesPattern(token_idx) => todo!(),
            OriginalExprError::ExpectedLetVariablesType(token_idx)
            | OriginalExprError::ExpectedAssign(token_idx)
            | OriginalExprError::ExpectedInitialValue(token_idx)
            | OriginalExprError::ExpectedResult(token_idx)
            | OriginalExprError::ExpectedCondition(token_idx)
            | OriginalExprError::ExpectedRightCurlyBrace(token_idx)
            | OriginalExprError::ExpectedIdent(token_idx)
            | OriginalExprError::ExpectedColon(token_idx)
            | OriginalExprError::ExpectedRightParenthesis(token_idx)
            | OriginalExprError::ExpectedEolColon(token_idx)
            | OriginalExprError::ExpectedIdentAfterModifier(token_idx)
            | OriginalExprError::ExpectedFieldType(token_idx)
            | OriginalExprError::ExpectedParameterType(token_idx)
            | OriginalExprError::HtmlTodo(token_idx)
            | OriginalExprError::ExpectedValueForFieldBindInitialization(token_idx)
            | OriginalExprError::ExpectedFunctionIdentAfterOpeningHtmlBra(token_idx) => {
                todo!()
            }
            OriginalExprError::MismatchingBracket {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectedRightAngleBracket {
                langle_token_idx: token_idx,
            }
            | OriginalExprError::NoMatchingBra {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::NoLeftOperandForBinaryOperator {
                binary_token_idx: token_idx,
            }
            | OriginalExprError::NoRightOperandForBinaryOperator {
                punctuation_token_idx: token_idx,
                ..
            }
            | OriginalExprError::NoOperandForPrefixOperator {
                prefix_token_idx: token_idx,
                ..
            }
            | OriginalExprError::UnexpectedKeyword(token_idx)
            | OriginalExprError::ExpectedItemBeforeComma {
                comma_token_idx: token_idx,
            }
            | OriginalExprError::ExpectedItemBeforeBe {
                be_token_idx: token_idx,
            }
            | OriginalExprError::ExpectedForExpr(token_idx)
            | OriginalExprError::ExpectedBePattern(token_idx)
            | OriginalExprError::ExpectedParameterPattern(token_idx)
            | OriginalExprError::UnterminatedList {
                bra_token_idx: token_idx,
            }
            | OriginalExprError::UnexpectedSheba(token_idx)
            | OriginalExprError::UnrecognizedIdent { token_idx, .. }
            | OriginalExprError::UnresolvedSubentity { token_idx, .. }
            | OriginalExprError::SelfTypeNotAllowed(token_idx)
            | OriginalExprError::SelfValueNotAllowed(token_idx)
            | OriginalExprError::ExpectedIdentAfterDot {
                dot_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectedExprBeforeDot {
                dot_token_idx: token_idx,
            }
            | OriginalExprError::UnexpectedLeftCurlyBrace(token_idx) => {
                TokenIdxRange::new_single(*token_idx)
            }
            OriginalExprError::ExpectedBlock(_) => todo!(),
        }
    }
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
