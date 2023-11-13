use super::*;



#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct TraitForTypeImplBlockHirDecl {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    // pub trai: HirTrait,
    // pub self_ty: HirType,
    // todo: where clause
}

impl HasHirDecl for TraitForTypeImplBlockPath {
    type HirDecl = TraitForTypeImplBlockHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        trai_for_ty_impl_block_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_impl_block_hir_decl(
    db: &dyn HirDeclDb,
    path: TraitForTypeImplBlockPath,
) -> Option<TraitForTypeImplBlockHirDecl> {
    let syn_decl = path.syn_decl(db).expect("ok");
    // let ethereal_signature_template = path.ethereal_signature_template(db).expect("ok");
    // let self_ty = match ethereal_signature_template.self_ty_refined(db) {
    //     EtherealSelfTypeInTraitImpl::PathLeading(self_ty) => builder.hir_ty(self_ty, db),
    //     EtherealSelfTypeInTraitImpl::DeriveAny(_) => return None, // maybe we shouldn't do this???
    // };
    let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
    let template_parameters =
        HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
    // let trai = HirTrait::from_syn(syn_decl.trai(db), db);
    Some(TraitForTypeImplBlockHirDecl::new(
        db,
        path,
        template_parameters,
        // trai,
        // self_ty,
    ))
}
