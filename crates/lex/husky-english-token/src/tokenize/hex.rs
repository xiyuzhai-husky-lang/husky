use super::*;

impl<'a> EnglishTokenIter<'a> {
    pub(crate) fn next_hex(&mut self, i: usize, len: usize) -> EnglishTokenResult<char> {
        let mut buf = String::with_capacity(len);
        for _ in 0..len {
            match self.next_char_with_offset() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_ascii_hexdigit() => buf.push(ch),
                Some((i, ch)) => return Err(EnglishTokenError::InvalidHexEscape(i, ch)),
                None => return Err(EnglishTokenError::UnterminatedString),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(EnglishTokenError::InvalidEscapeValue(i, val)),
        }
    }
}
