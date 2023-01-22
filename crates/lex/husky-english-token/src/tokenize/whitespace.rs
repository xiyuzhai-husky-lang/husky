use super::*;

impl<'a> EnglishTokenIter<'a> {
    pub(crate) fn eat_whitespace_then_next(&mut self) -> Option<EnglishToken> {
        while self.try_eat_char(' ') || self.try_eat_char('\t') {
            // ...
        }
        self.next()
    }
}
