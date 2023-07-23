use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnionTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: UnionTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnionTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeSynDecl,
}
