#[test]
fn test_is_valid_ident() {
    assert_eq!(is_valid_word("a"), true);
    assert_eq!(is_valid_word("b"), true);
    assert_eq!(is_valid_word("c"), true);
    assert_eq!(is_valid_word("d"), true);
    assert_eq!(is_valid_word("e"), true);
    assert_eq!(is_valid_word("g"), true);
    assert_eq!(is_valid_word("h"), true);
    assert_eq!(is_valid_word("i"), true);
    assert_eq!(is_valid_word("j"), true);
    assert_eq!(is_valid_word("a1"), true);
    assert_eq!(is_valid_word("a2"), true);
    assert_eq!(is_valid_word("a3"), true);
    assert_eq!(is_valid_word("_a1"), true);
    assert_eq!(is_valid_word("_1"), true);
    assert_eq!(is_valid_word("_"), true);
    assert_eq!(is_valid_word("9"), false);
    assert_eq!(is_valid_word("9a"), false);
    assert_eq!(is_valid_word(" "), false);
    assert_eq!(is_valid_word("*"), false);
    assert_eq!(is_valid_word("&"), false);
    assert_eq!(is_valid_word(":"), false);
    assert_eq!(is_valid_word("{"), false);
    assert_eq!(is_valid_word("}"), false);
}

pub(crate) fn is_valid_word(word: &str) -> bool {
    let mut chars = word.chars();
    if let Some(start) = chars.next() {
        if !(start.is_alphabetic() || start == '_') {
            return false;
        }
    }
    for c in chars {
        if !(c.is_alphanumeric() || c == '_') {
            return false;
        }
    }
    true
}
