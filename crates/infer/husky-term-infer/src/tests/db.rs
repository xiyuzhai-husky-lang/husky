mod init;
mod trivia;

use super::*;
use husky_entity_path::{
    EntityPath, EntityPathDb, EntityPathDbStorage, EntityPathInterner, EntityPathItd,
    EntityPathMenu, InternEntityPath,
};
use husky_expr_syntax::RawExprIdx;
use husky_symbol_syntax::{
    Symbol, SymbolContext, SymbolDb, SymbolDbStorage, SymbolKind, SymbolQueries,
};
use husky_term::{
    new_term_itr, AskDecl, Decl, TermDb, TermError, TermInterner, TermMenu, TermResult,
    TermResultArc, Ty, TyDecl,
};
use husky_term::{InternTerm, TermDbStorage};
use husky_word::{InternWord, RootBuiltinIdentifier, WordInterner};
use salsa::Database;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::database(
    TermDbStorage,
    SymbolDbStorage,
    TermInferDbStorage,
    EntityPathDbStorage
)]
pub(crate) struct TermInferTestsDb {
    storage: salsa::Storage<Self>,
    term_itr: TermInterner,
    entity_path_itr: EntityPathInterner,
    word_itr: WordInterner,
    entity_tys: HashMap<EntityPathItd, Ty>,
    decls: HashMap<EntityPathItd, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl TermInferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            term_itr: Default::default(),
            entity_path_itr: Default::default(),
            word_itr: Default::default(),
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

impl TyInferQueries for TermInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPathItd) -> Ty {
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

    fn ask_decl(&self, entity_path: EntityPathItd) -> TermResultArc<Decl> {
        self.decls.get(&entity_path).map_or(
            Err(TermError::NoDeclForEntityPath { entity_path }),
            |decl| Ok(decl.clone()),
        )
    }
}
