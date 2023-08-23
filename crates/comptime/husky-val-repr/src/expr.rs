use crate::*;

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValExpr {
    #[return_ref]
    pub data: ValExprData,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValExprData {}
