use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_atoms<S>(
        &mut self,
        tokens: &[Token],
        f: impl FnOnce(AtomLRParser) -> S,
    ) -> S {
        let symbol_context = self.symbol_context();
        let semantic_tokens_ptr: *const Vec<AbsSemanticToken> = &self.semantic_tokens;
        let semantic_tokens_mut_ptr: *mut Vec<AbsSemanticToken> =
            semantic_tokens_ptr as *mut Vec<AbsSemanticToken>;
        f(AtomLRParser::new(
            &symbol_context,
            Some(unsafe { &mut *semantic_tokens_mut_ptr }),
            tokens,
        ))
    }
}
