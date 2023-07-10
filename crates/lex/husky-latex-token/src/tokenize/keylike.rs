use super::*;

impl<'a> EnglishTokenIter<'a> {
    pub(crate) fn next_keylike(&mut self, start: usize) -> EnglishTokenVariant {
        while let Some(ch) = self.peek_char() {
            if !is_keylike(ch) {
                break;
            }
            self.next_char();
        }
        EnglishTokenVariant::Word(
            self.db
                .it_coword_borrowed(&self.input[start..self.current()]),
        )
    }
}

pub(crate) fn is_keylike(ch: char) -> bool {
    ('A'..='Z').contains(&ch)
        || ('a'..='z').contains(&ch)
        || ('0'..='9').contains(&ch)
        || ch == '-'
        || ch == '_'
}
