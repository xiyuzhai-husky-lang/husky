use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct RecordTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: RecordTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct RecordTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeSynDecl,
}
