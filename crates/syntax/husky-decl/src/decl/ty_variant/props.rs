use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsVariantNodeDecl {
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
    fn from_raw(db: &dyn DeclDb, node_decl: PropsVariantNodeDecl) -> Self {
        PropsVariantDecl::new(db, node_decl.node_path(db), node_decl.expr_region(db))
    }
}
