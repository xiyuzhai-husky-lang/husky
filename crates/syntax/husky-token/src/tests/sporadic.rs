use husky_test_utils::expect::expect_test;

use super::*;

#[test]
fn test_tokenize1() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("struct A {}"),
        r#"[
    Token("struct", [1:1, 1:7)),
    Token("A", [1:8, 1:9)),
    Token("{", [1:10, 1:11)),
    Token("}", [1:11, 1:12)),
]"#
    );
}

#[test]
fn test_tokenize2() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("1 + b * t / 2 | 3.5"),
        r#"[
    Token("1", [1:1, 1:2)),
    Token("+", [1:3, 1:4)),
    Token("b", [1:5, 1:6)),
    Token("*", [1:7, 1:8)),
    Token("t", [1:9, 1:10)),
    Token("/", [1:11, 1:12)),
    Token("2", [1:13, 1:14)),
    Token("|", [1:15, 1:16)),
    Token("3.5", [1:17, 1:20)),
]"#
    );
}

#[test]
fn test_tokenize3() {
    use husky_word::Keyword::*;
    use TokenKind::*;
    assert_eq!(
        tokenize_debug("def f -> []i32"),
        r#"[
    Token("def", [1:1, 1:4)),
    Token("f", [1:5, 1:6)),
    Token("->", [1:7, 1:9)),
    Token("[", [1:10, 1:11)),
    Token("]", [1:11, 1:12)),
    Token("i32", [1:12, 1:15)),
]"#
    );
}

#[test]
fn test_tokenize() {
    expect_test::<String, _>("sporadic", tokenize_debug)
}
