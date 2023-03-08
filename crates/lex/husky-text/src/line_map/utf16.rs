#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Utf16CharColRange {
    /// Start offset of a character inside a line, zero-based
    pub(crate) start: u32,
    /// End offset of a character inside a line, zero-based
    pub(crate) end: u32,
}

impl Utf16CharColRange {
    /// Returns the length in 8-bit UTF-8 code units.
    pub(crate) fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Returns the length in 16-bit UTF-16 code units.
    pub(crate) fn len_utf16(&self) -> u32 {
        if self.len() == 4 {
            2
        } else {
            1
        }
    }
}
