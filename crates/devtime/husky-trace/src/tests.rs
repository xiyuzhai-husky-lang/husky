use husky_print_utils::p;

use crate::*;

#[test]
fn test_serialize() {
    p!(serde_json::to_string(&TraceTokenKind::Keyword));
}
