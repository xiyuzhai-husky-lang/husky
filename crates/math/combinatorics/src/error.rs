use num_try::int::error::IntError;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum CombinatoricsError {
    #[error("unsigned int error: {0}")]
    UnsignedInt(#[from] IntError),
    #[error("multinomial expansion number of summands before expansion exceeds limit")]
    MultinomialExpansionNumberOfSummandsBeforeExpansionExceedsLimit,
    #[error("multinomial expansion number of summands after expansion exceeds limit")]
    MultinomialExpansionNumberOfSummandsAfterExpansionExceedsLimit,
}

pub type CombinatoricsResult<T> = Result<T, CombinatoricsError>;
