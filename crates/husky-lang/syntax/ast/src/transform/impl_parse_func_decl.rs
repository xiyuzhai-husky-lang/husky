use crate::{atom::parser::AtomLRParser, *};
use token::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_func_decl(&mut self, tokens: &[Token]) -> AstResult<FuncDecl> {
        AtomLRParser::new(self.symbol_proxy(), tokens).func_decl()
    }
}
