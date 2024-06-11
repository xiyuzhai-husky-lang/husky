use super::*;
use husky_syn_decl::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynDecl;

#[salsa::interned]
pub struct TraitForTypeAssocValHirDecl {
    pub path: TraitForTypeItemPath,
    pub return_ty: HirType,
    pub hir_expr_region: HirExprRegion,
}

impl TraitForTypeAssocValHirDecl {
    pub(super) fn from_syn(
        _path: TraitForTypeItemPath,
        syn_decl: TraitForTypeAssocValSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
        // let self_value_parameter =
        //     HirRitchieParameter::from_syn_regular(template.self_value_parameter(db), db);
        // let parenate_parameters =
        //     HirParenateParameters::from_syn(template.parenate_parameters(db), db);
        // let return_ty = builder.hir_ty(template.return_ty(db), db);
        // TraitForTypeMethodRitchieHirDecl::new(
        //     db,
        //     path,
        //     template_parameters,
        //     self_value_parameter,
        //     parenate_parameters,
        //     return_ty,
        // )
    }
}
