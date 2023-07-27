use crate::*;

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValExpr {}
