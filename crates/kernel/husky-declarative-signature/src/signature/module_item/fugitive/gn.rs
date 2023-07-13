use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct GnDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameters,
    #[return_ref]
    pub explicit_parameters: DeclarativeSpecificParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for GnDecl {
    type DeclarativeSignatureTemplate = GnDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        gn_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn gn_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: GnDecl,
) -> DeclarativeSignatureResult<GnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let generic_parameters = DeclarativeGenericParameters::from_decl(
        decl.generic_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let explicit_parameters = DeclarativeSpecificParameters::from_decl(
        decl.explicit_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(GnDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        explicit_parameters,
        return_ty,
    ))
}

impl GnDeclarativeSignatureTemplate {}
