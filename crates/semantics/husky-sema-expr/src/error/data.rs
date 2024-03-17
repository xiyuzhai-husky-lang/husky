use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemaExprDataError {
    #[error("original {0}")]
    Original(#[from] OriginalSemaExprDataError),
    #[error("derived {0}")]
    Derived(#[from] DerivedSemaExprDataError),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSemaExprDataError {
    #[error("RitchieParameterArgumentMismatch")]
    RitchieParameterArgumentMismatch {
        match_error: RitchieArgumentError,
        ritchie_arguments: Vec<SemaExprIdx>,
    },
    #[error("no such field")]
    NoSuchField {
        owner_ty: FlyTerm,
        ident_token: IdentRegionalToken,
    },
    #[error("no such method")]
    NoSuchMethod {
        self_expr_ty: FlyTerm,
        ident_token: IdentRegionalToken,
    },
    #[error("expected indices")]
    ExpectedIndices,
    #[error("CannotIndexIntoType")]
    CannotIndexIntoType { self_expr_ty: FlyTerm },
}

#[derive(Debug, Error, PartialEq, Eq)]
// #[salsa::derive_debug_with_db]
pub enum DerivedSemaExprDataError {
    #[error("syn expr")]
    SynExpr,
    #[error("ApplicationOrRitchieCallFunctionTypeNotInferred")]
    ApplicationOrRitchieCallFunctionTypeNotInferred { function_sema_expr_idx: SemaExprIdx },
    #[error("MethodOwnerTypeNotInferred")]
    MethodOwnerTypeNotInferred {
        self_argument_sema_expr_idx: SemaExprIdx,
        list_item_sema_expr_idxs: Vec<SemaExprIdx>,
    },
    #[error("FieldOwnerTypeNotInferred")]
    FieldOwnerTypeNotInferred { owner_sema_expr_idx: SemaExprIdx },
    #[error("UnableToInferIndexExprType")]
    UnableToInferIndexExprType,
    #[error("UnveilOutputTemplate")]
    UnveilOutputTemplate {
        opd_sema_expr_idx: SemaExprIdx,
        e: EtherealSignatureError,
    },
    #[error("SynPatternError")]
    SynPatternError,
    #[error("ExplicitApplicationFunctionTypeNotInferred")]
    ExplicitApplicationFunctionTypeNotInferred,
}

impl From<&SynExprError> for SemaExprDataError {
    fn from(value: &SynExprError) -> Self {
        DerivedSemaExprDataError::SynExpr.into()
    }
}

impl From<FlyTermError> for SemaExprDataError {
    fn from(value: FlyTermError) -> Self {
        todo!()
    }
}

impl OriginalError for OriginalSemaExprDataError {
    type Error = SemaExprDataError;
}

pub type SemaExprDataResult<T> = Result<T, SemaExprDataError>;
pub type SemaExprDataResultRef<'a, T> = Result<T, &'a SemaExprDataError>;
