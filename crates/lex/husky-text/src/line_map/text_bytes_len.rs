/// Primitives with a textual length that can be passed to [`usize::of`].
pub trait TextBytesLen: Copy {
    /// The textual length of this primitive.
    fn text_bytes_len(self) -> u32;
}

impl TextBytesLen for &'_ str {
    #[inline]
    fn text_bytes_len(self) -> u32 {
        self.len().try_into().unwrap()
    }
}

impl TextBytesLen for &'_ String {
    #[inline]
    fn text_bytes_len(self) -> u32 {
        self.as_str().text_bytes_len()
    }
}

impl TextBytesLen for char {
    #[inline]
    fn text_bytes_len(self) -> u32 {
        self.len_utf8() as u32
    }
}
