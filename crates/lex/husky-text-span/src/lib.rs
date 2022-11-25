/// A span, designating a range of bytes where a token is located.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct TextSpan {
    /// The start of the range.
    pub start: usize,
    /// The end of the range (exclusive).
    pub end: usize,
}

impl From<TextSpan> for (usize, usize) {
    fn from(TextSpan { start, end }: TextSpan) -> (usize, usize) {
        (start, end)
    }
}
