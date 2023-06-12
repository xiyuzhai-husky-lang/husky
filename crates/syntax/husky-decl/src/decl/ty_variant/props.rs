use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantRawDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub expr_region: ExprRegion,
}

impl PropsVariantDecl {
    fn from_raw(db: &dyn DeclDb, raw_decl: PropsVariantRawDecl) -> Self {
        PropsVariantDecl::new(db, raw_decl.node_path(db), raw_decl.expr_region(db))
    }
}
