use husky_text::{TextCharIter, TextPosition};

use crate::*;

#[derive(Clone)]
pub(crate) struct TokenIter<'a> {
    pub(crate) db: &'a dyn WordDb,
    pub(crate) input: &'a str,
    chars: TextCharIter<'a>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = TomlToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (start_offset, start_position, c) = self.chars.next_char_with_offset_and_position()?;

        let variant = match c {
            '\n' => Ok(TomlTokenVariant::Newline),
            ' ' => Ok(self.next_whitespace_token()),
            '\t' => Ok(self.next_whitespace_token()),
            '#' => Ok(self.next_comment_token()),
            '=' => Ok(TomlSpecialToken::Equals.into()),
            '.' => Ok(TomlSpecialToken::Period.into()),
            ',' => Ok(TomlSpecialToken::Comma.into()),
            ':' => Ok(TomlSpecialToken::Colon.into()),
            '+' => Ok(TomlSpecialToken::Plus.into()),
            '{' => Ok(TomlSpecialToken::LeftCurly.into()),
            '}' => Ok(TomlSpecialToken::RightCurly.into()),
            '[' => Ok(TomlSpecialToken::LeftBox.into()),
            ']' => Ok(TomlSpecialToken::RightBox.into()),
            '\'' => self.next_literal_string(),
            '"' => self.next_basic_string(),
            ch if is_keylike(ch) => Ok(self.next_keylike(start_offset)),
            ch => Err(TomlTokenError::UnexpectedChar(ch)),
        };

        Some(self.emit_token(start_offset, start_position, variant))
    }
}

impl<'a> TokenIter<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, input: &'a str) -> Self {
        let mut t = TokenIter {
            db,
            input,
            chars: TextCharIter::new(input),
        };
        // Eat utf-8 BOM
        t.try_eat_char('\u{feff}');
        t
    }

    pub(crate) fn current(&self) -> usize {
        self.chars.current_offset()
    }

    pub(crate) fn emit_token(
        &self,
        start_offset: usize,
        start_position: TextPosition,
        variant: TomlTokenResult<TomlTokenVariant>,
    ) -> TomlToken {
        TomlToken::new(
            TextSpan {
                start: start_offset,
                end: self.chars.current_offset(),
            },
            (start_position..self.chars.current_position()).into(),
            variant,
        )
    }
}

impl<'a> TokenIter<'a> {
    pub(crate) fn try_eat_char(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some(ch2) if ch == ch2 => {
                self.next_char();
                true
            }
            _ => false,
        }
    }

    /// Peek one char without consuming it.
    pub(crate) fn peek_char(&self) -> Option<char> {
        self.chars.peek()
    }

    /// Take one char.
    pub(crate) fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn next_char_with_offset(&mut self) -> Option<(usize, char)> {
        self.chars.next_char_with_offset()
    }
}

#[derive(Clone)]
pub(crate) struct CrlfFold<'a> {
    chars: std::str::CharIndices<'a>,
}

impl<'a> CrlfFold<'a> {
    fn new(chars: std::str::CharIndices<'a>) -> Self {
        Self { chars }
    }
}

impl<'a> Iterator for CrlfFold<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(i, c)| {
            if c == '\r' {
                let mut attempt = self.chars.clone();
                if let Some((_, '\n')) = attempt.next() {
                    self.chars = attempt;
                    return (i, '\n');
                }
            }
            (i, c)
        })
    }
}
