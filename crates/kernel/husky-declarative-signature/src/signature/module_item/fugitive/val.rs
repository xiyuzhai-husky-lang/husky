use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ValDeclarativeSignatureTemplate {
    pub initialization_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for ValSynDecl {
    type DeclarativeSignatureTemplate = ValDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        val_declarative_signature_template(db, self)
    }
}

impl ValDeclarativeSignatureTemplate {
    #[inline(always)]
    pub fn generic_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeGenericParameter] {
        &[]
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn val_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: ValSynDecl,
) -> DeclarativeSignatureResult<ValDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let val_ty = match decl.return_ty(db) {
        Some(val_ty) => declarative_term_region.expr_term(val_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(ValDeclarativeSignatureTemplate::new(db, val_ty))
}
