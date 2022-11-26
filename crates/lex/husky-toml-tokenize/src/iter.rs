use crate::{crlf_fold::CrlfFold, *};

#[derive(Clone)]
pub(crate) struct TokenIter<'a> {
    pub(crate) db: &'a dyn WordDb,
    pub(crate) input: &'a str,
    chars: CrlfFold<'a>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = TomlToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, c) = self.next_char()?;

        let variant = match c {
            '\n' => Ok(TomlTokenVariant::Newline),
            ' ' => Ok(self.next_whitespace_token(start)),
            '\t' => Ok(self.next_whitespace_token(start)),
            '#' => Ok(self.next_comment_token(start)),
            '=' => Ok(TomlSpecialToken::Equals.into()),
            '.' => Ok(TomlSpecialToken::Period.into()),
            ',' => Ok(TomlSpecialToken::Comma.into()),
            ':' => Ok(TomlSpecialToken::Colon.into()),
            '+' => Ok(TomlSpecialToken::Plus.into()),
            '{' => Ok(TomlSpecialToken::LeftCurly.into()),
            '}' => Ok(TomlSpecialToken::RightCurly.into()),
            '[' => Ok(TomlSpecialToken::LeftBox.into()),
            ']' => Ok(TomlSpecialToken::RightBox.into()),
            '\'' => {
                return Some(TomlToken {
                    span: self.calc_span(start),
                    variant: self.next_literal_string(start),
                })
            }
            '"' => {
                return Some(TomlToken {
                    span: self.calc_span(start),
                    variant: self.next_basic_string(start),
                })
            }
            ch if is_keylike(ch) => Ok(self.next_keylike(start)),
            ch => Err(TomlTokenError::Unexpected(start, ch)),
        };

        let span = self.calc_span(start);
        Some(TomlToken { span, variant })
    }
}

impl<'a> TokenIter<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, input: &'a str) -> Self {
        let mut t = TokenIter {
            db,
            input,
            chars: CrlfFold::new(input.char_indices()),
        };
        // Eat utf-8 BOM
        t.try_eat_one_char('\u{feff}');
        t
    }

    pub(crate) fn try_eat_one_char(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.next_char();
                true
            }
            _ => false,
        }
    }

    pub(crate) fn current(&self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    fn next_whitespace_token(&mut self, start: usize) -> TomlTokenVariant {
        while self.try_eat_one_char(' ') || self.try_eat_one_char('\t') {
            // ...
        }
        TomlTokenVariant::Whitespace
    }

    fn next_comment_token(&mut self, start: usize) -> TomlTokenVariant {
        while let Some((_, ch)) = self.chars.clone().next() {
            if ch != '\t' && !('\u{20}'..='\u{10ffff}').contains(&ch) {
                break;
            }
            self.next_char();
        }
        TomlTokenVariant::Comment
    }

    pub(crate) fn next_hex(
        &mut self,
        start: usize,
        i: usize,
        len: usize,
    ) -> TomlTokenizeResult<char> {
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

    /// Calculate the span from start to next char
    fn calc_span(&mut self, start: usize) -> TextSpan {
        let end = self
            .peek_char()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        TextSpan { start, end }
    }

    /// Peek one char without consuming it.
    pub(crate) fn peek_char(&self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    /// Take one char.
    pub(crate) fn next_char(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
}
