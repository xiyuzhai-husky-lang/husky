use crate::*;
use error_lineage::ErrorLineage;
use husky_term::TermError;
use husky_text::TextRange;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TermPatternInferErrorSource {
    #[error("`{0}`")]
    Original(#[from] OriginalTermPatternInferError),
    #[error("`{0}`")]
    Derived(#[from] DerivedTermPatternInferError),
}

impl ErrorLineage for TermPatternInferError {
    fn opt_parent(&self) -> Option<&dyn ErrorLineage> {
        match self.source {
            TermPatternInferErrorSource::Original(_) => None,
            TermPatternInferErrorSource::Derived(ref error) => match error {
                DerivedTermPatternInferError::Haha => todo!(),
            },
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("`{source}` at {range}")]
pub struct TermPatternInferError {
    source: TermPatternInferErrorSource,
    range: TextRange,
}

pub type TermPatternInferResult<T> = Result<T, TermPatternInferError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalTermPatternInferError {
    #[error("ident `{ident}` not recognized")]
    IdentUnrecognized { ident: Identifier },
    #[error("`{0}`")]
    TermError(#[from] TermError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedTermPatternInferError {
    #[error("haha")]
    Haha,
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn map_original<T, E>(&self, result: Result<T, E>) -> TermPatternInferResult<T>
    where
        E: Into<OriginalTermPatternInferError>,
    {
        result.map_err(|e| TermPatternInferError {
            source: todo!(),
            range: self.range(),
        })
    }
    pub(crate) fn err_original<T, E>(&self, error: E) -> TermPatternInferResult<T>
    where
        E: Into<OriginalTermPatternInferError>,
    {
        Err(TermPatternInferError {
            source: TermPatternInferErrorSource::Original(error.into()),
            range: self.range(),
        })
    }

    fn range(&self) -> TextRange {
        todo!()
    }
}
