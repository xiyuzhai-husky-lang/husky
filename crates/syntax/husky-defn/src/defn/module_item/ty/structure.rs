use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: StructureTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}
