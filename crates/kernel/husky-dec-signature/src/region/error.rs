use husky_dec_term::term::DecSymbolicVariableTypeErrorKind;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum DecTermError2 {
    Original(OriginalDecTermError2),
    Derived(DerivedDecTermError2),
}

impl Into<DecSymbolicVariableTypeErrorKind> for DecTermError2 {
    fn into(self) -> DecSymbolicVariableTypeErrorKind {
        DecSymbolicVariableTypeErrorKind::SignatureDecTermError
    }
}

impl Into<DecSymbolicVariableTypeErrorKind> for DerivedDecTermError2 {
    fn into(self) -> DecSymbolicVariableTypeErrorKind {
        DecSymbolicVariableTypeErrorKind::SignatureDecTermError
    }
}

impl From<OriginalDecTermError2> for DecTermError2 {
    fn from(v: OriginalDecTermError2) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedDecTermError2> for DecTermError2 {
    fn from(v: DerivedDecTermError2) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalDecTermError2 {
    ExpectedLiteralForArrayLength,
    InvalidSymbolForTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DerivedDecTermError2 {
    InvalidEntityPath,
    CannotInferFunctionDecTermInApplication,
    CannotInferArgumentDecTermInApplication,
    CannotInferOperandDecTermInPrefix,
    ExprError,
    DecTermAbortion,
    CannotInferArgumentDecTermInBoxList,
    CannotInferArrayLength,
    // should have been reported as syntax error
    SelfTypeNotAllowedInThisRegion,
    // should have been reported as syntax error
    SelfValueNotAllowedInThisRegion,
    InheritedSynSymbolIsNotValidTerm,
}

pub type DecTermResult2<T> = Result<T, DecTermError2>;
pub type DerivedDecTermResult2<T> = Result<T, DerivedDecTermError2>;
pub type DecTermResultBorrowed2<'a, T> = Result<T, &'a DecTermError2>;
