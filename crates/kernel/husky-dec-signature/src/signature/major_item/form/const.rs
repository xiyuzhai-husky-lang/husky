use crate::*;
use husky_syn_decl::decl::r#const::MajorConstSynDecl;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct MajorConstDecTemplate {
    pub return_ty: DecTerm,
}

impl MajorConstDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: MajorConstSynDecl,
    ) -> DecSignatureResult<MajorConstDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let return_ty = dec_term_region.expr_term(decl.return_ty(db).syn_expr_idx())?;
        Ok(MajorConstDecTemplate::new(db, return_ty))
    }

    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
