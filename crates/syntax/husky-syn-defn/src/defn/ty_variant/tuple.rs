use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleVariantSynNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub syn_node_decl: TupleTypeVariantNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TupleVariantSynDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: TupleTypeVariantDecl,
}
