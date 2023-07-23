use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct InductiveTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: InductiveTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct InductiveTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeSynDecl,
}
