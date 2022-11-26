use crate::*;

impl<'a> TokenIter<'a> {
    pub(crate) fn next_hex(&mut self, start: usize, i: usize, len: usize) -> TomlTokenResult<char> {
        let mut buf = String::with_capacity(len);
        for _ in 0..len {
            match self.next_char() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_ascii_hexdigit() => buf.push(ch),
                Some((i, ch)) => return Err(TomlTokenError::InvalidHexEscape(i, ch)),
                None => return Err(TomlTokenError::UnterminatedString(start)),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(TomlTokenError::InvalidEscapeValue(i, val)),
        }
    }
}
