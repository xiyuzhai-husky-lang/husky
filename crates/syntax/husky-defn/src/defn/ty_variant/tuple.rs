use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: TupleTypeVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: TupleTypeVariantDecl,
}
