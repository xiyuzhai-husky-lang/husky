use num_traits::unsigned_int::error::UnsignedIntError;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum CombinatoricsError {
    #[error("unsigned int error: {0}")]
    UnsignedInt(#[from] UnsignedIntError),
    #[error("multinomial expansion variable count exceeds limit")]
    MultinomialExpansionVariableCountExceedsLimit,
    #[error("multinomial expansion number of terms exceeds limit")]
    MultinomialExpansionNumberOfTermsExceedsLimit,
}

pub type CombinatoricsResult<T> = Result<T, CombinatoricsError>;
