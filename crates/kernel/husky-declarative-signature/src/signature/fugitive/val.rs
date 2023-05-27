use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ValDeclarativeSignatureTemplate {
    pub initialization_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for ValDecl {
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
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterDeclarativeSignature] {
        &[]
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn val_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: ValDecl,
) -> DeclarativeSignatureResult<ValDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let var_ty = match decl.var_ty(db) {
        Some(var_ty) => declarative_term_region.expr_term(var_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(ValDeclarativeSignatureTemplate::new(db, var_ty))
}
