use super::utils::*;
use crate::{
    symbol::{Symbol, SymbolKind},
    *,
};
use text::TextRanged;
use token::*;
use word::RoutineKeyword;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_routine_decl(
        &mut self,
        routine_keyword: RoutineKeyword,
        token_group: &[Token],
    ) -> AstResult<AstKind> {
        let tokens = trim_colon!(token_group; keyword, colon);
        let head = match routine_keyword {
            RoutineKeyword::Test => {
                self.env.set_value(AstContext::Test);
                todo!()
            }
            RoutineKeyword::Proc => {
                self.env.set_value(AstContext::Proc);
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(RoutineContextKind::Proc)
                })?
            }
            RoutineKeyword::Func => {
                self.env.set_value(AstContext::Func);
                self.parse_atoms(tokens, |parser| {
                    parser.routine_defn_head(RoutineContextKind::Func)
                })?
            }
        };
        self.symbols.extend(
            head.input_placeholders
                .iter()
                .map(|input_placeholder| Symbol {
                    ident: input_placeholder.ident.ident,
                    kind: SymbolKind::Variable {
                        init_row: input_placeholder.ranged_ty.row(),
                    },
                }),
        );
        Ok(AstKind::RoutineDefnHead(head))
    }
}
