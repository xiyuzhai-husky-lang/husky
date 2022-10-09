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
    Token {
        range: TextRange {
            start: TextPosition {
                row: 1,
                col: Column(
                    0,
                ),
            },
            end: TextPosition {
                row: 1,
                col: Column(
                    6,
                ),
            },
        },
        kind: Keyword(
            Type(
                Struct,
            ),
        ),
    },
    Token {
        range: TextRange {
            start: TextPosition {
                row: 1,
                col: Column(
                    7,
                ),
            },
            end: TextPosition {
                row: 1,
                col: Column(
                    8,
                ),
            },
        },
        kind: Identifier(
            Custom(
                A,
            ),
        ),
    },
    Token {
        range: TextRange {
            start: TextPosition {
                row: 1,
                col: Column(
                    9,
                ),
            },
            end: TextPosition {
                row: 1,
                col: Column(
                    10,
                ),
            },
        },
        kind: Special(
            LCurl,
        ),
    },
    Token {
        range: TextRange {
            start: TextPosition {
                row: 1,
                col: Column(
                    10,
                ),
            },
            end: TextPosition {
                row: 1,
                col: Column(
                    11,
                ),
            },
        },
        kind: Special(
            RCurl,
        ),
    },
]"#
    );
}
