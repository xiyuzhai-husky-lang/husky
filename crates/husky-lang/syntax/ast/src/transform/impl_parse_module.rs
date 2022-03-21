use text::TextRanged;
use token::*;
use vm::*;
use word::*;

use crate::{
    atom::symbol_proxy::{Symbol, SymbolKind},
    transform::utils::*,
    *,
};

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_module_item(
        &mut self,
        tokens: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        let keyword = if let TokenKind::Keyword(keyword) = tokens[0].kind {
            keyword
        } else {
            todo!()
        };
        Ok(match keyword {
            Keyword::Routine(routine_keyword) => match routine_keyword {
                RoutineKeyword::Main => {
                    enter_block(self);
                    self.env.set_value(Env::Main);
                    AstKind::MainDecl
                }
                RoutineKeyword::Test => {
                    enter_block(self);
                    self.env.set_value(Env::Test);
                    todo!()
                }
                RoutineKeyword::Proc => {
                    enter_block(self);
                    self.env.set_value(Env::Proc);
                    let head =
                        self.parse_routine_decl(trim!(Some(self.file), tokens; keyword, colon))?;
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
                    enter_block(self);
                    self.env.set_value(Env::Func);
                    let decl =
                        self.parse_routine_decl(trim!(Some(self.file), tokens; keyword, colon))?;
                    for input_placeholder in decl.input_placeholders.iter() {
                        match input_placeholder.contract {
                            EagerContract::Pure | EagerContract::Ref | EagerContract::Take => (),
                            EagerContract::BorrowMut | EagerContract::TakeMut => {
                                todo!("report invalid input contract")
                            }
                            EagerContract::Exec => todo!(),
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
            },
            Keyword::Type(ty_kw) => match ty_kw {
                TypeKeyword::Struct => {
                    enter_block(self);
                    self.env.set_value(Env::Struct);
                    expect_len!(Some(self.file), tokens, 3);
                    expect_head!(Some(self.file), tokens);
                    AstKind::TypeDef {
                        ident: identify!(Some(self.file), tokens[1]),
                        kind: RawTyKind::Struct,
                        generics: Vec::new(),
                    }
                }
                TypeKeyword::Rename => todo!(),
                TypeKeyword::Enum => {
                    enter_block(self);
                    self.env.set_value(Env::Enum);
                    expect_len!(Some(self.file), tokens, 3);
                    expect_head!(Some(self.file), tokens);
                    AstKind::TypeDef {
                        ident: identify!(Some(self.file), tokens[1]),
                        kind: RawTyKind::Enum,
                        generics: Vec::new(),
                    }
                }
                TypeKeyword::Props => todo!(),
            },
            Keyword::Use | Keyword::Mod => todo!(),
            Keyword::Stmt(_) => todo!("no stmt in module level"),
            Keyword::Config(cfg) => match cfg {
                ConfigKeyword::Dataset => {
                    enter_block(self);
                    self.env.set_value(Env::DatasetConfig);
                    self.use_all(
                        BuiltinIdentifier::DatasetType.into(),
                        tokens[0].text_range(),
                    )?;
                    AstKind::DatasetConfig
                }
            },
            Keyword::Def => todo!(),
        })
    }
}
