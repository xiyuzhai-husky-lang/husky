use super::*;

impl<'a> Automata<'a> {
    pub(crate) fn next_token(&mut self) -> Option<&'a Token> {
        self.stream.next()
    }

    pub(crate) fn accept_token(&mut self, token: ResolvedToken) {
        todo!()
    }
}
