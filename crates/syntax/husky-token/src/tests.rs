use std::sync::Arc;

use husky_print_utils::ep;
use husky_text::{Column, Row, TextPosition};
use husky_word::new_word_itr;

use crate::*;

fn tokenize_debug(text: &'static str) -> String {
    format!("{:#?}", new_word_itr().tokenize(text))
}

#[test]
fn test_play() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("struct A {}"),
        r#"[
    Token { kind: `struct`, range: [1:1, 1:7) },
    Token { kind: `A`, range: [1:8, 1:9) },
    Token { kind: `{`, range: [1:10, 1:11) },
    Token { kind: `}`, range: [1:11, 1:12) },
]"#
    );
}
