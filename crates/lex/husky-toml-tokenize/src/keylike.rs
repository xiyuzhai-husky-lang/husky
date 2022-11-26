use crate::*;

impl<'a> Tokenizer<'a> {
    pub(crate) fn next_keylike(&mut self, start: usize) -> TomlTokenVariant {
        while let Some((_, ch)) = self.peek_char() {
            if !is_keylike(ch) {
                break;
            }
            self.next_char();
        }
        TomlTokenVariant::Keylike(self.db.it_word_borrowed(&self.input[start..self.current()]))
    }
}

pub(crate) fn is_keylike(ch: char) -> bool {
    ('A'..='Z').contains(&ch)
        || ('a'..='z').contains(&ch)
        || ('0'..='9').contains(&ch)
        || ch == '-'
        || ch == '_'
}
