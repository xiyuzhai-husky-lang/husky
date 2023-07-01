use super::*;

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitTypeVariantNodeDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitTypeVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

impl UnitTypeVariantDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        node_decl: UnitTypeVariantNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, node_decl.expr_region(db)))
    }
}
