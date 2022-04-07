use super::utils::*;
use crate::{
    atom::parser::AtomLRParser,
    symbol_proxy::{Symbol, SymbolKind},
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
        let tokens = trim_colon!(Some(self.file), token_group; keyword, colon);
        let head = match routine_keyword {
            RoutineKeyword::Test => {
                self.env.set_value(AstContext::Test);
                todo!()
            }
            RoutineKeyword::Proc => {
                self.env.set_value(AstContext::Proc);
                AtomLRParser::new(Some(self.file), self.symbol_proxy(), tokens)
                    .routine_defn_head(RoutineKind::Proc)?
            }
            RoutineKeyword::Func => {
                self.env.set_value(AstContext::Func);
                AtomLRParser::new(Some(self.file), self.symbol_proxy(), tokens)
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
