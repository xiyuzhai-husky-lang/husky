mod impl_abs_semantic_token;
mod impl_call;
mod impl_enum_item;
mod impl_expr;
mod impl_feature;
mod impl_module_item;
mod impl_parse_atoms;
mod impl_record_item;
mod impl_stmt;
mod impl_struct;
mod impl_symbol_context;
mod impl_ty;
mod impl_use;
mod impl_xml;

use crate::{
    query::{AstSalsaQueryGroup, AstText},
    *,
};
use atom::context::{Symbol, SymbolKind};
use entity_route::EntityRouteKind;
use entity_syntax::EntitySyntaxResult;
use file::FilePtr;
use fold::{FoldableIter, FoldableList, LocalStack, LocalValue};
use text::TextRanged;
use token::*;

pub type AstIter<'a> = FoldableIter<'a, AstResult<Ast>, FoldableList<AstResult<Ast>>>;

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FilePtr,
    file: FilePtr,
    arena: RawExprArena,
    symbols: LocalStack<Symbol>,
    context: LocalValue<AstContext>,
    opt_this_ty: LocalValue<Option<EntityRoutePtr>>,
    opt_this_liason: LocalValue<Option<ParameterLiason>>,
    pub(crate) folded_results: FoldableList<AstResult<Ast>>,
    abs_semantic_tokens: Vec<AbsSemanticToken>,
    tokenized_text: Arc<TokenizedText>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(
        db: &'a dyn AstSalsaQueryGroup,
        module: EntityRoutePtr,
    ) -> EntitySyntaxResult<Self> {
        let module_file = db.module_file(module)?;
        return Ok(Self {
            db,
            main: db.main_file(module_file).unwrap(),
            file: module_file,
            arena: RawExprArena::new(),
            folded_results: FoldableList::new(),
            symbols: module_symbols(db, module),
            context: LocalValue::new(match module.kind {
                EntityRouteKind::Package { main, .. } => AstContext::Package(main),
                EntityRouteKind::Child { .. } => AstContext::Module(module),
                _ => panic!(),
            }),
            opt_this_ty: LocalValue::new(None),
            opt_this_liason: LocalValue::new(None),
            abs_semantic_tokens: vec![],
            tokenized_text: db.tokenized_text(module_file)?,
        });

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

    fn context(&self) -> AstContext {
        self.context.value()
    }
}

impl<'a> fold::Transformer<[Token], TokenizedText, AstResult<Ast>> for AstTransformer<'a> {
    fn _enter_block(&mut self) {
        self.context.enter();
        self.symbols.enter();
        self.opt_this_ty.enter();
        self.opt_this_liason.enter();
    }

    fn _exit_block(&mut self) {
        self.context.exit();
        self.symbols.exit();
        self.opt_this_ty.exit();
        self.opt_this_liason.exit();
    }

    fn transform(
        &mut self,
        _indent: fold::Indent,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        Ok(Ast {
            range: token_group.text_range(),
            variant: match self.context() {
                AstContext::Package(_) | AstContext::Module(_) => {
                    self.parse_module_item(token_group, enter_block)?
                }
                AstContext::Stmt(_) | AstContext::Match(_) | AstContext::Visual => {
                    match token_group[0].kind {
                        TokenKind::Keyword(keyword) => match keyword {
                            Keyword::Config(_) => todo!(),
                            Keyword::Paradigm(_) => todo!(),
                            Keyword::Type(_) => todo!(),
                            Keyword::Stmt(keyword) => self
                                .parse_stmt_with_keyword(keyword, token_group, enter_block)?
                                .into(),
                            Keyword::Use => todo!(),
                            Keyword::Mod => todo!(),
                            Keyword::Main => todo!(),
                            Keyword::Visual => todo!(),
                            Keyword::Liason(_) => todo!(),
                        },
                        _ => self.parse_stmt_without_keyword(token_group)?.into(),
                    }
                }
                AstContext::Struct(struct_item_context) => {
                    self.parse_struct_item(token_group, struct_item_context, enter_block)?
                }
                AstContext::Enum(_) => self.parse_enum_variant(token_group)?,
                AstContext::Record => self.parse_record_item(token_group, enter_block)?,
                AstContext::Props => todo!(),
            },
        })
    }

    fn foldable_outputs_mut(&mut self) -> &mut FoldableList<AstResult<Ast>> {
        &mut self.folded_results
    }

    fn foldable_inputs(&self) -> &TokenizedText {
        &self.tokenized_text
    }

    fn misplaced(&self) -> AstResult<Ast> {
        derived_err!()
    }
}
