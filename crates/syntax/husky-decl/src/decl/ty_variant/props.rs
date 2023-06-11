use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantRawDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

impl PropsVariantDecl {
    fn from_raw(db: &dyn DeclDb, raw_decl: PropsVariantRawDecl) -> Self {
        PropsVariantDecl::new(db, raw_decl.path(db), raw_decl.expr_region(db))
    }
}
