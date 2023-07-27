use crate::*;

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {}
