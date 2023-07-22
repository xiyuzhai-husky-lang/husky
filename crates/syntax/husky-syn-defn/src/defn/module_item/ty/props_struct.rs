use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsStructTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: PropsStructTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsStructTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeDecl,
}
