use crate::{
    symbol::{Symbol, SymbolKind},
    *,
};
use text::TextRanged;
use token::*;
use token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_function_defn_head(
        &mut self,
        paradigm: Paradigm,
        token_group: &[Token],
    ) -> AstResult<AstKind> {
        let tokens = trim_colon!(token_group; keyword, colon);
        let head = match paradigm {
            Paradigm::Procedural => {
                self.context.set(AstContext::Stmt(Paradigm::Procedural));
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(Paradigm::Procedural)
                })?
            }
            Paradigm::EagerFunctional => {
                self.context
                    .set(AstContext::Stmt(Paradigm::EagerFunctional));
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(Paradigm::EagerFunctional)
                })?
            }
            Paradigm::LazyFunctional => todo!(),
        };
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstKind::CallFormDefnHead(head))
    }
}
