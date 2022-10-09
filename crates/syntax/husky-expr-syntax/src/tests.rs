use crate::*;
use husky_token::Tokenize;
fn parse_raw_exprs_debug(text: &'static str) -> String {
    format!("{:#?}", parse_raw_exprs(text))
}

fn parse_raw_exprs(text: &'static str) -> RawExprArena {
    let word_itr = husky_word::new_word_itr();
    let tokens = word_itr.tokenize(text);
    let mut arena = RawExprArena::new();
    Automata::parse_raw_exprs(&mut arena, &tokens);
    arena
}

#[test]
fn test_parse_raw_exprs1() {
    assert_eq!(parse_raw_exprs_debug("haha"), "");
}
