use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedValDeclarativeSignatureTemplate {
    pub path: TypeItemPath,
}

impl TypeAssociatedValDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssociatedValSynDecl,
    ) -> DeclarativeSignatureResult<TypeAssociatedValDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = declarative_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TypeAssociatedValDeclarativeSignatureTemplate::new(db, path))
    }
}
