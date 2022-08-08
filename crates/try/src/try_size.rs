use std::sync::Arc;

#[test]
fn test_option_arc_size() {
    assert_eq!(std::mem::size_of::<Option<Arc<i32>>>(), 8)
}
