use wild_utils::ref_to_mut_ref;

use super::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_atoms<S>(
        &mut self,
        tokens: &[HuskyToken],
        f: impl FnOnce(AtomParser) -> S,
    ) -> S {
        f(AtomParser::new(self, &mut tokens.into()))
    }
}
