use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct ValDeclarativeSignatureTemplate {
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for ValDecl {
    type DeclarativeSignatureTemplate = ValDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        var_signature(db, self)
    }
}

impl ValDeclarativeSignatureTemplate {
    #[inline(always)]
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        &[]
    }
}

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub fn var_signature(
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
