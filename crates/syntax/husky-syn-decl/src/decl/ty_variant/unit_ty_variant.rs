use super::*;

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub syn_expr_region: SynExprRegion,
}

impl UnitTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypeVariantPath,
        syn_node_decl: UnitTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, syn_node_decl.syn_expr_region(db)))
    }
}
