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
use fold::{FoldableIter, FoldableList, LocalStack, LocalValue};
use husky_atom::context::{Symbol, SymbolKind};
use husky_entity_route::EntityRouteVariant;
use husky_entity_syntax::EntitySyntaxResult;
use husky_file::FileItd;
use husky_text::TextRanged;
use husky_token::*;
use husky_word::Keyword;

pub type AstIter<'a> = FoldableIter<'a, FoldableList<AstResult<Ast>>>;

pub struct AstTransformer<'a> {
    db: &'a dyn AstSalsaQueryGroup,
    main: FileItd,
    file: FileItd,
    arena: RawExprArena,
    symbols: LocalStack<Symbol>,
    context: LocalValue<AstContext>,
    opt_base_ty: LocalValue<Option<EntityRoutePtr>>,
    opt_this_liason: LocalValue<Option<ParameterModifier>>,
    pub(crate) folded_results: FoldableList<AstResult<Ast>>,
    abs_semantic_tokens: Vec<AbsSemanticToken>,
    tokenized_text: Arc<TokenizedText>,
    infer_roots: Vec<AstEntrance>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(
        db: &'a dyn AstSalsaQueryGroup,
        module: EntityRoutePtr,
    ) -> EntitySyntaxResult<Self> {
        let module_file = db.module_file(module)?;
        return Ok(Self {
            db,
            main: db.module_target_entrance(module_file).unwrap(),
            file: module_file,
            arena: RawExprArena::new(),
            folded_results: FoldableList::new(),
            symbols: module_symbols(db, module),
            context: LocalValue::new(match module.variant {
                EntityRouteVariant::Package { main, .. } => AstContext::Package(main),
                EntityRouteVariant::Child { .. } => AstContext::Module(module),
                _ => panic!(),
            }),
            opt_base_ty: LocalValue::new(None),
            opt_this_liason: LocalValue::new(None),
            abs_semantic_tokens: vec![],
            tokenized_text: db.tokenized_text(module_file)?,
            infer_roots: vec![],
        });

        fn module_symbols(
            db: &dyn AstSalsaQueryGroup,
            module: EntityRoutePtr,
        ) -> LocalStack<Symbol> {
            let mut symbols = LocalStack::new();
            let subroute_table = db.subroute_table(module).unwrap();
            for entry in subroute_table.entries.iter() {
                if let Some(entry_ident) = entry.ident {
                    symbols.push(Symbol {
                        init_ident: entry_ident,
                        kind: SymbolKind::EntityRoute(db.subroute(
                            module,
                            entry_ident.ident,
                            Default::default(),
                        )),
                    })
                }
            }
            symbols
        }
    }

    pub(crate) fn push_infer_roots(&mut self, infer_roots: Vec<AstEntrance>) {
        self.infer_roots.extend(infer_roots)
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            file: self.file,
            arena: self.arena,
            folded_results: self.folded_results,
            semantic_tokens: self.abs_semantic_tokens,
            text: self.db.text(self.file).unwrap(),
            infer_roots: self.infer_roots,
        }
    }

    fn context(&self) -> AstContext {
        self.context.value()
    }
}

impl<'a> fold::Transformer for AstTransformer<'a> {
    type Input = [Token];
    type InputStorage = TokenizedText;
    type Output = AstResult<Ast>;

    fn _enter_block(&mut self) {
        self.context.enter();
        self.symbols.enter();
        self.opt_base_ty.enter();
        self.opt_this_liason.enter();
    }

    fn _exit_block(&mut self) {
        self.context.exit();
        self.symbols.exit();
        self.opt_base_ty.exit();
        self.opt_this_liason.exit();
    }

    fn transform(
        &mut self,
        _indent: fold::Indent,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        let variant = match self.context() {
            AstContext::Package(_) | AstContext::Module { .. } => {
                self.parse_module_item(token_group, enter_block)?
            }
            AstContext::Stmt { .. } | AstContext::Match { .. } | AstContext::Visual => {
                match token_group[0].kind {
                    TokenKind::Keyword(keyword) => match keyword {
                        Keyword::Stmt(keyword) => self
                            .parse_stmt_with_keyword(keyword, token_group, enter_block)?
                            .into(),
                        Keyword::Use => todo!(),
                        _ => todo!(),
                    },
                    _ => self.parse_stmt_without_keyword(token_group)?.into(),
                }
            }
            AstContext::Struct { .. } => self.parse_struct_item(token_group, enter_block)?,
            AstContext::Enum(_) => self.parse_enum_variant(token_group)?,
            AstContext::Record => self.parse_record_item(token_group, enter_block)?,
        };
        self.push_infer_roots(variant.ast_entrances());
        Ok(Ast {
            range: token_group.text_range(),
            variant,
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
