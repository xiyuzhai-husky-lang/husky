mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::expect_test;
use husky_expr_syntax::{parse_raw_exprs, RawExprArena};
use husky_print_utils::epin;
use husky_token::*;

#[test]
fn test_infer_ty_works() {
    expect_test::<String, _>("", debug_infer_ty);

    fn debug_infer_ty(text: &str) -> String {
        let db = TyInferTestsDb::new();
        let tokens = db.tokenize_line(text);
        let mut arena = RawExprArena::new();
        let mut ctx = db.fake_symbol_ctx();
        parse_raw_exprs(&mut ctx, &mut arena, &tokens);
        epin!();
        format!("{:#?}", arena)
    }
}
