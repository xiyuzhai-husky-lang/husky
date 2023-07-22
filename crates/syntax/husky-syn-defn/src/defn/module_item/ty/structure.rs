use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: StructureTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}
