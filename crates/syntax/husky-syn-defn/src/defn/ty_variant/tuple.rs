use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleVariantSynNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub node_decl: TupleTypeVariantNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: TupleTypeVariantDecl,
}
