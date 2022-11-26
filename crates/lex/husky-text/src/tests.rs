use husky_check_utils::should_eq;

use crate::*;

#[test]
fn test_text() {
    let text = Text::new(
        r#"abcd
efgh
  123456"#,
    );
    should_eq!(&text[(0, 0)..(0, 1)], "a");
    should_eq!(&text[(0, 0)..(0, 3)], "abc");
    should_eq!(&text[(0, 0)..(0, 4)], "abcd");
    should_eq!(&text[(0, 0)..(1, 1)], "abcde");
}

#[test]
fn test_string_length() {
    let single_letter = "a";
    assert_eq!(single_letter.len(), 1);
    let single_chinese_character = "好";
    assert_eq!(single_chinese_character.len(), 3);
    let single_greek_character = "α";
    assert_eq!(single_chinese_character.len(), 3);
    let single_symbol = "→";
    assert_eq!(single_symbol.len(), 3);
}
