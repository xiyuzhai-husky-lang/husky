use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct StructureTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: StructureTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}
