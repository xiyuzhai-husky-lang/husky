use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum SemaExprDataError {
    #[error("original {0}")]
    Original(#[from] OriginalSemaExprDataError),
    #[error("derived {0}")]
    Derived(#[from] DerivedSemaExprDataError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum OriginalSemaExprDataError {
    #[error("RitchieParameterArgumentMismatch")]
    RitchieParameterArgumentMismatch {
        match_error: RitchieParameterArgumentMatchError,
        ritchie_arguments: Vec<SemaExprIdx>,
    },
    #[error("no such field")]
    NoSuchField {
        owner_ty: FluffyTerm,
        ident_token: IdentRegionalToken,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
// #[salsa::debug_with_db(db = SemaExprDb)]
pub enum DerivedSemaExprDataError {
    #[error("syn expr")]
    SynExpr,
    #[error("ApplicationOrRitchieCallFunctionTypeNotInferred")]
    ApplicationOrRitchieCallFunctionTypeNotInferred { function_sema_expr_idx: SemaExprIdx },
    #[error("MethodOwnerTypeNotInferred")]
    MethodOwnerTypeNotInferred {
        list_item_sema_expr_idxs: Vec<SemaExprIdx>,
    },
    #[error("FieldOwnerTypeNotInferred")]
    FieldOwnerTypeNotInferred { owner_sema_expr_idx: SemaExprIdx },
}

impl From<&SynExprError> for SemaExprDataError {
    fn from(value: &SynExprError) -> Self {
        DerivedSemaExprDataError::SynExpr.into()
    }
}

impl From<FluffyTermError> for SemaExprDataError {
    fn from(value: FluffyTermError) -> Self {
        todo!()
    }
}

impl OriginalError for OriginalSemaExprDataError {
    type Error = SemaExprDataError;
}

pub type SemaExprDataResult<T> = Result<T, SemaExprDataError>;
pub type SemaExprDataResultRef<'a, T> = Result<T, &'a SemaExprDataError>;
