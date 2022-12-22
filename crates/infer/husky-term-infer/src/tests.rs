mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_expr::{parse_expr, ExprArena};
use husky_print_utils::epin;
use husky_term::TermDb;
use husky_token::*;
use husky_vfs::VfsDb;

#[test]
fn test_decl() {
    // expect_test_husky_to_rust("decl", &debug_decl);

    fn debug_decl(text: &str) -> String {
        let db = TermInferTestsDb::new();
        let (arena, expr) = db.parse_expr_from_text(text);
        let mut sheet = TermSheet::new(&arena);
        let toolchain = db.dev_toolchain();
        let term_menu = db.term_menu(toolchain).as_ref().unwrap();
        let mut ctx = InferContext::new(&db, &mut sheet, &arena, expr, &term_menu);
        let term = ctx.term_result().unwrap();
        todo!()
        // format!("{:?}", db.decl(term.path()))
    }
}

#[test]
fn test_infer_ty_works() {
    // expect_test::<String, _>("", debug_infer_ty);

    fn debug_infer_ty(text: &str) -> String {
        todo!()
        //         let db = TermInferTestsDb::new();
        //         let (arena, expr) = db.parse_expr_from_text(text);
        //         let mut sheet = TermSheet::new(&arena);
        //         let term_menu = db.term_menu();
        //         InferContext::new(&db, &mut sheet, &arena, expr, &term_menu).run();
        //         format!(
        //             r#"raw expr arena:
        // {}
        // ty infer sheet:
        // {}"#,
        //             textwrap::indent(&format!("{:#?}", arena), "    "),
        //             textwrap::indent(&format!("{:#?}", sheet), "    "),
        //         )
    }
}
