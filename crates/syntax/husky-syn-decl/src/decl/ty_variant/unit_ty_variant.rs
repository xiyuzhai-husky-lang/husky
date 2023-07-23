use super::*;

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: SynExprRegion,
}

impl UnitTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeVariantPath,
        syn_node_decl: UnitTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, syn_node_decl.expr_region(db)))
    }
}
