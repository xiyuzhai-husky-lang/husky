mod query;

pub use query::*;

use serde::{Deserialize, Serialize};
use text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Highlight {
    kind: HighlightKind,
    range: TextRange,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HighlightKind {
    Comment,
    Keyword,
    String,
    Number,
    Operator,
    Namespace,
    Type,
    Function,
    Method,
    Field,
    Variable,
}
