use crate::*;

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValStmt {
    pub region_path: RegionPath,
    #[return_ref]
    pub data: ValStmtData,
    pub domain: ValDomain,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ValStmtData {}
