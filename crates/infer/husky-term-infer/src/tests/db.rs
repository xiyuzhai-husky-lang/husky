mod init;
mod trivia;

use super::*;
use husky_entity_path::{EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu};
use husky_expr_syntax::ExprIdx;
use husky_identifier::{IdentifierDb, IdentifierJar};
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolDb, SymbolKind};
use husky_term::{
    AskDecl, Decl, TermDb, TermError, TermJar, TermMenu, TermResult, TermResultArc, Ty, TyDecl,
};
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(TermJar, TermInferJar, EntityPathJar, IdentifierJar)]
pub(crate) struct TermInferTestsDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Ty>,
    decls: HashMap<EntityPath, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl TermInferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            entity_tys: Default::default(),
            decls: Default::default(),
            prelude_symbols: Default::default(),
        };
        db.init();
        db
    }

    pub fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        let mut ctx = SymbolContext::new(self, &self.prelude_symbols);
        let entity_path_menu = self.entity_path_menu();
        ctx
    }

    pub(super) fn parse_raw_expr_from_text(&self, text: &str) -> (ExprArena, ExprIdx) {
        use husky_tokenize::Tokenize;
        let tokens = self.tokenize_line(text);
        let mut arena = ExprArena::new();
        let mut symbol_ctx = self.fake_symbol_ctx();
        let expr = parse_raw_expr(&mut symbol_ctx, &mut arena, &tokens);
        (arena, expr)
    }
}

impl TyInferQueries for TermInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPath) -> Ty {
        self.entity_tys[&entity]
    }
}

impl AskDecl for TermInferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, entity_path: EntityPath) -> TermResultArc<Decl> {
        self.decls.get(&entity_path).map_or(
            Err(TermError::NoDeclForEntityPath { entity_path }),
            |decl| Ok(decl.clone()),
        )
    }
}
