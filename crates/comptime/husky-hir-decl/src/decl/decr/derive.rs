use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct DeriveDecrHirDecl {
    pub path: DecrPath,
    pub trai_term: EtherealTerm,
}
