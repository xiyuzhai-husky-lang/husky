// mod ident;
// mod opr;
// mod special;
// mod sporadic;



use crate::*;
use husky_expect_test_utils::*;


use husky_word::WordInterner;

#[test]
fn it_works() {
    expect_test::<String, _>("", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", WordInterner::default().tokenize_line(text))
    }
}
