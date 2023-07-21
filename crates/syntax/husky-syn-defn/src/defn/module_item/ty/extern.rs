use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct ExternTypeNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub node_decl: ExternTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: ExternTypeDecl,
}
