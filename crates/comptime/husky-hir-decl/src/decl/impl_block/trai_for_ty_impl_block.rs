use super::*;
use husky_hir_ty::trai::HirTrait;
use smallvec::SmallVec;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct TraitForTypeImplBlockHirDecl {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub trai: HirTrait,
    pub self_ty: HirType,
    // todo: where clause
}

impl HasHirDecl for TraitForTypeImplBlockPath {
    type HirDecl = TraitForTypeImplBlockHirDecl;

    fn hir_decl_with_source_map(
        self,
        db: &dyn HirDeclDb,
    ) -> Option<(Self::HirDecl, Self::HirExprSourceMap)> {
        trai_for_ty_impl_block_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_impl_block_hir_decl(
    db: &dyn HirDeclDb,
    path: TraitForTypeImplBlockPath,
) -> Option<TraitForTypeImplBlockHirDecl> {
    let ethereal_signature_template = path.ethereal_signature_template(db).expect("ok");
    let self_ty = match ethereal_signature_template.self_ty_refined(db) {
        EtherealSelfTypeInTraitImpl::PathLeading(self_ty) => HirType::from_ethereal(self_ty, db),
        EtherealSelfTypeInTraitImpl::DeriveAny(_) => return None, // maybe we shouldn't do this???
    };
    let template_parameters = HirTemplateParameters::from_ethereal(
        ethereal_signature_template.template_parameters(db),
        db,
    );
    let trai = HirTrait::from_ethereal(ethereal_signature_template.trai(db), db);
    Some(TraitForTypeImplBlockHirDecl::new(
        db,
        path,
        template_parameters,
        trai,
        self_ty,
    ))
}
