use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsStructTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: PropsStructTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeDecl,
}
