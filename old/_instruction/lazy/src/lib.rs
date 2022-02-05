mod error;
mod query;

pub use error::{Origin, SemanticError, SemanticResult, SemanticResultArc};
pub use query::LazyInstructionQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InferenceTable {
    data: Vec<Option<ExprData>>,
    errors: Vec<SemanticError>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprData {}
