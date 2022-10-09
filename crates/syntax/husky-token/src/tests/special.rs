use super::*;

#[test]
fn test_add() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("+"),
        r#"[
    Token("+", [1:1, 1:2)),
]"#
    );
}

#[test]
fn test_minus() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("-"),
        r#"[
    Token("-", [1:1, 1:2)),
]"#
    );
}
