use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub node_decl: StructureTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}
