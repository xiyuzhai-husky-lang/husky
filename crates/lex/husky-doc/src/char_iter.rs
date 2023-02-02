use crate::*;

#[derive(Clone)]
pub struct TextCharIter<'a> {
    pub(super) iter: core::slice::Iter<'a, u8>,
    pub(super) current_offset: usize,
    current_position: TextPosition,
}

impl<'a> Iterator for TextCharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let pre_len = self.iter.len();
        let ch = self.next_char_raw()?;
        // CrlfFold
        match ch {
            '\r' => {
                let mut attempt = self.iter.clone();
                if Some('\n')
                    == unsafe {
                        core::str::next_code_point(&mut attempt)
                            .map(|ch| char::from_u32_unchecked(ch))
                    }
                {
                    self.iter = attempt;
                    self.current_offset += 2;
                    self.current_position = self.current_position.to_next_line();
                    Some('\n')
                } else {
                    let len = self.iter.len();
                    self.current_offset += pre_len - len;
                    self.current_position = self.current_position.to_right(1);
                    Some(ch)
                }
            }
            '\n' => {
                self.current_offset += 1;
                self.current_position = self.current_position.to_next_line();
                Some(ch)
            }
            _ => {
                let len = self.iter.len();
                self.current_offset += pre_len - len;
                self.current_position = self.current_position.to_right(1);
                Some(ch)
            }
        }
    }
}

impl<'a> TextCharIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self::new_aux(input, Default::default(), Default::default())
    }

    pub(crate) fn new_aux(input: &'a str, next_offset: usize, start_pos: TextPosition) -> Self {
        Self {
            iter: input.as_bytes().iter(),
            current_offset: next_offset,
            current_position: start_pos,
        }
    }

    fn eat_char(&mut self) {
        self.next().expect("what");
    }

    fn eat_chars_with(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.eat_char();
            } else {
                break;
            }
        }
    }

    pub fn get_str_slice_with(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
        let slice = self.iter.as_slice();
        let start = self.current_offset;
        self.eat_chars_with(predicate);
        let end = self.current_offset;
        unsafe { std::str::from_utf8_unchecked(&slice[..(end - start)]) }
    }

    fn next_char_raw(&mut self) -> Option<char> {
        unsafe { core::str::next_code_point(&mut self.iter).map(|ch| char::from_u32_unchecked(ch)) }
    }

    pub fn current_position(&self) -> TextPosition {
        self.current_position
    }

    pub fn current_offset(&self) -> usize {
        self.current_offset
    }

    pub fn next_char_with_offset(&mut self) -> Option<(usize, char)> {
        let offset = self.current_offset;
        let ch = self.next()?;
        Some((offset, ch))
    }

    pub fn next_char_with_offset_and_position(&mut self) -> Option<(usize, TextPosition, char)> {
        let offset = self.current_offset;
        let position = self.current_position;
        let ch = self.next()?;
        Some((offset, position, ch))
    }

    pub fn peek(&self) -> Option<char> {
        self.clone().next()
    }
}

#[derive(Clone)]
pub struct OffsetedTextCharIter<'a> {
    iter: TextCharIter<'a>,
}

impl<'a> OffsetedTextCharIter<'a> {
    pub(crate) fn new_aux(input: &'a str, front_offset: usize, start_pos: TextPosition) -> Self {
        Self {
            iter: TextCharIter {
                iter: input.as_bytes().iter(),
                current_offset: front_offset,
                current_position: start_pos,
            },
        }
    }

    pub fn new(input: &'a str) -> Self {
        Self::new_aux(input, Default::default(), Default::default())
    }
}

impl<'a> Iterator for OffsetedTextCharIter<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.iter.current_offset();
        Some((offset, self.iter.next()?))
    }
}

pub struct PositionedTextCharIter<'a> {
    iter: TextCharIter<'a>,
}

