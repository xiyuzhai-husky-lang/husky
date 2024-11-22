use maybe_result::MaybeResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvError {
    InvalidValue(String),
}

pub type LpCsvResult<T> = Result<T, LpCsvError>;
pub type LpCsvMaybeResult<T> = MaybeResult<T, LpCsvError>;
