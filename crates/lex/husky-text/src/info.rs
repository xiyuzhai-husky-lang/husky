use crate::*;

/// Info associated with a text range.
#[derive(Debug)]
pub struct RangeInfo<T> {
    pub range: TextRange,
    pub info: T,
}