impl<'a> PositionedTextCharIter<'a> {
    pub(crate) fn new_aux(input: &'a str, front_offset: usize, start_pos: TextPosition) -> Self {
        Self {
            iter: TextCharIter {
                iter: input.as_bytes().iter(),
                current_offset: front_offset,
                current_position: start_pos,
            },
        }
    }

    pub fn new(input: &'a str) -> Self {
        Self::new_aux(input, Default::default(), Default::default())
    }
}

impl<'a> Iterator for PositionedTextCharIter<'a> {
    type Item = (TextPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.iter.current_position();
        let ch = self.iter.next()?;
        Some((pos, ch))
    }
}

impl Document {
    pub fn char_iter<'a>(&'a self) -> TextCharIter<'a> {
        TextCharIter::new_aux(&self.content, 0, Default::default())
    }

    pub fn positioned_char_iter<'a>(&'a self) -> PositionedTextCharIter<'a> {
        PositionedTextCharIter::new_aux(&self.content, 0, Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_iter_works() {
        fn t(text: &str, expect: &[char]) {
            let text = Document::new(text);
            let chars: Vec<_> = text.char_iter().collect();
            assert_eq!(chars, expect);
        }

        t("a\n\r\n", &['a', '\n', '\n']);
    }

    #[test]
    fn get_str_slice_with_works() {
        fn t(text: &str, n: usize, predicate: impl Fn(char) -> bool, expect: &str) {
            let text = Document::new(text);
            let mut char_iter = text.char_iter();
            for _ in 0..n {
                char_iter.next();
            }
            assert_eq!(char_iter.get_str_slice_with(predicate), expect);
        }

        t("a\n\r\n", 0, |_| true, "a\n\r\n");
        t("aa\n\r\n", 1, |_| true, "a\n\r\n");
        t("i32 sdf", 0, |c| c.is_alphanumeric(), "i32");
        t("2.0f32+sdf", 3, |c| c.is_alphanumeric(), "f32");
    }

    #[test]
    fn ranged_char_iter_works() {
        fn t<S>(text: &str, expect: &[(S, char)])
        where
            TextPosition: for<'a> From<&'a S>,
        {
            let text = Document::new(text);
            let chars: Vec<_> = text.positioned_char_iter().collect();
            let expect: Vec<(TextPosition, char)> =
                expect.iter().map(|(s, ch)| (s.into(), *ch)).collect();
            assert_eq!(chars, expect);
        }

        t("a\n\r\n", &[((0, 0), 'a'), ((0, 1), '\n'), ((1, 0), '\n')]);
    }

    #[test]
    fn test_char_iter_peek() {
        fn t(sample_text: &str) {
            let mut iter = TextCharIter::new(sample_text);
            while let Some(ch) = iter.peek() {
                assert_eq!(Some(ch), iter.next())
            }
            assert_eq!(iter.next(), None)
        }

        t("\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"")
    }
    #[test]
    fn test_crlf_fold() {
        fn t(sample_text: &str, expect: &[(usize, char)]) {
            let fold = OffsetedTextCharIter::new(sample_text);
            assert_eq!(fold.collect::<Vec<_>>(), expect)
        }

        t(
            "\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"",
            &[
                (0, '"'),
                (1, '"'),
                (2, '"'),
                (3, '\\'),
                (4, '\n'),
                (5, ' '),
                (6, ' '),
                (7, ' '),
                (8, ' '),
                (9, ' '),
                (10, '\t'),
                (11, ' '),
                (12, ' '),
                (13, ' '),
                (14, '\t'),
                (15, ' '),
                (16, ' '),
                (17, '\\'),
                (18, '\n'),
                (20, ' '),
                (21, ' '),
                (22, '\t'),
                (23, ' '),
                (24, '\n'),
                (25, ' '),
                (26, ' '),
                (27, '\t'),
                (28, ' '),
                (29, '\n'),
                (31, '"'),
                (32, '"'),
                (33, '"'),
            ],
        );
    }
}
