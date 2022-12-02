use super::*;

impl<'a> TomlTokenIter<'a> {
    pub(crate) fn eat_whitespace_then_next(&mut self) -> Option<TomlToken> {
        while self.try_eat_char(' ') || self.try_eat_char('\t') {
            // ...
        }
        self.next()
    }
}
