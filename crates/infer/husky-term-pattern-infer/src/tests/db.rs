mod trivia;

use super::*;
use husky_entity_path::{EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu};
use husky_expr::ExprIdx;
use husky_package_path::PackagePathJar;
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolKind};
use husky_term::{AskDecl, Decl, Term, TermDb, TermMenu, TermResultArc, TyDecl};
use husky_token::*;
use husky_toolchain::*;
use husky_word::WordDb;
use husky_word::WordJar;
use salsa::Database;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(
    TermJar,
    TermPatternInferJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    WordJar
)]
pub struct TermPatternInferFakeDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Term>,
    decls: HashMap<EntityPath, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl TermPatternInferFakeDb {
    pub(super) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            entity_tys: Default::default(),
            decls: Default::default(),
            prelude_symbols: Default::default(),
        };
        db
    }

    pub(crate) fn new_sheet(&self, arena: &ExprArena) -> TermPatternInferSheet {
        TermPatternInferSheet::new_test(arena, Default::default())
    }

    pub(super) fn parse_expr_from_text(&self, text: &str) -> (ExprArena, ExprIdx) {
        // use husky_tokenize::TokenizeDb;

        // let tokens = self.tokenize_line(text);
        // let mut arena = ExprArena::default();
        todo!()
        // let expr = parse_expr(&mut symbol_ctx, &mut arena, &tokens);
        // (arena, expr)
    }
}

impl AskDecl for TermPatternInferFakeDb {
    fn ask_namespace_decl(
        &self,
        _namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, _ty: Term) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, _entity_path: EntityPath) -> TermResultArc<Decl> {
        todo!()
    }
}
