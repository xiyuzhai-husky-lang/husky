use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitVariantSynNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub syn_node_decl: UnitTypeVariantSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitVariantSynDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitTypeVariantSynDecl,
}
