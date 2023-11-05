use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ValFugitiveDeclarativeSignatureTemplate {
    pub initialization_ty: DeclarativeTerm,
}

impl ValFugitiveDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        decl: ValFugitiveSynDecl,
    ) -> DeclarativeSignatureResult<ValFugitiveDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let val_ty = match decl.return_ty(db) {
            Some(val_ty) => declarative_term_region.expr_term(val_ty.syn_expr_idx())?,
            None => declarative_term_menu.unit(),
        };
        Ok(ValFugitiveDeclarativeSignatureTemplate::new(db, val_ty))
    }

    #[inline(always)]
    pub fn template_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
