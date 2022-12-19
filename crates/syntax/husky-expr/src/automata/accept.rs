use super::*;

impl<'a, 'b> Automata<'a, 'b> {
    pub(crate) fn next_token(&mut self) -> Option<&'a Token> {
        self.token_iter.next()
    }

    pub(crate) fn accept_token(&mut self, token: ResolvedToken) -> ExprSyntaxResult<()> {
        match token.kind() {
            ResolvedTokenKind::Atom(_atom) => Ok(self.accept_atom(token.to_expr(self.arena))),
            ResolvedTokenKind::BinaryOpr(opr) => self.accept_binary_opr(*opr),
            ResolvedTokenKind::Prefix(opr) => Ok(self.accept_prefix_opr(*opr, token.text_start())),
            ResolvedTokenKind::Suffix(opr) => {
                Ok(self.accept_suffix_opr(/* ad hoc */ opr.clone(), token.text_end()))
            }
        }
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.stack.push_expr(atom)
    }

    pub(crate) fn accept_prefix_opr(&mut self, prefix: PrefixOpr, start: TextPosition) {
        self.stack.push_opr(OnStackOpr::prefix(prefix, start))
    }

    pub(crate) fn accept_suffix_opr(&mut self, suffix: RawSuffixOpr, end: TextPosition) {
        self.synthesize_suffix(suffix, end)
    }

    pub(crate) fn accept_binary_opr(&mut self, binary: BinaryOpr) -> ExprSyntaxResult<()> {
        let stack_opr = OnStackOpr::binary(binary);
        self.synthesize_all_above(stack_opr.precedence())?;
        self.stack.push_opr(stack_opr);
        Ok(())
    }
}
