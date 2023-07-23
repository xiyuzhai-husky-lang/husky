use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleStructTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: TupleStructTypeSynNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleStructTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: TupleStructTypeSynDecl,
}
