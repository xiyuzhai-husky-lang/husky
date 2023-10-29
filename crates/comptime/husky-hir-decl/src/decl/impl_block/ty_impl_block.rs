use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeImplBlockHirDecl {
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_ty: HirType,
}

impl HasHirDecl for TypeImplBlockPath {
    type HirDecl = TypeImplBlockHirDecl;

    fn hir_decl_with_source_map(
        self,
        db: &dyn HirDeclDb,
    ) -> Option<(Self::HirDecl, Self::HirExprSourceMap)> {
        Some(ty_impl_block_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn ty_impl_block_hir_decl(db: &dyn HirDeclDb, path: TypeImplBlockPath) -> TypeImplBlockHirDecl {
    let ethereal_signature_template = path.ethereal_signature_template(db).expect("ok");
    let self_ty = HirType::from_ethereal(ethereal_signature_template.self_ty(db), db);
    let template_parameters = HirTemplateParameters::from_ethereal(
        ethereal_signature_template.template_parameters(db),
        db,
    );
    TypeImplBlockHirDecl::new(db, path, template_parameters, self_ty)
}
