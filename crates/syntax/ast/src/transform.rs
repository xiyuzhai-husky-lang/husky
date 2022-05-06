mod impl_abs_semantic_token;
mod impl_enum_item;
mod impl_expr;
mod impl_module_item;
mod impl_morphism_decl;
mod impl_parse_atoms;
mod impl_record_item;
mod impl_routine_decl;
mod impl_stmt;
mod impl_struct_item;
mod impl_symbol_context;
mod impl_ty;
mod impl_use_all;

use crate::{
    query::{AstSalsaQueryGroup, AstText},
    *,
};
use atom::symbol::{Symbol, SymbolKind};
use entity_route::EntityRouteKind;
use file::FilePtr;
use fold::{FoldIter, FoldedList, LocalStack, LocalValue};
use token::*;
use vm::InputContract;

pub type AstIter<'a> = FoldIter<'a, AstResult<Ast>, FoldedList<AstResult<Ast>>>;

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FilePtr,
    file: FilePtr,
    arena: RawExprArena,
    symbols: LocalStack<Symbol>,
    env: LocalValue<AstContext>,
    opt_this_ty: LocalValue<Option<EntityRoutePtr>>,
    opt_this_contract: LocalValue<Option<InputContract>>,
    folded_results: FoldedList<AstResult<Ast>>,
    abs_semantic_tokens: Vec<AbsSemanticToken>,
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
                EntityRouteKind::Package { main, .. } => AstContext::Package(main),
                EntityRouteKind::Child { .. } => AstContext::Module(module),
                _ => panic!(),
            }),
            opt_this_ty: LocalValue::new(None),
            opt_this_contract: LocalValue::new(None),
            abs_semantic_tokens: vec![],
        };

        fn module_symbols(
            db: &dyn AstSalsaQueryGroup,
            module: EntityRoutePtr,
        ) -> LocalStack<Symbol> {
            let mut symbols = LocalStack::new();
            for route in db.subscopes(module).iter() {
                match route.kind {
                    EntityRouteKind::Root { .. }
                    | EntityRouteKind::Package { .. }
                    | EntityRouteKind::Input { .. } => panic!(),
                    EntityRouteKind::Child { ident, .. } => symbols.push(Symbol {
                        ident,
                        kind: SymbolKind::EntityRoute(*route),
                    }),
                    EntityRouteKind::Generic { .. } => panic!(),
                    EntityRouteKind::ThisType => panic!(),
                    EntityRouteKind::TypeAsTraitMember { .. } => panic!(),
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
            semantic_tokens: self.abs_semantic_tokens,
            text: self.db.text(self.file).unwrap(),
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
        self.opt_this_ty.enter();
        self.opt_this_contract.enter();
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
                AstContext::Enum(_) => self.parse_enum_variant(token_group)?,
                AstContext::Record => self.parse_record_item(token_group, enter_block)?,
                AstContext::Props => todo!(),
            },
        })
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
