use crate::*;

impl<'a> TomlTokenIter<'a> {
    pub(crate) fn next_comment_token(&mut self) -> TomlTokenVariant {
        while let Some(ch) = self.peek_char() {
            if ch != '\t' && !('\u{20}'..='\u{10ffff}').contains(&ch) {
                break;
            }
            self.next_char();
        }
        TomlTokenVariant::Comment
    }
}
