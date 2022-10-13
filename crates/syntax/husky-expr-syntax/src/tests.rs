use crate::*;
use husky_expect_test_utils::expect_test;
use husky_symbol_syntax::{SymbolContext, SymbolTestsDb};
use husky_test_utils::*;
use husky_token::Tokenize;

#[test]
fn it_works() {
    expect_test::<String, _>(test_parse_raw_exprs_debug);

    fn test_parse_raw_exprs_debug(text: &str) -> String {
        let db = SymbolTestsDb::new();
        let tokens = db.tokenize_line(text);
        let mut arena = RawExprArena::new();
        let mut ctx = db.fake_ctx();
        parse_raw_exprs(&mut ctx, &mut arena, &tokens);
        format!("{:#?}", arena)
    }
}
