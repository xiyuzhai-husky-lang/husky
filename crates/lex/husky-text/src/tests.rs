use husky_check_utils::should_eq;

use crate::*;

#[test]
fn test_text() {
    let text = HuskyText::new("abcd\nefgh\n  123456");
    should_eq!(text.ranged(((0, 0)..(0, 1)).into()), "a");
    should_eq!(text.ranged(((0, 0)..(0, 3)).into()), "abc");
    should_eq!(text.ranged(((0, 0)..(0, 4)).into()), "abcd");
    should_eq!(text.ranged(((0, 0)..(1, 1)).into()), "abcde");
}
