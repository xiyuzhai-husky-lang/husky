use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub node_decl: UnitStructTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
}
