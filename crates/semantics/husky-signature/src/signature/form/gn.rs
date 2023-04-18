use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct GnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl HasDeclarativeSignature for GnDecl {
    type DeclarativeSignature = GnSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        gn_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn gn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: GnDecl,
) -> DeclarativeSignatureResult<GnSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(GnSignature::new(db, implicit_parameters))
}

impl GnSignature {}
