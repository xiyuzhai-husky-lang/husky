use super::utils::*;
use crate::{
    symbol_proxy::{Symbol, SymbolKind},
    *,
};
use atom::parser::AtomLRParser;
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
                AtomLRParser::new(self.symbol_proxy(), tokens)
                    .routine_defn_head(RoutineKind::Proc)?
            }
            RoutineKeyword::Func => {
                self.env.set_value(AstContext::Func);
                AtomLRParser::new(self.symbol_proxy(), tokens)
                    .routine_defn_head(RoutineKind::Func)?
            }
        };
        self.symbols.extend(
            head.input_placeholders
                .iter()
                .map(|input_placeholder| Symbol {
                    ident: input_placeholder.ident,
                    kind: SymbolKind::Variable {
                        init_row: input_placeholder.ranged_ty.row(),
                    },
                }),
        );
        Ok(AstKind::RoutineDefnHead(head))
    }
}
