use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleStructTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: TupleStructTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: TupleStructTypeDecl,
}
