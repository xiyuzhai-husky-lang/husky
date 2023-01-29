use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: UnitVariantDecl,
}
