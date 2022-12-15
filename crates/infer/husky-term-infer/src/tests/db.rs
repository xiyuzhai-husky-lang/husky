mod init;
mod trivia;

use super::*;
use husky_entity_path::{
    EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu, EntityPathMenuPlace,
};
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprIdx;
use husky_package_path::PackagePathJar;
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolDb, SymbolJar, SymbolKind};
use husky_term::{
    AskDecl, Decl, Term, TermDb, TermError, TermJar, TermMenu, TermResult, TermResultArc, TyDecl,
};
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_word::WordDb;
use husky_word::WordJar;
use salsa::ParallelDatabase;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(
    EntityTreeJar,
    EntityPathJar,
    PackagePathJar,
    ToolchainJar,
    WordJar,
    TermJar,
    TermInferJar,
    TokenJar,
    VfsJar,
    SymbolJar
)]
pub(crate) struct TermInferTestsDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Term>,
    decls: HashMap<EntityPath, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl HasVfsConfig for TermInferTestsDb {
    fn vfs_config(&self) -> &VfsConfig {
        todo!()
    }
}

impl ParallelDatabase for TermInferTestsDb {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        todo!()
    }
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

    pub(super) fn parse_expr_from_text(&self, text: &str) -> (ExprArena, ExprIdx) {
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

impl AskDecl for TermInferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Term) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, entity_path: EntityPath) -> TermResultArc<Decl> {
        self.decls.get(&entity_path).map_or(
            Err(TermError::NoDeclForEntityPath { entity_path }),
            |decl| Ok(decl.clone()),
        )
    }
}
