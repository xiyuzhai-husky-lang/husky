use crate::*;

pub struct TextCharIter<'a> {
    pub(super) iter: core::slice::Iter<'a, u8>,
    pub(super) front_offset: usize,
    next_pos: TextPosition,
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
                    self.front_offset += 2;
                    self.next_pos = self.next_pos.to_next_line();
                    Some('\n')
                } else {
                    let len = self.iter.len();
                    self.front_offset += pre_len - len;
                    self.next_pos = self.next_pos.to_right(1);
                    Some(ch)
                }
            }
            '\n' => {
                self.front_offset += 1;
                self.next_pos = self.next_pos.to_next_line();
                Some(ch)
            }
            _ => {
                let len = self.iter.len();
                self.front_offset += pre_len - len;
                self.next_pos = self.next_pos.to_right(1);
                Some(ch)
            }
        }
    }
}

impl<'a> TextCharIter<'a> {
    pub(crate) fn new(input: &'a str, front_offset: usize, start_pos: TextPosition) -> Self {
        Self {
            iter: input.as_bytes().iter(),
            front_offset,
            next_pos: start_pos,
        }
    }

    pub fn new_from_start(input: &'a str) -> Self {
        Self::new(input, Default::default(), Default::default())
    }

    fn next_char_raw(&mut self) -> Option<char> {
        unsafe { core::str::next_code_point(&mut self.iter).map(|ch| char::from_u32_unchecked(ch)) }
    }

    pub fn next_position(&self) -> TextPosition {
        self.next_pos
    }

    pub fn peek(&self) -> Option<char> {
        unsafe {
            core::str::next_code_point(&mut self.iter.clone())
                .map(|ch| char::from_u32_unchecked(ch))
        }
    }
}

pub struct PositionedTextCharIter<'a> {
    iter: TextCharIter<'a>,
}

impl<'a> PositionedTextCharIter<'a> {
    pub(crate) fn new(input: &'a str, front_offset: usize, start_pos: TextPosition) -> Self {
        Self {
            iter: TextCharIter {
                iter: input.as_bytes().iter(),
                front_offset,
                next_pos: start_pos,
            },
        }
    }
}

impl<'a> Iterator for PositionedTextCharIter<'a> {
    type Item = (TextPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.iter.next_position();
        let ch = self.iter.next()?;
        Some((pos, ch))
    }
}

impl Text {
    pub fn char_iter<'a>(&'a self) -> TextCharIter<'a> {
        TextCharIter::new(&self.content, 0, Default::default())
    }

    pub fn positioned_char_iter<'a>(&'a self) -> PositionedTextCharIter<'a> {
        PositionedTextCharIter::new(&self.content, 0, Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_iter_works() {
        fn t(text: &str, expect: &[char]) {
            let text = Text::new(text);
            let chars: Vec<_> = text.char_iter().collect();
            assert_eq!(chars, expect);
        }

        t("a\n\r\n", &['a', '\n', '\n']);
    }

    #[test]
    fn ranged_char_iter_works() {
        fn t<S>(text: &str, expect: &[(S, char)])
        where
            TextPosition: for<'a> From<&'a S>,
        {
            let text = Text::new(text);
            let chars: Vec<_> = text.positioned_char_iter().collect();
            let expect: Vec<(TextPosition, char)> =
                expect.iter().map(|(s, ch)| (s.into(), *ch)).collect();
            assert_eq!(chars, expect);
        }

        t("a\n\r\n", &[((0, 0), 'a'), ((0, 1), '\n'), ((1, 0), '\n')]);
    }
}
