use super::*;

impl<'a> Automata<'a> {
    pub(crate) fn next_token(&mut self) -> Option<&'a Token> {
        self.stream.next()
    }

    pub(crate) fn accept_token(&mut self, token: ResolvedToken) {
        match token.kind() {
            ResolvedTokenKind::Atom(atom) => self.accept_atom(token.to_expr()),
        }
    }

    fn accept_atom(&mut self, atom: RawExpr) {
        self.stack.push_expr(atom)
    }
}
