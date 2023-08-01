use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct PropsStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[RegularFieldHirDecl; 4]>,
    pub hir_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct RegularFieldHirDecl {
    ident: Ident,
    ty: HirType,
}

impl PropsStructTypeHirDecl {
    pub(super) fn from_ethereal(
        path: TypePath,
        ethereal_signature_template: PropsStructTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        todo!()
    }
}
