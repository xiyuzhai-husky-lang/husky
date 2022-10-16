mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::expect_test;
use husky_expr_syntax::{parse_raw_expr, RawExprArena};
use husky_print_utils::epin;
use husky_term::TermDb;
use husky_token::*;

#[test]
fn test_infer_ty_works() {
    expect_test::<String, _>("", debug_infer_ty);

    fn debug_infer_ty(text: &str) -> String {
        let mut db = TyInferTestsDb::new();
        let tokens = db.tokenize_line(text);
        let mut arena = RawExprArena::new();
        let mut symbol_ctx = db.fake_symbol_ctx();
        let expr = parse_raw_expr(&mut symbol_ctx, &mut arena, &tokens);
        let mut sheet = TyInferSheet::new(&arena);
        let term_menu = db.term_menu();
        TyInferContext::new(&db, &mut sheet, &arena, expr, &term_menu).run();
        format!(
            r#"raw expr arena:
{}
ty infer sheet:
{}"#,
            textwrap::indent(&format!("{:#?}", arena), "    "),
            textwrap::indent(&format!("{:#?}", sheet), "    "),
        )
    }
}
