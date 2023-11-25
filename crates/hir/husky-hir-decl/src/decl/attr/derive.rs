use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct DeriveAttrHirDecl {
    pub path: AttrItemPath,
    pub trai_term: EtherealTerm,
}
