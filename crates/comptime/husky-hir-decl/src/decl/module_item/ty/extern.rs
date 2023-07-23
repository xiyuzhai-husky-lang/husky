use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct ExternHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
}
