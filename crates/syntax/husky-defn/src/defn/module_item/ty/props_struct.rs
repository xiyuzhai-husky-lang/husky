use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RegularStructTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: RegularStructTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RegularStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RegularStructTypeDecl,
}
