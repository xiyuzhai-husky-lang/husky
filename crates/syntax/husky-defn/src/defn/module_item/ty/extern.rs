use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct ExternTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: ExternTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: ExternTypeDecl,
}
