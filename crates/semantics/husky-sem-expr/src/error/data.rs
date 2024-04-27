use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemExprDataError {
    #[error("original {0}")]
    Original(#[from] OriginalSemExprDataError),
    #[error("derived {0}")]
    Derived(#[from] DerivedSemExprDataError),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSemExprDataError {
    #[error("RitchieParameterArgumentMismatch")]
    RitchieParameterArgumentMismatch {
        match_error: RitchieArgumentError,
        ritchie_arguments: Vec<SemExprIdx>,
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
pub enum DerivedSemExprDataError {
    #[error("syn expr")]
    SynExpr,
    #[error("ApplicationOrRitchieCallFunctionTypeNotInferred")]
    ApplicationOrRitchieCallFunctionTypeNotInferred { function_sem_expr_idx: SemExprIdx },
    #[error("MethodOwnerTypeNotInferred")]
    MethodOwnerTypeNotInferred {
        self_argument_sem_expr_idx: SemExprIdx,
        list_item_sem_expr_idxs: Vec<SemExprIdx>,
    },
    #[error("FieldOwnerTypeNotInferred")]
    FieldOwnerTypeNotInferred { owner_sem_expr_idx: SemExprIdx },
    #[error("UnableToInferIndexExprType")]
    UnableToInferIndexExprType,
    #[error("UnveilOutputTemplate")]
    UnveilOutputTemplate {
        opd_sem_expr_idx: SemExprIdx,
        e: EtherealSignatureError,
    },
    #[error("SynPatternError")]
    SynPatternError,
    #[error("ExplicitApplicationFunctionTypeNotInferred")]
    ExplicitApplicationFunctionTypeNotInferred,
}

impl From<&SynExprError> for SemExprDataError {
    fn from(value: &SynExprError) -> Self {
        DerivedSemExprDataError::SynExpr.into()
    }
}

impl From<FlyTermError> for SemExprDataError {
    fn from(value: FlyTermError) -> Self {
        todo!()
    }
}

impl OriginalError for OriginalSemExprDataError {
    type Error = SemExprDataError;
}

pub type SemExprDataResult<T> = Result<T, SemExprDataError>;
pub type SemExprDataResultRef<'a, T> = Result<T, &'a SemExprDataError>;
