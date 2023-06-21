use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: PropsVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsVariantDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn props_variant_defn(_db: &dyn DefnDb, _decl: PropsVariantDecl) -> PropsVariantDefn {
    todo!()
}
