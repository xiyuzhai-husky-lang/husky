use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: UnionTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
}
