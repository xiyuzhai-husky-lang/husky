mod query;

pub use query::*;

use text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Highlight {
    kind: HighlightKind,
    range: TextRange,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HighlightKind {}
