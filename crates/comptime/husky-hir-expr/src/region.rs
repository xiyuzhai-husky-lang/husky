use crate::*;

/// this is interned on purpose
#[salsa::interned(db = HirExprDb, jar = HirExprJar)]
pub struct HirExprRegion {}
