mod init;
mod trivia;
mod var;

use super::*;
use husky_entity_path::{
    EntityPathDb, EntityPathDbStorage, EntityPathInterner, EntityPathItd, EntityPathMenu,
    InternEntityPath,
};
use husky_expr_syntax::RawExprIdx;
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolDbStorage, SymbolKind, SymbolQueries};
use husky_term::{AskDecl, Decl, TermDb, TermInterner, TermMenu, TermResultArc, Ty, TyDecl};
use husky_term::{InternTerm, TermDbStorage};
use husky_token_syntax::*;
use husky_word::{InternWord, RootBuiltinIdentifier, WordInterner};
use salsa::Database;
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::database(
    TermDbStorage,
    SymbolDbStorage,
    TermPatternInferQueryGroupStorage,
    EntityPathDbStorage
)]
pub struct TermPatternInferFakeDb {
    storage: salsa::Storage<Self>,
    term_itr: TermInterner,
    entity_path_itr: EntityPathInterner,
    word_itr: WordInterner,
    entity_tys: HashMap<EntityPathItd, Ty>,
    decls: HashMap<EntityPathItd, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl TermPatternInferFakeDb {
    pub(super) fn new() -> Self {
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

    pub(crate) fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        SymbolContext::new(self, &self.prelude_symbols)
    }

    pub(crate) fn new_sheet(&self, arena: &RawExprArena) -> TermPatternInferSheet {
        TermPatternInferSheet::new_test(arena, Default::default())
    }

    pub(super) fn parse_raw_expr_from_text(&self, text: &str) -> (RawExprArena, RawExprIdx) {
        todo!()
        // let tokens = self.tokenize_line(text);
        // let mut arena = RawExprArena::new();
        // let mut symbol_ctx = self.fake_symbol_ctx();
        // let expr = parse_raw_expr(&mut symbol_ctx, &mut arena, &tokens);
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

    fn ask_ty_decl(&self, _ty: Ty) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, _entity_path: EntityPathItd) -> TermResultArc<Decl> {
        todo!()
    }
}
