use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: UnitVariantDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_variant_defn(_db: &dyn DefnDb, _decl: UnitVariantDecl) -> UnitVariantDefn {
    todo!()
}
