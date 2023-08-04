use crate::DeclarativeSignatureDb;
use husky_declarative_term::DeclarativeTermSymbolTypeErrorKind;
use husky_print_utils::p;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub enum DeclarativeTermError2 {
    Original(OriginalDeclarativeTermError2),
    Derived(DerivedDeclarativeTermError2),
}

impl Into<DeclarativeTermSymbolTypeErrorKind> for DeclarativeTermError2 {
    fn into(self) -> DeclarativeTermSymbolTypeErrorKind {
        DeclarativeTermSymbolTypeErrorKind::SignatureDeclarativeTermError
    }
}

impl From<OriginalDeclarativeTermError2> for DeclarativeTermError2 {
    fn from(v: OriginalDeclarativeTermError2) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedDeclarativeTermError2> for DeclarativeTermError2 {
    fn from(v: DerivedDeclarativeTermError2) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalDeclarativeTermError2 {
    ExpectedLiteralForArrayLength,
    InvalidSymbolForTerm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedDeclarativeTermError2 {
    InvalidEntityPath,
    CannotInferFunctionDeclarativeTermInApplication,
    CannotInferArgumentDeclarativeTermInApplication,
    CannotInferOperandDeclarativeTermInPrefix,
    ExprError,
    DeclarativeTermAbortion,
    CannotInferArgumentDeclarativeTermInBoxList,
    CannotInferArrayLength,
    // should have been reported as syntax error
    SelfTypeNotAllowedInThisRegion,
    // should have been reported as syntax error
    SelfValueNotAllowedInThisRegion,
    InheritedSymbolIsNotValidTerm,
}

pub type DeclarativeTermResult2<T> = Result<T, DeclarativeTermError2>;
pub type DeclarativeTermResultBorrowed2<'a, T> = Result<T, &'a DeclarativeTermError2>;
