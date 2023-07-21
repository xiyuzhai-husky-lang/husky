use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsStructTypeNodeDefn {
    #[id]
    pub node_path: TypeSynNodePath,
    pub node_decl: PropsStructTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeDecl,
}
