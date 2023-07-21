use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct InductiveTypeNodeDefn {
    #[id]
    pub node_path: TypeSynNodePath,
    pub node_decl: InductiveTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeDecl,
}
