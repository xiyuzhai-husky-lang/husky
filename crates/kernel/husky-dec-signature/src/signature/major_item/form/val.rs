use crate::*;
use husky_syn_decl::decl::major_item::form::val::MajorValSynDecl;

#[salsa::interned]
pub struct MajorValDecTemplate {
    pub return_ty: DecTerm,
}

impl MajorValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: MajorValSynDecl,
    ) -> DecSignatureResult<MajorValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let return_ty = dec_term_region.expr_term(decl.return_ty(db).syn_expr_idx())?;
        Ok(MajorValDecTemplate::new(db, return_ty))
    }

    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
