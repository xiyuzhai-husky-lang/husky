use super::*;
use husky_entity_path::{
    new_entity_path_itr, EntityPath, EntityPathInterner, EntityPathPtr, InternEntityPath,
};
use husky_expr_syntax::RawExprIdx;
use husky_symbol_syntax::{SymbolContext, SymbolDb, SymbolDbStorage, SymbolQueries};
use husky_term::{new_term_itr, AskDecl, TermDb, TermInterner, Ty};
use husky_term::{InternTerm, TermDbStorage};
use husky_word::{InternWord, WordInterner};
use salsa::Database;
use std::collections::HashMap;

#[salsa::database(TermDbStorage, SymbolDbStorage, TyInferDbStorage)]
pub(crate) struct TyInferTestsDb {
    storage: salsa::Storage<Self>,
    term_itr: TermInterner,
    entity_path_itr: EntityPathInterner,
    word_itr: WordInterner,
    entity_tys: HashMap<EntityPathPtr, Ty>,
}

impl TyInferTestsDb {
    pub(super) fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        SymbolContext::new(self)
    }

    pub(super) fn parse_raw_expr_from_text(&self, text: &str) -> (RawExprArena, RawExprIdx) {
        let tokens = self.tokenize_line(text);
        let mut arena = RawExprArena::new();
        let mut symbol_ctx = self.fake_symbol_ctx();
        let expr = parse_raw_expr(&mut symbol_ctx, &mut arena, &tokens);
        (arena, expr)
    }
}

impl Database for TyInferTestsDb {}

impl InternTerm for TyInferTestsDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternEntityPath for TyInferTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternWord for TyInferTestsDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl SymbolQueries for TyInferTestsDb {}

impl AskDecl for TyInferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> husky_term::TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> husky_term::TermResultArc<husky_term::TyDecl> {
        todo!()
    }
}

impl TyInferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            term_itr: Default::default(),
            entity_path_itr: Default::default(),
            word_itr: Default::default(),
            entity_tys: Default::default(),
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

impl TyInferQueries for TyInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPathPtr) -> Ty {
        self.entity_tys[&entity]
    }
}
