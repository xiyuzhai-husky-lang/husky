use crate::*;

#[salsa::tracked(db = HirExprDb, jar = HirExprJar)]
pub struct HirExprRegion {}
