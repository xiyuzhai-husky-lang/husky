use husky_check_utils::should_eq;

use crate::*;

#[test]
fn test_text() {
    let text = Document::new(
        r#"abcd
efgh
  123456"#,
    );
    should_eq!(&text[(0, 0)..(0, 1)], "a");
    should_eq!(&text[(0, 0)..(0, 3)], "abc");
    should_eq!(&text[(0, 0)..(0, 4)], "abcd");
    should_eq!(&text[(0, 0)..(1, 1)], "abcd\ne");
}

#[test]
fn test_string_length() {
    assert_eq!("a".len(), 1);
    assert_eq!("α".len(), 2);
    assert_eq!("好".len(), 3);
    assert_eq!("→".len(), 3);
    assert_eq!("𒀀".len(), 4);
}
