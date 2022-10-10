use husky_expr_syntax::*;
use husky_symbol_syntax::{SymbolContext, SymbolTestsDb};
use husky_test_utils::*;
use husky_token::Tokenize;

fn test_parse_raw_exprs_debug(text: &str) -> String {
    format!("{:#?}", test_parse_raw_exprs(text))
}

fn test_parse_raw_exprs(text: &str) -> RawExprArena {
    let db = SymbolTestsDb::new();
    let tokens = db.tokenize(text);
    let mut arena = RawExprArena::new();
    let mut ctx = db.fake_ctx();
    parse_raw_exprs(&mut ctx, &mut arena, &tokens);
    arena
}

expect_test!(test_parse_raw_exprs_debug, ["sporadic"]);
