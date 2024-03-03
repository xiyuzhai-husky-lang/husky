use super::*;
use husky_val::Val;

#[salsa::interned]
pub struct SuperNode {
    data: SuperNodeData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SuperNodeData {
    Val(Val),
    Gn,
    Vn,
    Qn,
    Tn,
}
