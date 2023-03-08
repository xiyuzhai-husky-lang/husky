/// A span, designating a range of bytes where a token is located.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct DocumentSpan {
    /// The start of the range.
    pub start: usize,
    /// The end of the range (exclusive).
    pub end: usize,
}

impl From<DocumentSpan> for (usize, usize) {
    fn from(DocumentSpan { start, end }: DocumentSpan) -> (usize, usize) {
        (start, end)
    }
}
