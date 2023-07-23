use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct UnionHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
}
