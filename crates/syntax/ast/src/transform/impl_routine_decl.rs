use super::utils::*;
use crate::atom::symbol_proxy::{Symbol, SymbolKind};
use crate::{atom::parser::AtomLRParser, *};
use text::TextRanged;
use token::*;
use vm::InputContract;
use word::RoutineKeyword;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_routine_decl(
        &mut self,
        routine_keyword: RoutineKeyword,
        token_group: &[Token],
    ) -> AstResult<AstKind> {
        let tokens = trim_colon!(Some(self.file), token_group; keyword, colon);
        Ok({
            match routine_keyword {
                RoutineKeyword::Test => {
                    self.env.set_value(Env::Test);
                    todo!()
                }
                RoutineKeyword::Proc => {
                    self.env.set_value(Env::Proc);
                    let head = AtomLRParser::new(Some(self.file), self.symbol_proxy(), tokens)
                        .routine_decl()?;
                    self.symbols
                        .extend(
                            head.input_placeholders
                                .iter()
                                .map(|input_placeholder| Symbol {
                                    ident: input_placeholder.ident,
                                    kind: SymbolKind::Variable {
                                        init_row: input_placeholder.ranged_ty.row(),
                                    },
                                }),
                        );
                    AstKind::RoutineDecl {
                        routine_kind: RoutineKind::Proc,
                        routine_head: head,
                    }
                }
                RoutineKeyword::Func => {
                    self.env.set_value(Env::Func);
                    let decl = AtomLRParser::new(Some(self.file), self.symbol_proxy(), tokens)
                        .routine_decl()?;
                    for input_placeholder in decl.input_placeholders.iter() {
                        match input_placeholder.contract {
                            InputContract::Pure
                            | InputContract::GlobalRef
                            | InputContract::Take => (),
                            InputContract::BorrowMut | InputContract::TakeMut => {
                                todo!("report invalid input contract")
                            }
                            InputContract::Exec => todo!(),
                        }
                        self.symbols.push(Symbol {
                            ident: input_placeholder.ident,
                            kind: SymbolKind::Variable {
                                init_row: input_placeholder.ranged_ty.row(),
                            },
                        })
                    }
                    AstKind::RoutineDecl {
                        routine_kind: RoutineKind::Func,
                        routine_head: decl,
                    }
                }
            }
        })
    }
}
