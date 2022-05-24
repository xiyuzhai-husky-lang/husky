use crate::{
    symbol::{Symbol, SymbolKind},
    *,
};
use text::TextRanged;
use token::*;
use token::*;
use word::RoutineKeyword;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_routine_defn_head(
        &mut self,
        routine_keyword: RoutineKeyword,
        token_group: &[Token],
    ) -> AstResult<AstKind> {
        let tokens = trim_colon!(token_group; keyword, colon);
        let head = match routine_keyword {
            RoutineKeyword::Proc => {
                self.context.set(AstContext::Routine(RoutineKeyword::Proc));
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(RoutineKeyword::Proc)
                })?
            }
            RoutineKeyword::Func => {
                self.context.set(AstContext::Routine(RoutineKeyword::Func));
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(RoutineKeyword::Func)
                })?
            }
        };
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstKind::RoutineDefnHead(head))
    }
}
