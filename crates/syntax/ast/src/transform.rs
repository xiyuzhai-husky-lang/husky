mod impl_class_item;
mod impl_enum_item;
mod impl_expr;
mod impl_module_item;
mod impl_morphism_decl;
mod impl_routine_decl;
mod impl_stmt;
mod impl_struct_item;
mod impl_symbol_proxy;
mod impl_ty_decl;
mod impl_use_all;
mod utils;

use crate::{
    query::{AstSalsaQueryGroup, AstText},
    *,
};
use atom::symbol_proxy::{Symbol, SymbolKind};
use entity_route::EntityRouteKind;
use file::FilePtr;
use fold::{FoldIter, FoldedList, LocalStack, LocalValue};
use token::*;

pub type AstIter<'a> = FoldIter<'a, AstResult<Ast>, FoldedList<AstResult<Ast>>>;

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FilePtr,
    file: FilePtr,
    arena: RawExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<AstContext>,
    this: LocalValue<Option<EntityRoutePtr>>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstSalsaQueryGroup, module: EntityRoutePtr) -> Self {
        return Self {
            db,
            main: db.main_file(db.module_file(module).unwrap()).unwrap(),
            file: db.module_file(module).unwrap(),
            arena: RawExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: module_symbols(db, module),
            env: LocalValue::new(match module.kind {
                EntityRouteKind::Package { main, ident } => AstContext::Package(main),
                EntityRouteKind::ChildScope { .. } => AstContext::Module(module),
                EntityRouteKind::Root { .. } | EntityRouteKind::Input { .. } => panic!(),
                EntityRouteKind::Generic { ident, .. } => todo!(),
                EntityRouteKind::ThisType => todo!(),
            }),
            this: LocalValue::new(None),
        };

        fn module_symbols(
            db: &dyn AstSalsaQueryGroup,
            module: EntityRoutePtr,
        ) -> LocalStack<Symbol> {
            let mut symbols = LocalStack::new();
            for scope in db.subscopes(module).iter() {
                match scope.kind {
                    EntityRouteKind::Root { .. }
                    | EntityRouteKind::Package { .. }
                    | EntityRouteKind::Input { .. } => panic!(),
                    EntityRouteKind::ChildScope { ident, .. } => symbols.push(Symbol {
                        ident,
                        kind: SymbolKind::Scope(scope.kind),
                    }),
                    EntityRouteKind::Generic { ident, .. } => todo!(),
                    EntityRouteKind::ThisType => todo!(),
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

    fn env(&self) -> AstContext {
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
                AstContext::Package(_) | AstContext::Module(_) => {
                    self.parse_module_item(token_group, enter_block)?
                }
                AstContext::DatasetConfig
                | AstContext::Main
                | AstContext::Morphism
                | AstContext::Func
                | AstContext::Proc
                | AstContext::Test => match token_group[0].kind {
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
                        Keyword::Main => todo!(),
                    },
                    _ => self.parse_stmt_without_keyword(token_group)?.into(),
                },
                AstContext::Struct => self.parse_struct_item(token_group, enter_block)?,
                AstContext::Enum => self.parse_enum_variant(token_group)?,
                AstContext::Record => self.parse_class_item(token_group, enter_block)?,
                AstContext::Props => todo!(),
            },
        })
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
