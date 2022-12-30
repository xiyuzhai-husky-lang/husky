use super::*;
use husky_ast::AstJar;
use husky_entity_path::{EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu};
use husky_entity_tree::EntityTreeJar;
use husky_expr::{ExprIdx, ExprSheet};
use husky_manifest::ManifestJar;
use husky_symbol::{Symbol, SymbolContext};
use husky_term::{
    Decl, Term, TermDb, TermError, TermJar, TermMenu, TermResult, TermResultArc, TyDecl,
};
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordDb;
use husky_word::WordJar;
use salsa::ParallelDatabase;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(
    EntityTreeJar,
    EntityPathJar,
    VfsJar,
    WordJar,
    TermJar,
    TermInferJar,
    TokenJar,
    AstJar,
    ManifestJar
)]
pub(crate) struct TermInferTestsDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Term>,
    decls: HashMap<EntityPath, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl salsa::Database for TermInferTestsDb {}

impl Upcast<dyn TermDb> for TermInferTestsDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl TermInferTestsDb {
    pub(crate) fn new() -> Self {
        Self {
            storage: Default::default(),
            entity_tys: Default::default(),
            decls: Default::default(),
            prelude_symbols: Default::default(),
        }
    }

    pub(super) fn parse_expr_from_text(&self, text: &str) -> (ExprSheet, ExprIdx) {
        todo!()
        // use husky_tokenize::TokenizeDb;
        // let tokens = self.tokenize_line(text);
        // let mut arena = ExprArena::default();
        // let mut ctx = self.new_symbol_ctx();
        // let expr = parse_expr(self, &tokens, &mut ctx, &mut arena);
        // (arena, expr)
    }
}

impl TyInferQueries for TermInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPath) -> Term {
        self.entity_tys[&entity]
    }
}
