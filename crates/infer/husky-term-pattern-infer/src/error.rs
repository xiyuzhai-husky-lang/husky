use std::sync::Arc;

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
                DerivedTermPatternInferError::TermPatternInferError(ref error) => Some(error),
            },
        }
    }

    fn is_original(&self) -> bool {
        match self.source {
            TermPatternInferErrorSource::Original(_) => true,
            TermPatternInferErrorSource::Derived(_) => false,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[error("`{0}")]
pub struct TermPatternInferError(Arc<TermPatternInferErrorInternal>);

// impl TermPatternInferError {
//     pub(crate) fn derive(&self, ctx: &TermPatternInferContext) -> Self {
//         Self::new(
//             TermPatternInferErrorSource::Derived(
//                 DerivedTermPatternInferError::TermPatternInferError(self.clone()),
//             ),
//             ctx.range(),
//         )
//     }
// }

impl TermPatternInferError {
    fn new(source: TermPatternInferErrorSource, range: TextRange) -> Self {
        TermPatternInferError(Arc::new(TermPatternInferErrorInternal { source, range }))
    }
}

impl std::ops::Deref for TermPatternInferError {
    type Target = TermPatternInferErrorInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("`{source}` at {range}")]
pub struct TermPatternInferErrorInternal {
    source: TermPatternInferErrorSource,
    range: TextRange,
}

pub type TermPatternInferResult<T> = Result<T, TermPatternInferError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalTermPatternInferError {
    #[error("ident `{ident}` not recognized")]
    IdentUnrecognized { ident: String },
    #[error("`{0}`")]
    TermError(#[from] TermError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedTermPatternInferError {
    #[error("derived from `{0}`")]
    TermPatternInferError(#[from] TermPatternInferError),
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn map_original<T, E>(&self, result: Result<T, E>) -> TermPatternInferResult<T>
    where
        E: Into<OriginalTermPatternInferError>,
    {
        result.map_err(|error| self.error_original(error))
    }

    pub(crate) fn err_original<T, E>(&self, error: E) -> TermPatternInferResult<T>
    where
        E: Into<OriginalTermPatternInferError>,
    {
        Err(self.error_original(error))
    }

    pub(crate) fn error_original<E>(&self, error: E) -> TermPatternInferError
    where
        E: Into<OriginalTermPatternInferError>,
    {
        TermPatternInferError::new(
            TermPatternInferErrorSource::Original(error.into()),
            self.range(),
        )
    }

    pub(crate) fn err_derived<T, E>(&self, error: E) -> TermPatternInferResult<T>
    where
        E: Into<DerivedTermPatternInferError>,
    {
        Err(TermPatternInferError::new(
            TermPatternInferErrorSource::Derived(error.into()),
            self.range(),
        ))
    }

    pub(crate) fn range(&self) -> TextRange {
        todo!()
        // self.expr().range
    }
}
