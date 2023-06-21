use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: InductiveTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeDecl,
}
