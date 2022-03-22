mod impl_parse_enum;
mod impl_parse_expr;
mod impl_parse_func_decl;
mod impl_parse_module;
mod impl_parse_stmt;
mod impl_parse_struct_item;
mod impl_symbol_proxy;
mod impl_use_all;
mod utils;

use file::FilePtr;
use fold::{FoldIter, FoldedList, LocalStack, LocalValue};
use scope::ScopeRoute;
use token::*;

use crate::{
    atom::symbol_proxy::{Symbol, SymbolKind},
    query::{AstSalsaQueryGroup, AstText},
    *,
};

pub type AstIter<'a> = FoldIter<'a, AstResult<Ast>, FoldedList<AstResult<Ast>>>;

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FilePtr,
    file: FilePtr,
    arena: RawExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<Env>,
    this: LocalValue<Option<ScopePtr>>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstSalsaQueryGroup, module: ScopePtr) -> Self {
        return Self {
            db,
            main: db.main_file(db.module_file(module).unwrap()).unwrap(),
            file: db.module_file(module).unwrap(),
            arena: RawExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: module_symbols(db, module),
            env: LocalValue::new(match module.route {
                ScopeRoute::Package { main, ident } => Env::Package(main),
                ScopeRoute::ChildScope { .. } => Env::Module(module),
                ScopeRoute::Builtin { .. } | ScopeRoute::Implicit { .. } => panic!(),
            }),
            this: LocalValue::new(None),
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
            file: self.file,
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
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        Ok(Ast {
            range: token_group.into(),
            kind: match self.env() {
                Env::Package(_) | Env::Module(_) => {
                    self.parse_module_item(token_group, enter_block)?
                }
                Env::DatasetConfig | Env::Main | Env::Def | Env::Func | Env::Proc | Env::Test => {
                    match token_group[0].kind {
                        TokenKind::Keyword(keyword) => match keyword {
                            Keyword::Config(_) => todo!(),
                            Keyword::Routine(_) => todo!(),
                            Keyword::Type(_) => todo!(),
                            Keyword::Stmt(keyword) => {
                                self.parse_stmt_with_keyword(keyword, token_group)?.into()
                            }
                            Keyword::Def => todo!(),
                            Keyword::Use => todo!(),
                            Keyword::Mod => todo!(),
                        },
                        _ => self.parse_stmt_without_keyword(token_group)?.into(),
                    }
                }
                Env::Struct => self.parse_struct_item(token_group, enter_block)?,
                Env::Enum => self.parse_enum_variant(token_group)?,
            },
        })
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
