use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: PropsVariantDecl,
}
