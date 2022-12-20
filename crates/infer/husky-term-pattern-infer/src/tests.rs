use crate::*;

#[test]
fn test_infer_ty_works() {
    //     expect_test_husky_to_rust("", &|text: &str| -> String {
    //         let db = TermPatternInferFakeDb::new();
    //         let (arena, expr) = db.parse_expr_from_text(text);
    //         let mut sheet = db.new_sheet(&arena);
    //         let term_menu = db.term_menu();
    //         TermPatternInferContext::new(&db, &arena, expr, &term_menu).write_inference(&mut sheet);
    //         format!(
    //             r#"raw_expr_arena = {:#?};

    // ty_infer_sheet = {:#?};"#,
    //             arena, sheet
    //         )
    //     });
}

use husky_entity_path::{EntityPath, EntityPathJar};
use husky_expr::ExprIdx;
use husky_package_path::PackagePathJar;
use husky_symbol_syntax::Symbol;
use husky_term::{Decl, Term, TermDb};

use husky_toolchain::*;

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

impl Upcast<dyn TermDb> for TermPatternInferFakeDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for TermPatternInferFakeDb {}

impl TermPatternInferFakeDb {
    pub(super) fn new() -> Self {
        let db = Self {
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

    pub(super) fn parse_expr_from_text(&self, _text: &str) -> (ExprArena, ExprIdx) {
        // use husky_tokenize::TokenizeDb;

        // let tokens = self.tokenize_line(text);
        // let mut arena = ExprArena::default();
        todo!()
        // let expr = parse_expr(&mut symbol_ctx, &mut arena, &tokens);
        // (arena, expr)
    }
}
