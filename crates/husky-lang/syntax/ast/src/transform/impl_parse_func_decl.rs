use crate::{atom::parser::AtomLRParser, *};
use token::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_routine_decl(&mut self, tokens: &[Token]) -> AstResult<RoutineHead> {
        AtomLRParser::new(Some(self.file), self.symbol_proxy(), tokens).func_decl()
    }
}
