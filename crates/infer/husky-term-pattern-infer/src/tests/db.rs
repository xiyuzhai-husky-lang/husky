mod init;
mod trivia;
mod var;

use super::*;
use husky_entity_path::{EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu};
use husky_expr_syntax::ExprIdx;
use husky_identifier::IdentifierDb;
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolKind};
use husky_term::{AskDecl, Decl, TermDb, TermMenu, TermResultArc, Ty, TyDecl};
use husky_token::*;
use salsa::Database;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(TermJar, TermPatternInferJar, EntityPathJar, IdentifierJar)]
pub struct TermPatternInferFakeDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Ty>,
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
        db.init();
        db
    }

    pub(crate) fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        SymbolContext::new(self, &self.prelude_symbols)
    }

    pub(crate) fn new_sheet(&self, arena: &ExprArena) -> TermPatternInferSheet {
        TermPatternInferSheet::new_test(arena, Default::default())
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

impl AskDecl for TermPatternInferFakeDb {
    fn ask_namespace_decl(
        &self,
        _namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, _ty: Ty) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, _entity_path: EntityPath) -> TermResultArc<Decl> {
        todo!()
    }
}
