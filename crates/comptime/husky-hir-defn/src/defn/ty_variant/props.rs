use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct PropsVariantHirNodeDefn {
    #[id]
    pub syn_node_path: TypeVariantHirNodePath,
    pub syn_node_decl: PropsTypeVariantHirNodeDecl,
}

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct PropsVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsTypeVariantHirDecl,
}
