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
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub enum DerivedSemaExprDataError {
    #[error("syn expr")]
    SynExpr,
    #[error("ApplicationOrRitchieCallFunctionTypeNotInferred")]
    ApplicationOrRitchieCallFunctionTypeNotInferred { function_sema_expr_idx: SemaExprIdx },
}

impl From<&SynExprError> for SemaExprDataError {
    fn from(value: &SynExprError) -> Self {
        DerivedSemaExprDataError::SynExpr.into()
    }
}

pub type SemaExprDataResult<T> = Result<T, SemaExprDataError>;
pub type SemaExprDataResultRef<'a, T> = Result<T, &'a SemaExprDataError>;
