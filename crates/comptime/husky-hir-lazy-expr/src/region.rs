use crate::{db::*, *};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub struct HirLazyExprRegion {}
