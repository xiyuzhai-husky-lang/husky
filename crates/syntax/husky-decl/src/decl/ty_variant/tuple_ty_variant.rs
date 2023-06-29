use super::*;

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleTypeVariantNodeDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleTypeVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

impl TupleTypeVariantDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        node_decl: TupleTypeVariantNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, node_decl.expr_region(db)))
    }
}
