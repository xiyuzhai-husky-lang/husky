use crate::*;

#[derive(Clone)]
pub(crate) struct TokenIter<'a> {
    pub(crate) db: &'a dyn WordDb,
    pub(crate) input: &'a str,
    chars: CrlfFold<'a>,
}

impl<'a> TokenIter<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, input: &'a str) -> Self {
        let mut t = TokenIter {
            db,
            input,
            chars: CrlfFold::new(input.char_indices()),
        };
        // Eat utf-8 BOM
        t.try_eat_char('\u{feff}');
        t
    }

    pub(crate) fn current(&self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    pub(crate) fn emit_token(
        &self,
        start: usize,
        variant: TomlTokenResult<TomlTokenVariant>,
    ) -> TomlToken {
        TomlToken {
            span: self.calc_span(start),
            variant,
        }
    }

    /// Calculate the span from start to next char
    fn calc_span(&self, start: usize) -> TextSpan {
        let end = self
            .peek_char()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        TextSpan { start, end }
    }
}

impl<'a> TokenIter<'a> {
    pub(crate) fn try_eat_char(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.next_char();
                true
            }
            _ => false,
        }
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
