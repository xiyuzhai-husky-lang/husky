use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitVariantHirNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantHirNodePath,
    pub syn_node_decl: UnitTypeVariantHirNodeDecl,
}

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitTypeVariantHirDecl,
}
