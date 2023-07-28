use crate::*;
use husky_entity_syn_tree::EntityTreeError;
use husky_opr::Bracket;
use husky_token::*;
use original_error::IntoError;
use parsec::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
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
#[salsa::debug_with_db(db = SynExprDb)]
// #[salsa::derive_debug_with_db(db = ExprDb)]
pub enum OriginalExprError {
    #[error("expected `>`")]
    ExpectedRightAngleBracket { langle_token_idx: TokenIdx },
    #[error("expected `}}`")]
    ExpectedRightCurlyBrace(TokenStreamState),
    #[error("expected identifier")]
    ExpectedIdent(TokenStreamState),
    #[error("expected `:`")]
    ExpectedColon(TokenStreamState),
    #[error("expected `)`")]
    ExpectedRightParenthesis(TokenStreamState),
    #[error("no matching bracket")]
    NoMatchingBra {
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expected item before `,`")]
    ExpectedItemBeforeComma { comma_token_idx: TokenIdx },
    #[error("expected item before `be`")]
    ExpectedItemBeforeBe { be_token_idx: TokenIdx },
    #[error("expected variable pattern")]
    ExpectedLetVariableDecls(TokenStreamState),
    #[error("expected pattern expression after `be`")]
    ExpectedBeVariablesPattern(TokenStreamState),
    #[error("expected `=`")]
    ExpectedAssign(TokenStreamState),
    #[error("expected initial value")]
    ExpectedInitialValue(TokenStreamState),
    #[error("unexpected keyword")]
    UnexpectedKeyword(TokenIdx),
    #[error("expected result")]
    ExpectedResult(TokenStreamState),
    #[error("expected condition")]
    ExpectedCondition(TokenStreamState),
    #[error("expected for expr")]
    ExpectedForExpr(TokenIdx),
    #[error("expected be pattern")]
    ExpectedBePattern(TokenIdx),
    #[error("expected paramter pattern")]
    ExpectedParameterPattern(TokenIdx),
    #[error("ExpectedValueForFieldBindInitialization")]
    ExpectedValueForFieldBindInitialization(TokenStreamState),
    #[error("ExpectedFunctionIdentAfterOpeningHtmlBra")]
    ExpectedFunctionIdentAfterOpeningHtmlBra(TokenStreamState),
    #[error("expected identifier after `mut`")]
    ExpectedIdentAfterModifier(TokenStreamState),
    #[error("expected `:` at end of line")]
    ExpectedEolColon(TokenStreamState),
    #[error("expected constant implicit parameter type")]
    ExpectedConstantImplicitParameterType(TokenStreamState),
    #[error("mismatching bracket")]
    MismatchingBracket {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        ket: Bracket,
        ket_token_idx: TokenIdx,
    },
    #[error("expected let variables type")]
    ExpectedLetVariablesType(TokenStreamState),
    #[error("expected field type")]
    ExpectedFieldType(TokenStreamState),
    #[error("expected parameter type")]
    ExpectedParameterType(TokenStreamState),
    #[error("ExpectedTraits")]
    ExpectedTraits(TokenStreamState),
    #[error("ExpectedKeyedWithDefaultParameterInitialValue")]
    ExpectedExplicitParameterDefaultValue(TokenStreamState),
    #[error("expected identifier after `.`")]
    ExpectedIdentAfterDot { dot_token_idx: TokenIdx },
    #[error("expected exprBeforeDot")]
    ExpectedExprBeforeDot { dot_token_idx: TokenIdx },
    #[error("expect block")]
    ExpectedBlock(TokenGroupIdx),
    #[error("unterminated list")]
    UnterminatedList { bra_token_idx: TokenIdx },
    #[error("unterminated list")]
    UnterminatedFunctionCallKeyedArgumentList { bra_token_idx: TokenIdx },
    #[error("unterminated list")]
    UnterminatedMethodCallKeyedArgumentList { bra_token_idx: TokenIdx },
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
    UnresolvedSubitem { token_idx: TokenIdx, ident: Ident },
    #[error("SelfTypeNotAllowed")]
    SelfTypeNotAllowed(TokenIdx),
    #[error("SelfValueNotAllowed")]
    SelfValueNotAllowed(TokenIdx),
    #[error("HtmlTodo")]
    HtmlTodo(TokenStreamState),
    #[error("UnexpectedLeftCurlyBrace")]
    UnexpectedLeftCurlyBrace(TokenIdx),
    #[error("ExpectedTypeAfterLightArrow")]
    ExpectedTypeAfterLightArrow { light_arrow_token: LightArrowToken },
    #[error("ExpectedTypeTermForAssociatedType")]
    ExpectedTypeTermForAssociatedType(TokenStreamState),
}

impl OriginalExprError {
    pub fn token_idx_range(&self) -> TokenIdxRange {
        match self {
            OriginalExprError::ExpectedLetVariableDecls(token_idx)
            | OriginalExprError::ExpectedBeVariablesPattern(token_idx) => todo!(),
            OriginalExprError::ExpectedLetVariablesType(token_stream_state)
            | OriginalExprError::ExpectedAssign(token_stream_state)
            | OriginalExprError::ExpectedInitialValue(token_stream_state)
            | OriginalExprError::ExpectedResult(token_stream_state)
            | OriginalExprError::ExpectedCondition(token_stream_state)
            | OriginalExprError::ExpectedRightCurlyBrace(token_stream_state)
            | OriginalExprError::ExpectedIdent(token_stream_state)
            | OriginalExprError::ExpectedColon(token_stream_state)
            | OriginalExprError::ExpectedRightParenthesis(token_stream_state)
            | OriginalExprError::ExpectedEolColon(token_stream_state)
            | OriginalExprError::ExpectedIdentAfterModifier(token_stream_state)
            | OriginalExprError::ExpectedFieldType(token_stream_state)
            | OriginalExprError::ExpectedParameterType(token_stream_state)
            | OriginalExprError::HtmlTodo(token_stream_state)
            | OriginalExprError::ExpectedValueForFieldBindInitialization(token_stream_state)
            | OriginalExprError::ExpectedFunctionIdentAfterOpeningHtmlBra(token_stream_state)
            | OriginalExprError::ExpectedConstantImplicitParameterType(token_stream_state)
            | OriginalExprError::ExpectedTraits(token_stream_state)
            | OriginalExprError::ExpectedExplicitParameterDefaultValue(token_stream_state) => {
                let token_idx = token_stream_state.next_token_idx();
                match token_stream_state.drained() {
                    true => TokenIdxRange::new_drained(token_idx),
                    false => TokenIdxRange::new_single(token_idx),
                }
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
            | OriginalExprError::UnterminatedFunctionCallKeyedArgumentList {
                bra_token_idx: token_idx,
            }
            | OriginalExprError::UnterminatedMethodCallKeyedArgumentList {
                bra_token_idx: token_idx,
            }
            | OriginalExprError::UnexpectedSheba(token_idx)
            | OriginalExprError::UnrecognizedIdent { token_idx, .. }
            | OriginalExprError::UnresolvedSubitem { token_idx, .. }
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
            OriginalExprError::ExpectedTypeAfterLightArrow { light_arrow_token } => todo!(),
            OriginalExprError::ExpectedTypeTermForAssociatedType(_) => todo!(),
        }
    }
}

impl IntoError for OriginalExprError {
    type Error = ExprError;
}

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum DerivedExprError {
    #[error("token error {0}")]
    Token(#[from] TokenError),
}

pub type SynExprResult<T> = Result<T, ExprError>;
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

// impl<'a, Context> FromAbsent<LetVariableDecls, Context> for OriginalExprError
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
