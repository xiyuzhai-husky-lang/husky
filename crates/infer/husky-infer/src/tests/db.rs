use super::*;
use husky_entity_path::{
    new_entity_path_itr, EntityPath, EntityPathDb, EntityPathDbStorage, EntityPathInterner,
    EntityPathItd, InternEntityPath,
};
use husky_expr_syntax::RawExprIdx;
use husky_symbol_syntax::{
    Symbol, SymbolContext, SymbolDb, SymbolDbStorage, SymbolKind, SymbolQueries,
};
use husky_term::{new_term_itr, AskDecl, Decl, TermDb, TermError, TermInterner, TermResultArc, Ty};
use husky_term::{InternTerm, TermDbStorage};
use husky_word::{InternWord, RootBuiltinIdentifier, WordInterner};
use salsa::Database;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::database(TermDbStorage, SymbolDbStorage, InferDbStorage, EntityPathDbStorage)]
pub(crate) struct InferTestsDb {
    storage: salsa::Storage<Self>,
    term_itr: TermInterner,
    entity_path_itr: EntityPathInterner,
    word_itr: WordInterner,
    entity_tys: HashMap<EntityPathItd, Ty>,
    decls: HashMap<EntityPathItd, Arc<Decl>>,
}

impl Upcast<dyn TermDb> for InferTestsDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl InferTestsDb {
    pub fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        let mut ctx = SymbolContext::new(self);
        let entity_path_menu = self.entity_path_menu();
        ctx.define_symbol(Symbol {
            ident: RootBuiltinIdentifier::Core.into(),
            kind: SymbolKind::EntityPath(entity_path_menu.core()),
        });
        /* do something with ctx */
        ctx
    }

    pub(super) fn parse_raw_expr_from_text(&self, text: &str) -> (RawExprArena, RawExprIdx) {
        let tokens = self.tokenize_line(text);
        let mut arena = RawExprArena::new();
        let mut symbol_ctx = self.fake_symbol_ctx();
        let expr = parse_raw_expr(&mut symbol_ctx, &mut arena, &tokens);
        (arena, expr)
    }
}

impl Database for InferTestsDb {}

impl InternTerm for InferTestsDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternEntityPath for InferTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternWord for InferTestsDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl SymbolQueries for InferTestsDb {}

impl AskDecl for InferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<husky_term::TyDecl> {
        todo!()
    }

    fn ask_decl(&self, entity_path: EntityPathItd) -> TermResultArc<husky_term::Decl> {
        self.decls
            .get(&entity_path)
            .map_or(Err(TermError::NoDeclForEntityPath), |decl| Ok(decl.clone()))
    }
}

impl InferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            term_itr: Default::default(),
            entity_path_itr: Default::default(),
            word_itr: Default::default(),
            entity_tys: Default::default(),
            decls: Default::default(),
        };
        db.init();
        db
    }

    fn init(&mut self) {
        let term_menu = self.term_menu();
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("i32"))),
                term_menu.i32(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("i64"))),
                term_menu.i64(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("f32"))),
                term_menu.f32(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("f64"))),
                term_menu.f64(),
            )
            .is_none());
    }
}

impl TyInferQueries for InferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPathItd) -> Ty {
        self.entity_tys[&entity]
    }
}
