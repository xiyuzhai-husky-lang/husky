use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitVariantSynNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub node_decl: UnitTypeVariantNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitTypeVariantDecl,
}
