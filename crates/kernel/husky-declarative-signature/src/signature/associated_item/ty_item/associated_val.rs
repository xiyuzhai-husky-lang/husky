use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeAssociatedValDecTemplate {
    pub path: TypeItemPath,
}

impl TypeAssociatedValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssociatedValSynDecl,
    ) -> DecSignatureResult<TypeAssociatedValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = declarative_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TypeAssociatedValDecTemplate::new(db, path))
    }
}
