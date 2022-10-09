use super::*;

impl<'a> Automata<'a> {
    pub(crate) fn next_token(&mut self) -> Option<&Token> {
        self.stream.next()
    }

    pub(crate) fn accept_atom(&mut self) {
        todo!()
    }
}
