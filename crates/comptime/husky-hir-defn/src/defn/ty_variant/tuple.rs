use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleVariantHirNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantHirNodePath,
    pub syn_node_decl: TupleTypeVariantHirNodeDecl,
}

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: TupleTypeVariantHirDecl,
}
