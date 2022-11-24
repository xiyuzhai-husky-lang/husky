mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::*;

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
