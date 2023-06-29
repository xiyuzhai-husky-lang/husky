use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: UnitTypeVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitTypeVariantDecl,
}
