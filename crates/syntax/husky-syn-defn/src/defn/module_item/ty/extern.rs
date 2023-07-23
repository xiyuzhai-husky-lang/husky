use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct ExternTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: ExternTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct ExternTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: ExternTypeSynDecl,
}
