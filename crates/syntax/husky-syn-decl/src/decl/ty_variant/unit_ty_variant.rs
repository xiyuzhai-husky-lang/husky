use super::*;

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: SynExprRegion,
}

impl UnitTypeVariantDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        syn_node_decl: UnitTypeVariantNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, syn_node_decl.expr_region(db)))
    }
}
