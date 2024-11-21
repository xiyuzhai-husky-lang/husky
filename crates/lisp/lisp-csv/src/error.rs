#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvError {
    InvalidValue(String),
}

pub type LpCsvResult<T> = Result<T, LpCsvError>;
