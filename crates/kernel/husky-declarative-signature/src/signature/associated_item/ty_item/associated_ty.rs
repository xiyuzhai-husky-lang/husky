use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedTypeDeclarativeSignatureTemplate {}

impl HasDeclarativeSignatureTemplate for TypeAssociatedTypeDecl {
    type DeclarativeSignatureTemplate = TypeAssociatedTypeDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_associated_ty_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_associated_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAssociatedTypeDeclarativeSignatureTemplate::new(db))
}
