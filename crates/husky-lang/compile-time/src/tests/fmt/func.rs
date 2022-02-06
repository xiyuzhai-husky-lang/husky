use crate::*;

use super::utils;

#[test]
fn func() {
    utils::test_invariance_under_fmt(
        r#"
func f(x: i32, y: i32) -> i32:
    return x + y
"#,
    );
}
