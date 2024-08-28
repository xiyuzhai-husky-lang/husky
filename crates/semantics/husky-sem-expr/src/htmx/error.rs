use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemExprHtmxError {
    #[error("{0}")]
    Original(#[from] OriginalSemExprHtmxError),
    #[error("{0}")]
    Derived(#[from] DerivedSemExprHtmxError),
}

pub type SemExprHtmxResult<T> = Result<T, SemExprHtmxError>;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalSemExprHtmxError {}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedSemExprHtmxError {
    #[error("plot class not infered")]
    PlotClassNotInferred,
}
