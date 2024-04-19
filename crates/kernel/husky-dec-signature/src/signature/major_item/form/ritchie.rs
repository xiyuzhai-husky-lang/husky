use crate::*;
use husky_entity_kind::ritchie::RitchieItemKind;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct MajorRitchieDecTemplate {
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DecTerm,
}

impl MajorRitchieDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: MajorRitchieSynDecl,
    ) -> DecSignatureResult<Self> {
        let ritchie_item_kind = decl.ritchie_item_kind(db);
        let syn_expr_region = decl.syn_expr_region(db);
        let expr_region_data = syn_expr_region.data(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            expr_region_data,
            dec_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => dec_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => dec_term_menu.unit(),
        };
        Ok(MajorRitchieDecTemplate::new(
            db,
            ritchie_item_kind,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}
