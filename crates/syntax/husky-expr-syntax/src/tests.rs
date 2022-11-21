use crate::*;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_symbol_syntax::SymbolTestsDb;
use husky_tokenize::Tokenize;

#[test]
fn it_works() {
    expect_test_husky_to_rust("", &test_parse_raw_exprs_debug);

    fn test_parse_raw_exprs_debug(text: &str) -> String {
        let db = SymbolTestsDb::new();
        let tokens = db.tokenize_line(text);
        let mut arena = ExprArena::new();
        let mut ctx = db.fake_symbol_ctx();
        parse_raw_expr(&mut ctx, &mut arena, &tokens);
        format!("{:#?}", arena)
    }
}
