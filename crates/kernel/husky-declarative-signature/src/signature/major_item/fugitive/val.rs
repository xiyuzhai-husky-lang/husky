use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct MajorValDecTemplate {
    pub return_ty: DeclarativeTerm,
}

impl MajorValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: MajorValSynDecl,
    ) -> DeclarativeSignatureResult<MajorValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let return_ty = declarative_term_region.expr_term(decl.return_ty(db).syn_expr_idx())?;
        Ok(MajorValDecTemplate::new(db, return_ty))
    }

    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
