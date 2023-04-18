use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ValDeclarativeSignature {
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignature for ValDecl {
    type DeclarativeSignature = ValDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        var_signature(db, self)
    }
}

impl ValDeclarativeSignature {
    #[inline(always)]
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        &[]
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn var_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: ValDecl,
) -> DeclarativeSignatureResult<ValDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let var_ty = match decl.var_ty(db) {
        Some(var_ty) => declarative_term_region.expr_term(var_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(ValDeclarativeSignature::new(db, var_ty))
}
