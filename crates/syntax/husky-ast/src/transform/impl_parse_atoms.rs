use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_atoms<S>(
        &mut self,
        tokens: &[Token],
        f: impl FnOnce(AtomParser) -> S,
    ) -> S {
        f(AtomParser::new(self, &mut tokens.into()))
    }
}
