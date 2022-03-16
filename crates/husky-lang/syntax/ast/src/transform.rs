mod impl_parse_expr;
mod impl_parse_func_decl;
mod impl_parse_stmt;
mod impl_symbol_proxy;
mod impl_use_all;
mod utils;

use file::FilePtr;
use fold::{FoldedList, LocalStack, LocalValue};
use scope::ScopeRoute;
use syntax_types::*;
use text::TextRanged;
use token::*;
use vm::Contract;
use word::{RoutineKeyword, *};

use crate::{
    atom::symbol_proxy::{Symbol, SymbolKind},
    query::{AstSalsaQueryGroup, AstText},
    transform::utils::*,
    *,
};

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FilePtr,
    arena: RawExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<syntax_types::Env>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstSalsaQueryGroup, module: ScopePtr) -> Self {
        return Self {
            db,
            main: db
                .main_file_id(db.module_to_file_id(module).unwrap())
                .unwrap(),
            arena: RawExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: module_symbols(db, module),
            env: LocalValue::new(match module.route {
                ScopeRoute::Package { .. } => Env::Package,
                ScopeRoute::ChildScope { .. } => Env::Module(module),
                ScopeRoute::Builtin { .. } | ScopeRoute::Implicit { .. } => panic!(),
            }),
        };

        fn module_symbols(db: &dyn AstSalsaQueryGroup, module: ScopePtr) -> LocalStack<Symbol> {
            let mut symbols = LocalStack::new();
            for scope in db.subscopes(module).iter() {
                match scope.route {
                    ScopeRoute::Builtin { .. }
                    | ScopeRoute::Package { .. }
                    | ScopeRoute::Implicit { .. } => panic!(),
                    ScopeRoute::ChildScope { ident, .. } => symbols.push(Symbol {
                        ident,
                        kind: SymbolKind::Scope(scope.route),
                    }),
                }
            }
            symbols
        }
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            arena: self.arena,
            folded_results: self.folded_results,
        }
    }

    fn env(&self) -> Env {
        self.env.value()
    }
}

impl<'a> fold::Transformer<[Token], TokenizedText, AstResult<Ast>> for AstTransformer<'a> {
    fn _enter_block(&mut self) {
        self.env.enter();
        self.symbols.enter();
    }

    fn _exit_block(&mut self) {
        self.env.exit();
        self.symbols.exit();
    }

    fn transform(
        &mut self,
        _indent: fold::Indent,
        tokens: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        Ok(Ast {
            range: tokens.into(),
            kind: if let TokenKind::Keyword(keyword) = tokens[0].kind {
                match keyword {
                    Keyword::Routine(routine_keyword) => match routine_keyword {
                        RoutineKeyword::Main => {
                            enter_block(self);
                            self.env.set_value(Env::Main);
                            AstKind::MainDef
                        }
                        RoutineKeyword::Test => {
                            enter_block(self);
                            self.env.set_value(Env::Test);
                            todo!()
                        }
                        RoutineKeyword::Proc => {
                            enter_block(self);
                            self.env.set_value(Env::Proc);
                            let head = self.parse_routine_decl(trim!(tokens; keyword, colon))?;
                            self.symbols.extend(head.input_placeholders.iter().map(
                                |input_placeholder| Symbol {
                                    ident: input_placeholder.ident,
                                    kind: SymbolKind::Variable(input_placeholder.ranged_ty.range),
                                },
                            ));
                            AstKind::RoutineDef {
                                routine_kind: RoutineKind::Proc,
                                routine_head: head,
                            }
                        }
                        RoutineKeyword::Func => {
                            enter_block(self);
                            self.env.set_value(Env::Func);
                            let decl = self.parse_routine_decl(trim!(tokens; keyword, colon))?;
                            for input_placeholder in decl.input_placeholders.iter() {
                                match input_placeholder.contract {
                                    Contract::PureInput | Contract::Share | Contract::Take => (),
                                    Contract::BorrowMut | Contract::TakeMut => {
                                        todo!("report invalid input contract")
                                    }
                                }
                                self.symbols.push(Symbol {
                                    ident: input_placeholder.ident,
                                    kind: SymbolKind::Variable(input_placeholder.ranged_ty.range),
                                })
                            }
                            AstKind::RoutineDef {
                                routine_kind: RoutineKind::Func,
                                routine_head: decl,
                            }
                        }
                    },
                    Keyword::Type(ty_kw) => match ty_kw {
                        word::TypeKeyword::Struct => {
                            expect_len!(tokens, 3);
                            expect_head!(tokens);
                            AstKind::TypeDef {
                                ident: identify!(tokens[1]),
                                kind: TyKind::Struct,
                                generics: Vec::new(),
                            }
                        }
                        word::TypeKeyword::Rename => todo!(),
                        word::TypeKeyword::Enum => todo!(),
                        word::TypeKeyword::Props => todo!(),
                    },
                    Keyword::Use | Keyword::Mod => todo!(),
                    Keyword::Stmt(kw) => self
                        .parse_stmt(Some((kw, tokens[0].range.clone())), &tokens[1..])?
                        .into(),
                    Keyword::Config(cfg) => match cfg {
                        ConfigKeyword::Dataset => {
                            self.env.set_value(Env::DatasetConfig);
                            enter_block(self);
                            self.use_all(
                                BuiltinIdentifier::DatasetType.into(),
                                tokens[0].text_range(),
                            )?;
                            AstKind::DatasetConfig
                        }
                    },
                    Keyword::Def => todo!(),
                }
            } else {
                if tokens.len() >= 2 && tokens[1].kind == TokenKind::Special(Special::Colon) {
                    if tokens.len() == 2 {
                        todo!()
                    }
                    let ident = match tokens[0].kind {
                        TokenKind::Identifier(ident) => match ident {
                            Identifier::Builtin(_) => err!(
                                tokens[0].text_range(),
                                "expect custom identifier but got builtin"
                            )?,
                            Identifier::Implicit(_) => err!(
                                tokens[0].text_range(),
                                "expect implicit identifier but got builtin"
                            )?,
                            Identifier::Custom(custom_ident) => custom_ident,
                        },
                        _ => err!(tokens[0].text_range(), "expect custom identifier")?,
                    };
                    let ty = atom::parse_ty(self.symbol_proxy(), &tokens[2..])?;
                    AstKind::MembDef {
                        ident,
                        kind: MembKind::MembVar {
                            ty: MembType {
                                contract: Contract::Take,
                                scope: ty,
                            },
                        },
                    }
                } else {
                    self.parse_stmt(None, tokens)?.into()
                }
            },
        })
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
