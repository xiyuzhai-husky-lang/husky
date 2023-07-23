use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct PropsStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    #[return_ref]
    pub fields: SmallVec<[RegularFieldHirDecl; 4]>,
    pub hir_expr_region: HirExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct RegularFieldHirDecl {
    ident: Ident,
    ty: EtherealTerm,
}
