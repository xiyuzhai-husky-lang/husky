mod db;

use crate::*;
use db::*;
use husky_token::Tokenize;
fn parse_raw_exprs_debug(text: &'static str) -> String {
    format!("{:#?}", parse_raw_exprs(text))
}

fn parse_raw_exprs(text: &'static str) -> RawExprArena {
    let db = ExprSyntaxTestsDb::new();
    let tokens = db.tokenize(text);
    let mut arena = RawExprArena::new();
    Automata::parse_raw_exprs(&db, &mut arena, &tokens);
    arena
}

#[test]
fn test_parse_raw_exprs1() {
    assert_eq!(parse_raw_exprs_debug("haha"), "");
}
