use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct DeriveDecrHirDecl {
    trai_term: EtherealTerm,
}
