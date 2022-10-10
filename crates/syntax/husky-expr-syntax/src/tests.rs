mod db;

use crate::*;
use db::*;
use husky_token::Tokenize;
use husky_variable_syntax::{VariableContext, VariableTestsDb};

fn parse_raw_exprs_debug(text: &'static str) -> String {
    format!("{:#?}", parse_raw_exprs(text))
}

fn parse_raw_exprs(text: &'static str) -> RawExprArena {
    let db = VariableTestsDb::new();
    let tokens = db.tokenize(text);
    let mut arena = RawExprArena::new();
    let mut ctx = db.fake_ctx();
    Automata::parse_raw_exprs(&mut ctx, &mut arena, &tokens);
    arena
}

#[test]
fn test_parse_raw_exprs1() {
    assert_eq!(parse_raw_exprs_debug("haha"), "");
}
