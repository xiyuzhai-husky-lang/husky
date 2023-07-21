use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: UnitStructTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
}
