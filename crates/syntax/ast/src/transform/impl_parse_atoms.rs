use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_atoms<S>(
        &mut self,
        tokens: &[Token],
        f: impl FnOnce(AtomLRParser) -> S,
    ) -> S {
        let symbol_context = self.symbol_context();
        let abs_semantic_tokens_ptr: *const Vec<AbsSemanticToken> = &self.abs_semantic_tokens;
        let abs_semantic_tokens_mut_ptr: *mut Vec<AbsSemanticToken> =
            abs_semantic_tokens_ptr as *mut Vec<AbsSemanticToken>;
        f(AtomLRParser::new(
            &symbol_context,
            Some(unsafe { &mut *abs_semantic_tokens_mut_ptr }),
            tokens,
        ))
    }
}
