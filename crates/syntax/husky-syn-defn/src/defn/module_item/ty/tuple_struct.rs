use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleStructTypeNodeDefn {
    #[id]
    pub node_path: TypeSynNodePath,
    pub node_decl: TupleStructTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: TupleStructTypeDecl,
}
