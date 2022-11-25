#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Utf16Char {
    /// Start offset of a character inside a line, zero-based
    pub(crate) start: usize,
    /// End offset of a character inside a line, zero-based
    pub(crate) end: usize,
}

impl Utf16Char {
    /// Returns the length in 8-bit UTF-8 code units.
    pub(crate) fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the length in 16-bit UTF-16 code units.
    pub(crate) fn len_utf16(&self) -> usize {
        if self.len() == 4 {
            2
        } else {
            1
        }
    }
}
