// mod ident;
// mod opr;
// mod special;
// mod sporadic;

use std::sync::Arc;

use crate::*;
use husky_expect_test_utils::*;
use husky_print_utils::ep;
use husky_text::{Column, Row, TextPosition};
use husky_word::WordInterner;

#[test]
fn it_works() {
    expect_test::<String, _>("", tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", WordInterner::default().tokenize_line(text))
    }
}
