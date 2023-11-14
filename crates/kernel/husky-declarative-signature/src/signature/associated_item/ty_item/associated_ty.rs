use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedTypeDeclarativeSignatureTemplate {
    pub path: TypeItemPath,
}

impl TypeAssociatedTypeDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypeItemPath,
        decl: TypeAssociatedTypeSynDecl,
    ) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = declarative_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TypeAssociatedTypeDeclarativeSignatureTemplate::new(
            db, path,
        ))
    }
}
