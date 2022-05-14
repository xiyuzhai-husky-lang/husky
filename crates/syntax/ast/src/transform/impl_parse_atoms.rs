use wild_utils::ref_to_mut_ref;

use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_atoms<S>(
        &mut self,
        tokens: &[Token],
        f: impl FnOnce(AtomParser) -> S,
    ) -> S {
        let symbol_context = self.symbol_context();
        f(AtomParser::new(
            &symbol_context,
            Some(unsafe { ref_to_mut_ref(&self.abs_semantic_tokens) }),
            tokens,
        ))
    }
}
