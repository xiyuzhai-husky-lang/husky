use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedTypeDeclarativeSignatureTemplate {}

impl HasDeclarativeSignatureTemplate for TypeAssociatedTypeSynDecl {
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
    decl: TypeAssociatedTypeSynDecl,
) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let _declarative_term_region = declarative_term_region(db, syn_expr_region);
    let _declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    Ok(TypeAssociatedTypeDeclarativeSignatureTemplate::new(db))
}
