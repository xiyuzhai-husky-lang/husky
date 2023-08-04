use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: UnitStructTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeSynDecl,
}
