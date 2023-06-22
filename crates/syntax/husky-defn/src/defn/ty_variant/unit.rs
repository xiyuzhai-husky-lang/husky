use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: UnitVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitVariantDecl,
}
