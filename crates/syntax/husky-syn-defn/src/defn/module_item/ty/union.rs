use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnionTypeNodeDefn {
    #[id]
    pub node_path: TypeSynNodePath,
    pub node_decl: UnionTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
}
