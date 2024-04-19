use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_syn_decl::decl::MajorRitchieSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct MajorRitchieHirDecl {
    pub path: MajorFormPath,
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirExprRegion,
}

impl MajorRitchieHirDecl {
    pub(super) fn from_syn(
        path: MajorFormPath,
        syn_decl: MajorRitchieSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let ritchie_item_kind = syn_decl.ritchie_item_kind(db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = builder.return_ty_before_colon(syn_decl.return_ty(db));
        Self::new(
            db,
            path,
            ritchie_item_kind,
            template_parameters,
            parenate_parameters,
            return_ty,
            builder.finish().eager(),
        )
    }
}
