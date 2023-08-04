use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: StructureTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct StructureTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeSynDecl,
}
