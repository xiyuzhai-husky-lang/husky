use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct PropsStructHirDecl {
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    #[return_ref]
    pub fields: SmallVec<[RegularFieldHirDecl; 4]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct RegularFieldHirDecl {
    ident: Ident,
    ty: EtherealTerm,
}
